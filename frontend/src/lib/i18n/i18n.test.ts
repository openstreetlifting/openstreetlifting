import { describe, expect, it, vi, afterEach } from 'vitest';
import { I18n } from './index.js';
import { negotiateLocale } from './locale.js';
import { messages } from './en.js';
import french from './fr.js';
import german from './de.js';
import italian from './it.js';
import polish from './pl.js';
import {
  countryName,
  formatCountdown,
  formatDate,
  formatLongDate,
  formatScore,
  formatWeight,
} from '#lib/utils/format.js';

const en = new I18n(() => 'en');
const fr = new I18n(() => 'fr');
afterEach(() => vi.useRealTimers());

describe('language negotiation', () => {
  it.each([
    ['fr-CA,fr;q=0.9,en;q=0.8', 'fr'],
    ['de,fr;q=0.8,en;q=0.9', 'de'],
    ['it-IT,it;q=0.9', 'it'],
    ['pl-PL,pl;q=0.9', 'pl'],
    ['fr;q=0,en;q=0.8', 'en'],
    ['fr;q=bad,en;q=0.8', 'en'],
    ['fr;q=2,en;q=0.8', 'en'],
    ['es,*;q=0.5', 'en'],
    [null, 'en'],
  ])('chooses %s as %s', (header, expected) => {
    expect(negotiateLocale(header)).toBe(expected);
  });
});

it('keeps independent locales and interpolates values without interpreting their content', () => {
  expect(
    fr.t('Attempt {slot}: {weight} {unit}, {result}', {
      unit: 'kg',
      slot: 1,
      weight: '72,5',
      result: fr.t('no lift'),
    })
  ).toBe('Essai 1 : 72,5 kg, non validé');
  expect(en.t('Pull-up')).toBe('Pull-up');
  expect(fr.movement('Pull Up')).toBe('Traction');
  expect(fr.t('{athlete} on Instagram', { athlete: '<script>{score}</script>' })).toBe(
    '<script>{score}</script> sur Instagram'
  );
  expect(en.count('athlete', 0)).toBe('athletes');
  expect(fr.count('athlete', 0)).toBe('athlète');
  expect(fr.count('competition', 2)).toBe('compétitions');
});

it.each(Object.entries({ fr: french, de: german, it: italian, pl: polish }))(
  'translates every %s message and preserves its interpolation placeholders',
  (_locale, catalog) => {
    const placeholders = (text: string) =>
      [...text.matchAll(/\{([^{}]+)\}/g)].map((match) => match[1]).sort();
    expect(Object.keys(catalog).sort()).toEqual([...messages].sort());
    for (const message of messages) {
      expect(catalog[message].trim(), message).not.toBe('');
      expect(placeholders(catalog[message]), message).toEqual(placeholders(message));
    }
    const removalMessage =
      'The procedure replaces your name with a numbered label, such as “Redacted Athlete #7”, and removes your original-script name and Instagram link. Once published, the change appears on the website, API, and fresh downloads, and the old named profile is removed.';
    expect(catalog[removalMessage]).toContain('Redacted Athlete #7');
  }
);

it('uses Polish number forms for one, few and many', () => {
  const pl = new I18n(() => 'pl');
  expect(pl.count('country', 1)).toBe('kraj');
  expect(pl.count('country', 2)).toBe('kraje');
  expect(pl.count('country', 5)).toBe('krajów');
  expect(pl.count('country', 22)).toBe('kraje');
  expect(pl.count('country', 12)).toBe('krajów');
  expect(pl.count('country', 1.5)).toBe('kraju');
  expect(pl.count('athlete', 1)).toBe('zawodnik');
  expect(pl.count('athlete', 2)).toBe('zawodników');
  expect(pl.count('athlete', 5)).toBe('zawodników');
});

it('formats calendar dates in UTC and localizes countries, decimals and countdowns', () => {
  vi.useFakeTimers();
  vi.setSystemTime(new Date('2026-10-27T00:00:00Z'));
  expect(formatDate('2026-01-01', 'fr')).toBe('01/01/2026');
  expect(formatLongDate('2026-01-01', 'fr')).toBe('1 janvier 2026');
  expect(formatLongDate('invalid', 'fr')).toBe('');
  expect(formatDate(null, 'fr')).toBe('-');
  expect(formatCountdown('2026-10-28', 'fr')).toBe('demain');
  expect(countryName('DE', 'fr')).toBe('Allemagne');
  expect(formatWeight(72.5, 'fr')).toBe('72,5');
  expect(formatWeight(0, 'fr')).toBe('0');
  expect(formatWeight(null, 'fr')).toBe('-');
  expect(formatScore('100.3', 'fr')).toBe('100,30');
  expect(formatScore(0, 'fr')).toBe('0,00');
});

it.each([
  ['en', 'October 27, 2026'],
  ['fr', '27 octobre 2026'],
  ['de', '27. Oktober 2026'],
  ['it', '27 ottobre 2026'],
  ['pl', '27 października 2026'],
] as const)('formats a competition date in %s', (locale, expected) => {
  expect(formatLongDate('2026-10-27', locale)).toBe(expected);
});
