import { expect, it, vi } from 'vitest';
import { render } from 'svelte/server';
import { I18N_CONTEXT, I18n } from './index.js';
import type { Locale } from './locale.js';
import Contact from '../../routes/contact/+page.svelte';
import { websiteLd } from '#lib/seo/structured-data.js';
import { athleteTitle, competitionTitle } from '#lib/seo/names.js';

vi.mock('$app/env/public', () => ({
  PUBLIC_SITE_URL: 'https://openstreetlifting.org',
  PUBLIC_ENVIRONMENT: '',
}));
vi.mock('$app/state', () => ({ page: { url: new URL('https://openstreetlifting.org/contact') } }));

it('renders French content and metadata on the server without changing the canonical URL', () => {
  const context = new Map([[I18N_CONTEXT, new I18n(() => 'fr')]]);
  const { body, head } = render(Contact, { context });
  expect(body).toContain('Nous contacter');
  expect(body).toContain('Posez une question');
  expect(head).toContain('content="fr_FR"');
  expect(head).toContain('signaler une erreur');
  expect(head).toContain('href="https://openstreetlifting.org/contact"');
  expect(websiteLd('fr')).toMatchObject({
    inLanguage: 'fr',
    description: expect.stringContaining('projet ouvert'),
  });
  expect(athleteTitle('Alex Martin', 'DE', 'fr')).toContain('Allemagne - Résultats');
  expect(competitionTitle('French Open 2026', true, 'fr')).toContain('résultats');
});

it('does not share translation state between simultaneous server renders', async () => {
  const locales: Locale[] = ['en', 'fr', 'en', 'fr'];
  const pages = await Promise.all(
    locales.map(async (locale) =>
      render(Contact, { context: new Map([[I18N_CONTEXT, new I18n(() => locale)]]) })
    )
  );
  expect(pages.map(({ body }) => body.includes('Nous contacter'))).toEqual([
    false,
    true,
    false,
    true,
  ]);
});
