import { expect, it } from 'vitest';
import { COMPETITION_FILTERS, RANKING_FILTERS, RECORD_FILTERS } from '#lib/constants/filters.js';
import { keepSelected, readFilters } from './filters';

const params = (query: string) => new URLSearchParams(query);

it('reads each filter with its own parser and leaves the rest unset', () => {
  expect(
    readFilters(params('gender=F&year=2025&country=%20FR%20&page=3'), RANKING_FILTERS)
  ).toEqual({
    gender: 'F',
    category: null,
    year: 2025,
    country: 'FR',
    federation: null,
    series: null,
  });
  expect(readFilters(params('gender=X&year=soon'), RANKING_FILTERS)).toMatchObject({
    gender: null,
    year: null,
  });
  expect(readFilters(params('event=DP'), COMPETITION_FILTERS).event).toBe('PD');
  expect(readFilters(params('country=fr'), RECORD_FILTERS).country).toBe('FR');
});

it('adds a selected value its options no longer offer', () => {
  const facets = { years: [2026, 2025], countries: ['FR'] };
  expect(keepSelected(facets, { years: 2019, countries: 'FR' })).toEqual({
    years: [2026, 2025, 2019],
    countries: ['FR'],
  });
  expect(keepSelected(facets, { years: null })).toEqual(facets);
});
