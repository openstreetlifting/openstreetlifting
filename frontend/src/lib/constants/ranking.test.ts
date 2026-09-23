import { expect, it } from 'vitest';
import { defaultRankingSort, risAvailability } from './ranking';

const all4 = ['M', 'P', 'D', 'S'];

const meet = (movements: string[], scores: (string | null)[]) => ({
  movements,
  categories: [{ participants: scores.map((ris_score) => ({ ris_score })) }],
});

it('reports RIS as available when a participant carries a score', () => {
  expect(risAvailability(meet(all4, ['312.45', null]))).toBe('available');
});

it('separates a shorter event from a four lift meet with no published bodyweight', () => {
  expect(risAvailability(meet(['P', 'D'], [null, null]))).toBe('not-contested');
  expect(risAvailability(meet(all4, [null, null]))).toBe('not-published');
});

it('treats a missing ris_score field as no score', () => {
  expect(risAvailability({ movements: all4, categories: [{ participants: [{}] }] })).toBe(
    'not-published'
  );
});

it('sorts on total whenever no athlete has a RIS to sort on', () => {
  expect(defaultRankingSort('available')).toBe('ris');
  expect(defaultRankingSort('not-published')).toBe('total');
  expect(defaultRankingSort('not-contested')).toBe('total');
});
