import { expect, it } from 'vitest';
import { countryRedirect, localeHref, pathLocale, stripLocale, legacyRedirect } from './paths.js';
import { reroute } from '../../hooks.js';

it('switches languages once while preserving filters, escaped slugs and fragments', () => {
  expect(localeHref('/fr/competitions/open%20cup?q=alex&page=2#results', 'pl')).toBe(
    '/pl/competitions/open%20cup?q=alex&page=2#results'
  );
  expect(localeHref('/de', 'it')).toBe('/it');
  expect(localeHref('/en?q=alex', 'fr')).toBe('/fr?q=alex');
  expect(localeHref('/federations/fpcf', 'fr')).toBe('/fr/federations/fpcf');
  expect(localeHref('https://example.com/fr/contact', 'de')).toBe('https://example.com/fr/contact');
  expect(pathLocale('/france')).toBeUndefined();
  expect(stripLocale('/pl/contact')).toBe('/contact');
  expect(legacyRedirect('/sitemap.xml', '')).toBeUndefined();
});

it('sends retired country pages to the rankings filtered by that country', () => {
  expect(countryRedirect('/fr/countries/it')).toBe('/fr?country=IT');
  expect(countryRedirect('/countries/fr/')).toBe('/en?country=FR');
  expect(countryRedirect('/pl/countries')).toBe('/pl');
  expect(countryRedirect('/countries')).toBe('/en');
  expect(countryRedirect('/de/countries/unknown')).toBe('/de');
  expect(countryRedirect('/xx/countries/fr')).toBeUndefined();
  expect(countryRedirect('/de/federations/fpcf')).toBeUndefined();
});

it('routes localized pages while keeping assets, API and machine-readable endpoints at their original URLs', () => {
  const route = (path: string) =>
    reroute({ url: new URL(`https://openstreetlifting.org${path}`), fetch });
  expect(route('/de/athletes/alex-martin')).toBe('/athletes/alex-martin');
  expect(route('/pl')).toBe('/');
  expect(route('/it/manifest.webmanifest')).toBe('/manifest.webmanifest');
  expect(route('/fr/api/athletes/search')).toBeUndefined();
  expect(route('/sitemap.xml')).toBeUndefined();
  expect(route('/flags/fr.svg')).toBeUndefined();
});
