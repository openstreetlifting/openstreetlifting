import { describe, expect, it, vi, afterEach } from 'vitest';
import { I18n } from './index.js';
import { negotiateLocale } from './locale.js';
import { messages } from './en.js';
import french from './fr.js';
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
    ['en', 'fr-FR,fr;q=0.9', 'en'],
    ['fr', 'en-US,en;q=0.9', 'fr'],
    [undefined, 'fr-CA,fr;q=0.9,en;q=0.8', 'fr'],
    [undefined, 'de,fr;q=0.8,en;q=0.9', 'en'],
    ['invalid', 'fr-FR', 'fr'],
    [undefined, 'fr;q=0,en;q=0.8', 'en'],
    [undefined, 'fr;q=bad,en;q=0.8', 'en'],
    [undefined, 'fr;q=2,en;q=0.8', 'en'],
    [undefined, 'de,*;q=0.5', 'en'],
    [undefined, null, 'en'],
  ])('chooses %s / %s as %s', (cookie, header, expected) => {
    expect(negotiateLocale(cookie, header)).toBe(expected);
  });
});

it('keeps independent locales and interpolates values without interpreting their content', () => {
  expect(
    fr.t('Attempt {slot}: {weight} kg, {result}', {
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

it('translates every message and preserves its interpolation placeholders', () => {
  const placeholders = (text: string) =>
    [...text.matchAll(/\{(\w+)\}/g)].map((match) => match[1]).sort();
  expect(Object.keys(french).sort()).toEqual([...messages].sort());
  for (const message of messages) {
    expect(french[message].trim(), message).not.toBe('');
    expect(placeholders(french[message]), message).toEqual(placeholders(message));
  }
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
