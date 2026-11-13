import { expect, it } from 'vitest';
import { page } from 'vitest/browser';
import { render } from 'vitest-browser-svelte';
import RecordMark from './record-mark.svelte';

it('labels a world record', async () => {
  render(RecordMark, { mark: 'WR' });
  const mark = page.getByText('WR', { exact: true });
  await expect.element(mark).toBeVisible();
  await expect.element(mark).toHaveAttribute('title', 'Current world record in this weight class');
});

it('labels a national record', async () => {
  render(RecordMark, { mark: 'NR' });
  await expect
    .element(page.getByLabelText('Current national record in this weight class'))
    .toHaveTextContent('NR');
});

it('renders nothing without a record', async () => {
  const { container } = await render(RecordMark, { mark: null });
  expect(container.textContent).toBe('');
});

it('hides a national record where only world records are shown', async () => {
  const { container } = await render(RecordMark, { mark: 'NR', worldOnly: true });
  expect(container.textContent).toBe('');
});

it('keeps a world record where only world records are shown', async () => {
  render(RecordMark, { mark: 'WR', worldOnly: true });
  await expect.element(page.getByText('WR', { exact: true })).toBeVisible();
});

it('draws a round badge for personal-best cards', async () => {
  render(RecordMark, { mark: 'WR', badge: true });
  await expect.element(page.getByText('WR', { exact: true })).toHaveClass(/rounded-full/);
});
