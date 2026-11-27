import { expect, it, vi } from 'vitest';
import { render } from 'vitest-browser-svelte';
import RisHeader from './ris-header.svelte';
import { I18N_CONTEXT, I18n } from '#lib/i18n/index.js';

vi.mock('$app/paths', () => ({ resolve: (path: string) => `/${path}` }));
vi.mock('$app/state', () => ({ page: { data: { risEdition: 2026 } } }));

const context = new Map([[I18N_CONTEXT, new I18n(() => 'en')]]);

it('names the RIS edition the scores were computed with', async () => {
  await render(RisHeader, { context });
  expect(document.body.textContent).toContain('RIS 2026');
  expect(document.body.textContent).toContain('We recalculate RIS with the 2026 formula');
});

it('keeps a plain RIS label when every score is published', async () => {
  await render(RisHeader, { context, props: { published: true } });
  expect(document.body.textContent).not.toContain('RIS 2026');
  expect(document.body.textContent).toContain('We recalculate RIS with the current formula');
});
