import { expect, it, vi } from 'vitest';
import { render } from 'vitest-browser-svelte';
import { page } from 'vitest/browser';
import ResultState from './result-state.svelte';

it('prevents duplicate retries while a request is pending and enables retry afterward', async () => {
  let finish!: () => void;
  const retry = vi.fn(() => new Promise<void>((resolve) => (finish = resolve)));
  render(ResultState, { title: 'Failed to load rankings', kind: 'error', retry });
  await page.getByRole('button', { name: 'Try again', exact: true }).click();
  await expect.element(page.getByRole('button', { name: 'Retrying…' })).toBeDisabled();
  expect(retry).toHaveBeenCalledTimes(1);
  finish();
  await expect.element(page.getByRole('button', { name: 'Try again', exact: true })).toBeEnabled();
});

it('keeps a failed retry recoverable without losing the error message', async () => {
  const retry = vi.fn().mockRejectedValue(new Error('Network unavailable'));
  render(ResultState, { title: 'Failed to load rankings', kind: 'error', retry });
  await page.getByRole('button', { name: 'Try again', exact: true }).click();
  await expect.element(page.getByRole('alert')).toHaveTextContent('Failed to load rankings');
  await expect.element(page.getByRole('button', { name: 'Try again', exact: true })).toBeEnabled();
});
