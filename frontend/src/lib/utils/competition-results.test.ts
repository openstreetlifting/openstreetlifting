import { expect, it } from 'vitest';
import type { CategoryDetail, Participant } from '#lib/types/competition.js';
import {
  competitionDivisions,
  attemptSuccess,
  contestSummary,
  matchesAthlete,
  mixedResults,
  rankResults,
  resultTables,
  resultValue,
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

it('reads only scores the athlete actually lifted, keeping zero apart from a missing lift', () => {
  const athlete = result({
    total: '0',
    ris_score: '0',
    lifts: [
      { movement_name: 'Muscle-up', best_weight: '0', record: null, attempts: [] },
      { movement_name: 'Pull-up', best_weight: null, record: null, attempts: [] },
    ],
  });
  expect(resultValue(athlete, 'muscleup')).toBe(0);
  expect(resultValue(athlete, 'ris')).toBe(0);
  expect(resultValue(athlete, 'total')).toBe(0);
  expect(resultValue(athlete, 'pullup')).toBeNull();
  expect(resultValue(athlete, 'dips')).toBeNull();
  expect(resultValue(result({ status: 'disqualified' }), 'total')).toBeNull();
  expect(resultValue(result({ status: 'no_show', total: null }), 'total')).toBeNull();
  expect(
    resultValue(result({ total: null, ris_score: '72.3', ris_source: 'reported' }), 'ris')
  ).toBe(72.3);
});

const lifter = (id: string, overrides: Partial<Participant> = {}) =>
  result({
    participant_id: id,
    athlete: { ...result().athlete, athlete_id: id, last_name: id, slug: id },
    ...overrides,
  });

const ids = (rows: { participant: Participant; rank: number | null }[]) =>
  rows.map(({ participant, rank }) => [participant.athlete.athlete_id, rank]);

it('ranks by the chosen metric, lighter first on equal totals, and leaves the rest unranked', () => {
  const category = {
    name: '',
    division: null,
    gender: 'M',
    weight_class: '-80kg',
    rank_by: 'total',
  } as const;
  const entries = [
    lifter('dq', { status: 'disqualified', total: null }),
    lifter('heavy', { total: '300', bodyweight: '79.5' }),
    lifter('ns', { status: 'no_show', total: null }),
    lifter('bombed', { total: null }),
    lifter('light', { total: '300', bodyweight: '72' }),
    lifter('top', { total: '320' }),
  ].map((participant) => ({ participant, category }));

  expect(ids(rankResults(entries, 'total'))).toEqual([
    ['top', 1],
    ['light', 2],
    ['heavy', 3],
    ['bombed', null],
    ['dq', null],
    ['ns', null],
  ]);
});

it('keeps the announced places in the official view, ties included, over any metric', () => {
  const category = {
    name: '',
    division: null,
    gender: 'M',
    weight_class: '-80kg',
    rank_by: 'total',
  } as const;
  const entries = [
    lifter('third', { rank: 3, total: '320' }),
    lifter('dq', { rank: null, status: 'disqualified', total: null }),
    lifter('first', { rank: 1, total: '300' }),
    lifter('tied', { rank: 1, total: '300' }),
  ].map((participant) => ({ participant, category }));

  expect(ids(rankResults(entries, 'total', true))).toEqual([
    ['first', 1],
    ['tied', 1],
    ['third', 3],
    ['dq', null],
  ]);
});

it('ranks unplaced lifters by RIS too, with a missing score at the bottom', () => {
  const category = {
    name: '',
    division: null,
    gender: 'F',
    weight_class: '',
    rank_by: 'ris',
  } as const;
  const entries = [
    lifter('low', { ris_score: '40' }),
    lifter('none', { ris_score: null }),
    lifter('high', { ris_score: '90' }),
  ].map((participant) => ({ participant, category }));

  expect(ids(rankResults(entries, 'ris'))).toEqual([
    ['high', 1],
    ['low', 2],
    ['none', null],
  ]);
});

it('lists one table per contest, women light to heavy before men, each ranked from 1', () => {
  const field = (gender: 'M' | 'F', weight_class: string, ...totals: string[]): CategoryDetail => ({
    category: { name: '', division: null, gender, weight_class, rank_by: 'total' },
    participants: totals.map((total) => lifter(`${gender}${weight_class}${total}`, { total })),
  });
  const tables = resultTables(
    [
      field('M', '+80kg', '300'),
      field('F', '-63kg', '150'),
      field('M', '-80kg', '280', '290'),
      field('F', '-57kg', '140'),
    ],
    'total'
  );

  expect(tables.map(({ category }) => `${category.gender}${category.weight_class}`)).toEqual([
    'F-57kg',
    'F-63kg',
    'M-80kg',
    'M+80kg',
  ]);
  expect(ids(tables[2].rows)).toEqual([
    ['M-80kg290', 1],
    ['M-80kg280', 2],
  ]);
});

it('ranks every contest together on one RIS board', () => {
  const field = (gender: 'M' | 'F', ris: string): CategoryDetail => ({
    category: { name: '', division: null, gender, weight_class: '', rank_by: 'ris' },
    participants: [lifter(gender, { ris_score: ris })],
  });

  expect(ids(mixedResults([field('M', '80'), field('F', '95')], 'ris'))).toEqual([
    ['F', 1],
    ['M', 2],
  ]);
});

it('finds an athlete by any part of the name, in any order and without accents', () => {
  const athlete = result({
    athlete: { ...result().athlete, first_name: 'Zoé', last_name: 'Lévêque' },
  });
  expect(matchesAthlete(athlete, 'leveque zoe')).toBe(true);
  expect(matchesAthlete(athlete, 'Lév')).toBe(true);
  expect(matchesAthlete(athlete, 'martin')).toBe(false);
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
        { gender: 'F', classes: [], allWeights: true, rankBy: 'total' },
        { gender: 'M', classes: ['-66kg', '+87kg'], allWeights: false, rankBy: 'total' },
      ],
    },
    {
      name: 'FNSL Elite',
      groups: [{ gender: 'MX', classes: [], allWeights: true, rankBy: 'ris' }],
    },
    {
      name: 'FNSL Masters',
      groups: [{ gender: 'M', classes: [], allWeights: true, rankBy: 'ris' }],
    },
  ]);
});

