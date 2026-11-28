import { expect, it } from 'vitest';
import type { AthleteCompetitionSummary } from '#lib/types/athlete.js';
import { liftProgress, showsProgress } from './athlete-progress';

type Lifts = Record<string, string | null>;

function result(
  id: string,
  date: string,
  lifts: Lifts,
  overrides: Partial<AthleteCompetitionSummary> = {}
): AthleteCompetitionSummary {
  return {
    competition_id: id,
    division: null,
    competition_name: id,
    competition_slug: id,
    competition_date: date,
    category_name: 'Men -80kg',
    rank: 1,
    total: null,
    total_record: null,
    ris_score: null,
    ris_source: null,
    status: 'competed',
    event: 'MPDS',
    lifts: Object.entries(lifts).map(([movement_name, best_weight]) => ({
      movement_name: movement_name as AthleteCompetitionSummary['lifts'][number]['movement_name'],
      best_weight,
      record: null,
      attempts: [],
    })),
    ...overrides,
  };
}

const values = (competitions: AthleteCompetitionSummary[]) =>
  Object.fromEntries(
    liftProgress(competitions).map(({ lift, points }) => [
      lift.key,
      points.map((point) => [point.competition.competition_id, point.value]),
    ])
  );

it('draws each lift from every format, skipping meets where it has no result', () => {
  const competitions = [
    result('pd-meet', '2025-06-01', { 'Pull-up': '40', Dips: '60' }, { event: 'PD' }),
    result('mpds-meet', '2024-01-01', { 'Muscle-up': '20', 'Pull-up': '30', Dips: null }),
    result('bombed', '2025-01-01', { 'Pull-up': null, Dips: '50' }),
  ];

  expect(values(competitions)).toEqual({
    muscleup: [['mpds-meet', 20]],
    pullup: [
      ['mpds-meet', 30],
      ['pd-meet', 40],
    ],
    dips: [
      ['bombed', 50],
      ['pd-meet', 60],
    ],
  });
});

it('keeps zero, the best entry per meet, and only results the athlete competed in', () => {
  const competitions = [
    result('meet', '2025-01-01', { 'Pull-up': '30' }, { division: 'Open' }),
    result('meet', '2025-01-01', { 'Pull-up': '35' }, { division: 'Masters' }),
    result('zero', '2025-02-01', { 'Pull-up': '0' }),
    result('dq', '2025-03-01', { 'Pull-up': '50' }, { status: 'disqualified' }),
    result('undated', 'invalid', { 'Pull-up': '50' }),
  ];

  expect(values(competitions)).toEqual({
    pullup: [
      ['meet', 35],
      ['zero', 0],
    ],
  });
});

it('shows the chart only when two lifts each have two meets', () => {
  const first = result('first', '2024-01-01', { 'Pull-up': '30', Dips: '50' });
  const pullOnly = result('second', '2025-01-01', { 'Pull-up': '35' });
  const both = result('second', '2025-01-01', { 'Pull-up': '35', Dips: '55' });

  expect(showsProgress(liftProgress([first]))).toBe(false);
  expect(showsProgress(liftProgress([first, pullOnly]))).toBe(false);
  expect(showsProgress(liftProgress([first, both]))).toBe(true);
});
