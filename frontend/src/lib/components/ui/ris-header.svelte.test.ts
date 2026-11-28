import { expect, it, vi } from 'vitest';
import { render } from 'vitest-browser-svelte';
import RisHeader from './ris-header.svelte';

vi.mock('$app/state', () => ({ page: { data: { risEdition: 2026 } } }));

it('names the RIS edition the scores were computed with', async () => {
  await render(RisHeader);
  expect(document.body.textContent).toContain('RIS 2026');
});

it('keeps a plain RIS label when every score is published', async () => {
  await render(RisHeader, { props: { published: true } });
  expect(document.body.textContent).not.toContain('RIS 2026');
  expect(document.body.textContent).toContain('RIS');
});
