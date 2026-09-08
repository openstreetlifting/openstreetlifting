import { expect, it } from 'vitest';
import { athleteFilters } from './athlete-filters';

it('restores independent ranking and performance metrics and a valid event', () => {
  expect(
    athleteFilters(new URLSearchParams('ranking=dips&performance=squat&event=PD'), ['MPDS', 'PD'])
  ).toEqual({ ranking: 'dips', performance: 'squat', event: 'PD' });
});

it('falls back for absent or invalid query values', () => {
  expect(
    athleteFilters(new URLSearchParams('ranking=invalid&performance=invalid&event=invalid'), ['PD'])
  ).toEqual({ ranking: 'ris', performance: 'total', event: 'PD' });
  expect(athleteFilters(new URLSearchParams(), [])).toEqual({
    ranking: 'ris',
    performance: 'total',
    event: 'MPDS',
  });
});

it('falls back to Total for a previously shared All view', () => {
  expect(athleteFilters(new URLSearchParams('performance=all&ranking=all'), [])).toMatchObject({
    performance: 'total',
    ranking: 'ris',
  });
});
