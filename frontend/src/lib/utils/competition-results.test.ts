import { expect, it } from 'vitest';
import type { CategoryDetail, Participant } from '#lib/types/competition.js';
import {
  competitionDivisions,
  contestSummary,
  hasRankingResult,
  sharedRankBy,
  weightClasses,
} from './competition-results';

it('lists classes once per sex in weight order across divisions using the published labels', () => {
  const categories: CategoryDetail[] = (
    [
      { division: 'Junior', gender: 'M', weight_class: '-80kg' },
      { division: 'Open', gender: 'M', weight_class: '+101kg' },
      { division: 'Open', gender: 'M', weight_class: '-66kg' },
      { division: 'Open', gender: 'M', weight_class: '-80kg' },
      { division: 'Open', gender: 'M', weight_class: '-101kg' },
      { division: 'Open', gender: 'M', weight_class: '' },
      { division: 'Open', gender: 'F', weight_class: '-52.5kg' },
      { division: 'Junior', gender: 'F', weight_class: '-52kg' },
    ] as const
  ).map((category) => ({
    category: { ...category, name: category.weight_class, rank_by: 'total' as const },
    participants: [],
  }));

  expect(weightClasses(categories, 'M')).toEqual(['-66kg', '-80kg', '-101kg', '', '+101kg']);
  expect(weightClasses(categories, 'F')).toEqual(['-52kg', '-52.5kg']);
  expect(weightClasses(categories, 'MX')).toEqual([]);
});

it('opens the largest division first and keeps how each division is ranked', () => {
  const contest = (division: string | null, rankBy: 'total' | 'ris', entries: number) => ({
    category: { name: '', division, gender: 'M' as const, weight_class: '', rank_by: rankBy },
    participants: Array.from({ length: entries }, () => result()),
  });

  expect(
    competitionDivisions([
      contest('FNSL Elite', 'ris', 8),
      contest('Open', 'total', 30),
      contest('Open', 'total', 19),
      contest('FNSL Masters', 'ris', 3),
    ])
  ).toEqual([
    { name: 'Open', rankBy: 'total', entries: 49 },
    { name: 'FNSL Elite', rankBy: 'ris', entries: 8 },
    { name: 'FNSL Masters', rankBy: 'ris', entries: 3 },
  ]);
  expect(competitionDivisions([contest(null, 'total', 5)])).toEqual([]);
});

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
  age: null,
  rank: 1,
  total: '400',
  total_record: null,
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
      { movement_name: 'Muscle-up', best_weight: '0', record: null, attempts: [] },
      { movement_name: 'Pull-up', best_weight: null, record: null, attempts: [] },
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

const contest = (
  division: string | null,
  gender: 'M' | 'F' | 'MX',
  weight_class: string,
  rank_by: 'total' | 'ris',
  entries = 1
): CategoryDetail => ({
  category: { name: '', division, gender, weight_class, rank_by },
  participants: Array.from({ length: entries }, () => result()),
});

it('describes each division by its contests and how they were ranked', () => {
  const summary = contestSummary([
    contest('Open', 'M', '-66kg', 'total', 9),
    contest('Open', 'M', '+87kg', 'total', 5),
    contest('Open', 'F', '', 'total', 6),
    contest('FNSL Elite', 'MX', '', 'ris', 8),
    contest('FNSL Masters', 'M', '', 'ris', 3),
  ]);

  expect(summary).toEqual([
    {
      name: 'Open',
      groups: [
        { gender: 'M', classes: ['-66kg', '+87kg'], allWeights: false, rankBy: 'total' },
        { gender: 'F', classes: [], allWeights: true, rankBy: 'total' },
      ],
      classed: true,
      rankBy: 'total',
    },
    {
      name: 'FNSL Elite',
      groups: [{ gender: 'MX', classes: [], allWeights: true, rankBy: 'ris' }],
      classed: false,
      rankBy: 'ris',
    },
    {
      name: 'FNSL Masters',
      groups: [{ gender: 'M', classes: [], allWeights: true, rankBy: 'ris' }],
      classed: false,
      rankBy: 'ris',
    },
  ]);
  expect(sharedRankBy(summary)).toBeNull();
});

it('summarizes a meet without divisions as one contest list with its shared rule', () => {
  const summary = contestSummary([
    contest(null, 'M', '-80kg', 'total'),
    contest(null, 'F', '-63kg', 'total'),
  ]);

  expect(summary).toHaveLength(1);
  expect(summary[0]).toMatchObject({ name: null, classed: true, rankBy: 'total' });
  expect(sharedRankBy(summary)).toBe('total');
});

it('keeps a rule per sex when one division ranked its sexes differently', () => {
  const [division] = contestSummary([
    contest(null, 'M', '-80kg', 'total'),
    contest(null, 'F', '', 'ris'),
  ]);

  expect(division.rankBy).toBeNull();
  expect(division.groups.map(({ gender, rankBy }) => [gender, rankBy])).toEqual([
    ['M', 'total'],
    ['F', 'ris'],
  ]);
});