it('summarizes a meet without divisions as one list of fields, women first', () => {
  const summary = contestSummary([
    contest(null, 'M', '-80kg', 'total'),
    contest(null, 'F', '-63kg', 'total'),
  ]);

  expect(summary).toHaveLength(1);
  expect(summary[0].name).toBeNull();
  expect(summary[0].groups.map(({ gender }) => gender)).toEqual(['F', 'M']);
});

it('keeps a rule per sex when one division ranked its sexes differently', () => {
  const [division] = contestSummary([
    contest(null, 'M', '-80kg', 'total'),
    contest(null, 'F', '', 'ris'),
  ]);

  expect(division.groups.map(({ gender, rankBy }) => [gender, rankBy])).toEqual([
    ['F', 'ris'],
    ['M', 'total'],
  ]);
});

it('counts made and taken attempts per movement and attempt number', () => {
  const lift = (movement_name: 'Pull-up' | 'Dips', made: boolean[]) => ({
    movement_name,
    best_weight: null,
    record: null,
    attempts: made.map((is_successful, index) => ({
      attempt_number: index + 1,
      is_successful,
      weight: '50',
    })),
  });
  const field: CategoryDetail = {
    category: { name: '', division: null, gender: 'M', weight_class: '', rank_by: 'ris' },
    participants: [
      result({ lifts: [lift('Pull-up', [true, true, false]), lift('Dips', [true])] }),
      result({ status: 'disqualified', lifts: [lift('Pull-up', [true, false, false])] }),
    ],
  };

  expect(attemptSuccess([field], ['Pull-up', 'Dips'])).toEqual([
    {
      movement: 'Pull-up',
      rates: [
        { attempt: 1, made: 2, taken: 2 },
        { attempt: 2, made: 1, taken: 2 },
        { attempt: 3, made: 0, taken: 2 },
      ],
    },
    {
      movement: 'Dips',
      rates: [
        { attempt: 1, made: 1, taken: 1 },
        { attempt: 2, made: 0, taken: 0 },
        { attempt: 3, made: 0, taken: 0 },
      ],
    },
  ]);
});
