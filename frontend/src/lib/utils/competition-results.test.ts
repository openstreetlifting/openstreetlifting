import { expect, it } from 'vitest';
import type { Participant } from '$lib/types/competition';
import { hasRankingResult } from './competition-results';

const result = (overrides: Partial<Participant> = {}): Participant => ({
  participant_id: 'entry-1',
  athlete: {
    athlete_id: '1',
    first_name: 'Alex',
    last_name: 'Martin',
    slug: 'alex-martin',
    country: 'FR',
    gender: 'M',
  },
  bodyweight: null,
  rank: 1,
  total: '400',
  ris_score: null,
  ris_source: null,
  status: 'competed',
  status_reason: null,
  lifts: [],
  ...overrides,
});

it('keeps total-only athletes without RIS among the unranked competition entries', () => {
  const athlete = result();
  expect(hasRankingResult(athlete, 'ris')).toBe(false);
  expect(hasRankingResult(athlete, 'total')).toBe(true);
  expect(hasRankingResult(athlete, 'squat')).toBe(false);
});

it('does not duplicate total-only athletes who already have a ranked score', () => {
  expect(hasRankingResult(result({ ris_score: '80', ris_source: 'computed' }), 'ris')).toBe(true);
});

it('treats zero as a recorded result and distinguishes it from an unknown or failed lift', () => {
  const athlete = result({
    total: '0',
    ris_score: '0',
    lifts: [
      { movement_name: 'Muscle-up', best_weight: '0', attempts: [] },
      { movement_name: 'Pull-up', best_weight: null, attempts: [] },
    ],
  });
  expect(hasRankingResult(athlete, 'muscleup')).toBe(true);
  expect(hasRankingResult(athlete, 'ris')).toBe(true);
  expect(hasRankingResult(athlete, 'total')).toBe(true);
  expect(hasRankingResult(athlete, 'pullup')).toBe(false);
  expect(hasRankingResult(athlete, 'dips')).toBe(false);
});

it('keeps disqualifications, no-shows and missing scores out of the ranked rows', () => {
  expect(hasRankingResult(result({ status: 'disqualified' }), 'total')).toBe(false);
  expect(hasRankingResult(result({ status: 'no_show', total: null }), 'total')).toBe(false);
  expect(hasRankingResult(result({ total: null, ris_score: null }), 'ris')).toBe(false);
});

it('keeps a published RIS without lifts or total in one ranked row', () => {
  expect(
    hasRankingResult(
      result({ total: null, lifts: [], ris_score: '72.3', ris_source: 'reported' }),
      'ris'
    )
  ).toBe(true);
});
