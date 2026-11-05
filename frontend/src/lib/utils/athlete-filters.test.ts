import { expect, it } from 'vitest';
import { athleteFilters } from './athlete-filters';

it('restores independent ranking and performance metrics while ignoring legacy event filters', () => {
  expect(athleteFilters(new URLSearchParams('ranking=dips&performance=squat&event=PD'))).toEqual({
    ranking: 'dips',
    performance: 'squat',
  });
});

it('falls back for absent or invalid query values', () => {
  expect(
    athleteFilters(new URLSearchParams('ranking=invalid&performance=invalid&event=invalid'))
  ).toEqual({ ranking: 'ris', performance: 'total' });
  expect(athleteFilters(new URLSearchParams())).toEqual({ ranking: 'ris', performance: 'total' });
});

it('falls back to Total for a previously shared All view', () => {
  expect(athleteFilters(new URLSearchParams('performance=all&ranking=all'))).toMatchObject({
    performance: 'total',
    ranking: 'ris',
  });
});
