import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { render } from 'vitest-browser-svelte';
import { flushSync } from 'svelte';
import SearchInput from './search-input.svelte';

const navigation = vi.hoisted(() => ({ cancel: () => {} }));
vi.mock('$app/navigation', () => ({
  beforeNavigate: (callback: () => void) => (navigation.cancel = callback),
}));

beforeEach(() => vi.useFakeTimers());
afterEach(() => vi.useRealTimers());

async function setup() {
  const onSearch = vi.fn();
  const screen = await render(SearchInput, { value: '', onSearch });
  const input = screen.getByRole('textbox').element() as HTMLInputElement;
  const type = (value: string) => {
    input.value = value;
    input.dispatchEvent(new Event('input', { bubbles: true }));
    flushSync();
  };
  return { screen, input, type, onSearch };
}

it('updates typing immediately and searches only after the latest pause', async () => {
  const { input, type, onSearch } = await setup();
  type('mar');
  await vi.advanceTimersByTimeAsync(200);
  type('martin');
  expect(input.value).toBe('martin');
  await vi.advanceTimersByTimeAsync(299);
  expect(onSearch).not.toHaveBeenCalled();
  await vi.advanceTimersByTimeAsync(1);
  expect(onSearch).toHaveBeenCalledTimes(1);
});

it('flushes Enter and clear immediately without another delayed request', async () => {
  const { screen, input, type, onSearch } = await setup();
  type('mar');
  input.dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', bubbles: true }));
  expect(onSearch).toHaveBeenCalledTimes(1);
  await vi.advanceTimersByTimeAsync(300);
  expect(onSearch).toHaveBeenCalledTimes(1);
  type('martin');
  (screen.getByRole('button', { name: 'Clear search' }).element() as HTMLButtonElement).click();
  flushSync();
  expect(input.value).toBe('');
  expect(onSearch).toHaveBeenCalledTimes(2);
  await vi.advanceTimersByTimeAsync(300);
  expect(onSearch).toHaveBeenCalledTimes(2);
});

it('cancels a pending search when navigating away or restoring history', async () => {
  const { type, onSearch } = await setup();
  type('martin');
  navigation.cancel();
  await vi.advanceTimersByTimeAsync(300);
  expect(onSearch).not.toHaveBeenCalled();
});

it('cancels a pending search when the input is removed', async () => {
  const { screen, type, onSearch } = await setup();
  type('martin');
  await screen.unmount();
  await vi.advanceTimersByTimeAsync(300);
  expect(onSearch).not.toHaveBeenCalled();
});

it('waits until IME composition finishes before searching', async () => {
  const { input, type, onSearch } = await setup();
  input.dispatchEvent(new CompositionEvent('compositionstart', { bubbles: true }));
  type('王');
  input.dispatchEvent(
    new KeyboardEvent('keydown', { key: 'Enter', isComposing: true, bubbles: true })
  );
  await vi.advanceTimersByTimeAsync(300);
  expect(onSearch).not.toHaveBeenCalled();
  input.dispatchEvent(new CompositionEvent('compositionend', { bubbles: true }));
  await vi.advanceTimersByTimeAsync(300);
  expect(onSearch).toHaveBeenCalledTimes(1);
});
