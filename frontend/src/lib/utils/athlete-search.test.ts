import { describe, expect, it } from 'vitest';
import type { AthleteSearchHit } from '#lib/types/athlete.js';
import { notInRanking, rankingLabels } from './athlete-search.js';

const hit = (slug: string, rankings: AthleteSearchHit['rankings']): AthleteSearchHit => ({
  slug,
  first_name: 'A',
  last_name: slug,
  gender: 'M',
  country: 'FR',
  rankings,
  best_entry: null,
});

describe('rankingLabels', () => {
  it('names Total and RIS for a full MPDS athlete', () => {
    expect(
      rankingLabels(hit('a', ['ris', 'total', 'muscleup', 'pullup', 'dips', 'squat']))
    ).toEqual(['Total', 'RIS']);
  });

  it('lists single-lift rankings in lift order', () => {
    expect(rankingLabels(hit('a', ['dips', 'pullup']))).toEqual(['Pull Up', 'Dips']);
  });

  it('is empty without a valid lift', () => {
    expect(rankingLabels(hit('a', []))).toEqual([]);
  });
});

describe('notInRanking', () => {
  it('drops athletes already in the ranking', () => {
    const hits = [hit('a', ['total']), hit('b', ['pullup']), hit('c', [])];
    expect(notInRanking(hits, ['a']).map((h) => h.slug)).toEqual(['b', 'c']);
  });
});
