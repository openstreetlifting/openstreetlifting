import { expect, it } from 'vitest';
import { defaultRankingSort, risProvenance } from './ranking';

const all4 = ['M', 'P', 'D', 'S'];

const meet = (
  movements: string[],
  participants: { ris_score?: string; ris_source?: string }[]
) => ({
  movements,
  categories: [{ participants }],
});

it('reads a meet as recomputed when any score was computed from a bodyweight', () => {
  expect(risProvenance(meet(all4, [{ ris_score: '312.45', ris_source: 'computed' }]))).toBe(
    'recomputed'
  );
});

it('reads a meet as reported only when no score in it could be recomputed', () => {
  expect(risProvenance(meet(all4, [{ ris_score: '300', ris_source: 'reported' }]))).toBe(
    'reported'
  );
  expect(
    risProvenance(
      meet(all4, [
        { ris_score: '300', ris_source: 'reported' },
        { ris_score: '312.45', ris_source: 'computed' },
      ])
    )
  ).toBe('recomputed');
});

it('separates a shorter event from a four lift meet with no published score', () => {
  expect(risProvenance(meet(['P', 'D'], [{}]))).toBe('not-contested');
  expect(risProvenance(meet(all4, [{}, {}]))).toBe('not-published');
});

it('sorts on total whenever no athlete has a RIS to sort on', () => {
  expect(defaultRankingSort('recomputed')).toBe('ris');
  expect(defaultRankingSort('reported')).toBe('ris');
  expect(defaultRankingSort('not-published')).toBe('total');
  expect(defaultRankingSort('not-contested')).toBe('total');
});
