import { expect, it } from 'vitest';
import { displayWeight, kilograms, weightUnit } from './weight-unit.js';
import { formatWeight } from './format.js';
import { WeightPreference } from '#lib/state/weight-unit.svelte.js';

it('converts with the exact pound definition without changing missing or zero results', () => {
  expect(kilograms(1, 'lb')).toBe(0.45359237);
  expect(displayWeight(1, 'lb')).toBeCloseTo(2.2046226218, 9);
  expect(kilograms(displayWeight(72.5, 'lb'), 'lb')).toBeCloseTo(72.5, 12);
  expect(formatWeight('100', 'en', 'lb')).toBe('220.46');
  expect(formatWeight('-20', 'en', 'lb')).toBe('-44.09');
  expect(formatWeight(0, 'en', 'lb')).toBe('0');
  for (const value of [null, undefined, '']) expect(formatWeight(value, 'en', 'lb')).toBe('-');
  expect(formatWeight('72.50', 'en')).toBe('72.50');
});

it.each(['fr', 'de', 'it', 'pl'] as const)('uses %s decimal formatting for pounds', (locale) => {
  expect(formatWeight(80, locale, 'lb')).toBe('176,37');
});

it('converts class labels while preserving signs, source labels, and independent preferences', () => {
  const pounds = new WeightPreference(() => 'lb');
  const metric = new WeightPreference();
  expect(pounds.category('Elite Men -80kg', 'en')).toBe('Elite Men -176.37 lb');
  expect(pounds.category('+87', 'fr')).toBe('+191,8 lb');
  expect(pounds.category('Junior U23', 'en')).toBe('Junior U23');
  expect(pounds.category('All-4', 'en')).toBe('All-4');
  expect(pounds.category('Open', 'en')).toBe('Open');
  expect(metric.category('-80kg', 'en')).toBe('-80kg');
  pounds.unit = 'kg';
  expect(metric.unit).toBe('kg');
  expect(pounds.format(80, 'en')).toBe('80');
});

it('defaults unsupported preferences to kilograms', () => {
  expect(weightUnit('lb')).toBe('lb');
  for (const value of [undefined, 'kg', 'lbs', 'LB', 'invalid'])
    expect(weightUnit(value)).toBe('kg');
});
