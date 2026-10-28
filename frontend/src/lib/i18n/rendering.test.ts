import { expect, it, vi } from 'vitest';
import { render } from 'svelte/server';
import { I18N_CONTEXT, I18n } from './index.js';
import { LOCALES, OG_LOCALES } from './locale.js';
import Contact from '../../routes/contact/+page.svelte';
import Privacy from '../../routes/privacy/+page.svelte';
import Ris from '../../routes/ris/+page.svelte';
import { websiteLd } from '#lib/seo/structured-data.js';
import { athleteTitle, competitionTitle } from '#lib/seo/names.js';

vi.mock('$app/env/public', () => ({
  PUBLIC_SITE_URL: 'https://openstreetlifting.org',
  PUBLIC_ENVIRONMENT: '',
}));
vi.mock('$app/state', () => ({ page: { url: new URL('https://openstreetlifting.org/contact') } }));

it('renders French content and metadata on the server with a localized canonical URL and reciprocal language alternates', () => {
  const context = new Map([[I18N_CONTEXT, new I18n(() => 'fr')]]);
  const { body, head } = render(Contact, { context });
  expect(body).toContain('Nous contacter');
  expect(body).toContain('Posez une question');
  expect(head).toContain('content="fr_FR"');
  expect(head).toContain('signaler une erreur');
  expect(head).toContain('href="https://openstreetlifting.org/fr/contact"');
  expect(head).toContain('hreflang="pl" href="https://openstreetlifting.org/pl/contact"');
  expect(websiteLd('fr')).toMatchObject({
    inLanguage: 'fr',
    description: expect.stringContaining('projet ouvert'),
  });
  expect(athleteTitle('Alex Martin', 'DE', 'fr')).toContain('Allemagne - Résultats');
  expect(competitionTitle('French Open 2026', true, 'fr')).toContain('résultats');
});

it.each([
  ['en', 'Get in touch', 'privacy notice'],
  ['fr', 'Nous contacter', 'politique de confidentialité'],
  ['de', 'Kontakt aufnehmen', 'Datenschutzerklärung'],
  ['it', 'Contattaci', 'informativa sulla privacy'],
  ['pl', 'Napisz do nas', 'polityce prywatności'],
] as const)(
  'renders the %s contact page with working inline links and metadata',
  (locale, title, privacyLabel) => {
    const context = new Map([[I18N_CONTEXT, new I18n(() => locale)]]);
    const { body, head } = render(Contact, { context });
    const html = body.replace(/<!--[\s\S]*?-->/g, '');
    expect(html).toContain(`>${title}</h1>`);
    expect(html).toContain(`href="/${locale}/privacy"`);
    expect(html).toContain(`${privacyLabel}</a>.`);
    expect(html).toContain('contact@openstreetlifting.org</a>.');
    expect(html).not.toMatch(/\{(?:email|privacyNotice)\}/);
    expect(head).toContain(`content="${OG_LOCALES[locale]}"`);
    expect(head).toContain(`href="https://openstreetlifting.org/${locale}/contact"`);
  }
);

it.each([
  ['en', 'Privacy policy', 'GDPR', 'eng'],
  ['fr', 'Politique de confidentialité', 'RGPD', 'fra'],
  ['de', 'Datenschutzerklärung', 'DSGVO', 'deu'],
  ['it', 'Informativa sulla privacy', 'GDPR', 'ita'],
  ['pl', 'Polityka prywatności', 'RODO', 'pol'],
] as const)(
  'renders the %s privacy page with inline email and localized regulation links',
  (locale, title, regulation, language) => {
    const { body } = render(Privacy, {
      context: new Map([[I18N_CONTEXT, new I18n(() => locale)]]),
    });
    const html = body.replace(/<!--[\s\S]*?-->/g, '');
    expect(html).toContain(`>${title}</h1>`);
    expect(html).toContain(`href="https://eur-lex.europa.eu/eli/reg/2016/679/oj/${language}"`);
    expect(html).toContain(`>${regulation}</a>`);
    expect(html).toContain('href="mailto:contact@openstreetlifting.org"');
    expect(html).not.toMatch(/\{(?:email|gdpr|cnil)\}/);
  }
);

it.each(LOCALES)('renders RIS attribution and the reported-score symbol in %s', (locale) => {
  const { body } = render(Ris, {
    props: {
      data: { locale, weightUnit: 'kg', editions: [], distribution: { men: [], women: [] } },
    },
    context: new Map([[I18N_CONTEXT, new I18n(() => locale)]]),
  });
  expect(body).toContain('href="https://warisradji.com/ris/"');
  expect(body).toContain('Waris Radji');
  expect(body).toContain('Mathieu Ardoin');
  expect(body).not.toMatch(/\{(?:authors|mark)\}/);
});

it('does not share translation state between simultaneous server renders', async () => {
  const locales = [...LOCALES, ...LOCALES];
  const pages = await Promise.all(
    locales.map(async (locale) =>
      render(Contact, { context: new Map([[I18N_CONTEXT, new I18n(() => locale)]]) })
    )
  );
  for (const [index, { body, head }] of pages.entries()) {
    const locale = locales[index];
    expect(body.includes('Nous contacter')).toBe(locale === 'fr');
    expect(head).toContain(`content="${OG_LOCALES[locale]}"`);
  }
});
