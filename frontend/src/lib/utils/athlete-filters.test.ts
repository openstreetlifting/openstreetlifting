import { expect, it } from 'vitest';
import { athleteRanking } from './athlete-filters';

it('reads the ranking metric and falls back to RIS', () => {
  expect(athleteRanking(new URLSearchParams('ranking=dips'))).toBe('dips');
  expect(athleteRanking(new URLSearchParams('ranking=invalid'))).toBe('ris');
  expect(athleteRanking(new URLSearchParams())).toBe('ris');
});
