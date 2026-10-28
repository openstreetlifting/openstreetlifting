import { expect, it } from 'vitest';
import { localeHref, pathLocale, stripLocale, legacyRedirect } from './paths.js';
import { reroute } from '../../hooks.js';

it('switches languages once while preserving filters, escaped slugs and fragments', () => {
  expect(localeHref('/fr/competitions/open%20cup?q=alex&page=2#results', 'pl')).toBe(
    '/pl/competitions/open%20cup?q=alex&page=2#results'
  );
  expect(localeHref('/de', 'it')).toBe('/it');
  expect(localeHref('/en?q=alex', 'fr')).toBe('/fr?q=alex');
  expect(localeHref('/countries/fr', 'fr')).toBe('/fr/countries/fr');
  expect(localeHref('https://example.com/fr/contact', 'de')).toBe('https://example.com/fr/contact');
  expect(pathLocale('/france')).toBeUndefined();
  expect(stripLocale('/pl/contact')).toBe('/contact');
  expect(legacyRedirect('/sitemap.xml', '')).toBeUndefined();
});

it('routes localized pages while keeping assets, API and machine-readable endpoints at their original URLs', () => {
  const route = (path: string) =>
    reroute({ url: new URL(`https://openstreetlifting.org${path}`), fetch });
  expect(route('/de/athletes/alex-martin')).toBe('/athletes/alex-martin');
  expect(route('/pl')).toBe('/');
  expect(route('/it/manifest.webmanifest')).toBe('/manifest.webmanifest');
  expect(route('/fr/api/rankings')).toBeUndefined();
  expect(route('/sitemap.xml')).toBeUndefined();
  expect(route('/flags/fr.svg')).toBeUndefined();
});
