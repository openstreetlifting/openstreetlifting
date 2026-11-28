import type { CategoryDetail, Participant } from '#lib/types/competition.js';
import type { AthleteStatus, Gender, Movement, RankBy, RankingMetric } from '#lib/types/enums.js';
import { formatAthleteName } from './format.js';
import { slugify } from './slug.js';

/** Class labels are shared across divisions; capped classes precede open classes. */
export function weightClasses(categories: CategoryDetail[], gender: Gender): string[] {
  const max = (label: string) =>
    label.startsWith('-') ? Number.parseFloat(label.slice(1)) : Infinity;
  const min = (label: string) => (label.startsWith('+') ? Number.parseFloat(label.slice(1)) : 0);
  return [
    ...new Set(
      categories
        .filter(({ category }) => category.gender === gender)
        .map(({ category }) => category.weight_class)
    ),
  ].sort((a, b) => max(a) - max(b) || min(a) - min(b));
}

const FIELD_ORDER: Gender[] = ['F', 'M', 'MX'];

const movements: Record<Exclude<RankingMetric, 'total' | 'ris'>, Movement> = {
  muscleup: 'Muscle-up',
  pullup: 'Pull-up',
  dips: 'Dips',
  squat: 'Squat',
};

export function resultValue(participant: Participant, metric: RankingMetric): number | null {
  if (participant.status !== 'competed') return null;
  const value =
    metric === 'ris'
      ? participant.ris_score
      : metric === 'total'
        ? participant.total
        : participant.lifts.find((lift) => lift.movement_name === movements[metric])?.best_weight;
  return value == null ? null : Number(value);
}

export type ResultEntry = { participant: Participant; category: CategoryDetail['category'] };
export type ResultRow = ResultEntry & { rank: number | null };
export type ResultTable = { category: CategoryDetail['category']; rows: ResultRow[] };

const STATUS_ORDER: AthleteStatus[] = ['competed', 'disqualified', 'no_show'];

const descending = (a: number | null, b: number | null) => (b ?? -Infinity) - (a ?? -Infinity);
const lighterFirst = (a: Participant, b: Participant) =>
  (a.bodyweight == null ? Infinity : Number(a.bodyweight)) -
  (b.bodyweight == null ? Infinity : Number(b.bodyweight));

const unrankedOrder = (a: ResultEntry, b: ResultEntry) =>
  STATUS_ORDER.indexOf(a.participant.status) - STATUS_ORDER.indexOf(b.participant.status) ||
  a.participant.athlete.last_name.localeCompare(b.participant.athlete.last_name);

export function rankResults(entries: ResultEntry[], metric: RankingMetric): ResultRow[] {
  const value = (entry: ResultEntry) => resultValue(entry.participant, metric);
  const ranked = entries
    .filter((entry) => value(entry) != null)
    .sort(
      (a, b) =>
        descending(value(a), value(b)) ||
        (metric === 'total' ? lighterFirst(a.participant, b.participant) : 0) ||
        a.participant.athlete.athlete_id.localeCompare(b.participant.athlete.athlete_id)
    )
    .map((entry, index) => ({ ...entry, rank: index + 1 }));
  const unranked = entries
    .filter((entry) => value(entry) == null)
    .sort(unrankedOrder)
    .map((entry) => ({ ...entry, rank: null }));
  return [...ranked, ...unranked];
}

const entriesOf = ({ category, participants }: CategoryDetail): ResultEntry[] =>
  participants.map((participant) => ({ participant, category }));

export function resultTables(categories: CategoryDetail[], metric: RankingMetric): ResultTable[] {
  const classes = new Map(FIELD_ORDER.map((gender) => [gender, weightClasses(categories, gender)]));
  const classIndex = ({ category }: CategoryDetail) =>
    classes.get(category.gender)!.indexOf(category.weight_class);
  return [...categories]
    .sort(
      (a, b) =>
        FIELD_ORDER.indexOf(a.category.gender) - FIELD_ORDER.indexOf(b.category.gender) ||
        classIndex(a) - classIndex(b)
    )
    .map((detail) => ({ category: detail.category, rows: rankResults(entriesOf(detail), metric) }));
}

export function mixedResults(categories: CategoryDetail[], metric: RankingMetric): ResultRow[] {
  return rankResults(categories.flatMap(entriesOf), metric);
}

export function matchesAthlete(participant: Participant, query: string): boolean {
  const name = slugify(formatAthleteName(participant.athlete));
  return slugify(query)
    .split('-')
    .every((token) => name.includes(token));
}

export type CompetitionDivision = { name: string; rankBy: RankBy; entries: number };

/** The largest division opens first; a meet without two named divisions has one board. */
export function competitionDivisions(categories: CategoryDetail[]): CompetitionDivision[] {
  const divisions = new Map<string, CompetitionDivision>();
  for (const { category, participants } of categories) {
    if (!category.division) continue;
    let division = divisions.get(category.division);
    if (!division) {
      division = { name: category.division, rankBy: category.rank_by, entries: 0 };
      divisions.set(category.division, division);
    }
    division.entries += participants.length;
  }
  return [...divisions.values()].sort(
    (a, b) => b.entries - a.entries || a.name.localeCompare(b.name)
  );
}

export type ContestGroup = {
  gender: Gender;
  classes: string[];
  allWeights: boolean;
  rankBy: RankBy | null;
};

export type DivisionSummary = {
  name: string | null;
  groups: ContestGroup[];
};

const shared = <T>(values: T[]): T | null =>
  values.length > 0 && values.every((value) => value === values[0]) ? values[0] : null;

function summarize(name: string | null, categories: CategoryDetail[]): DivisionSummary {
  const groups = FIELD_ORDER.flatMap((gender) => {
    const contests = categories.filter(({ category }) => category.gender === gender);
    if (contests.length === 0) return [];
    return [
      {
        gender,
        classes: weightClasses(contests, gender).filter((label) => label !== ''),
        allWeights: contests.some(({ category }) => category.weight_class === ''),
        rankBy: shared(contests.map(({ category }) => category.rank_by)),
      },
    ];
  });
  return { name, groups };
}

/** One entry per division, in the order of the division switcher; a meet without divisions has one. */
export function contestSummary(categories: CategoryDetail[]): DivisionSummary[] {
  const divisions = competitionDivisions(categories);
  if (divisions.length < 2) return [summarize(divisions[0]?.name ?? null, categories)];
  return divisions.map(({ name }) =>
    summarize(
      name,
      categories.filter(({ category }) => category.division === name)
    )
  );
}

export type AttemptRate = { attempt: 1 | 2 | 3; made: number; taken: number };
export type MovementAttempts = { movement: Movement; rates: AttemptRate[] };

/** Every recorded attempt counts, whatever the athlete's final status. */
export function attemptSuccess(
  categories: CategoryDetail[],
  movements: Movement[]
): MovementAttempts[] {
  const attempts = categories.flatMap(({ participants }) =>
    participants.flatMap(({ lifts }) =>
      lifts.flatMap((lift) =>
        lift.attempts.map((attempt) => ({ ...attempt, movement: lift.movement_name }))
      )
    )
  );
  return movements.map((movement) => ({
    movement,
    rates: ([1, 2, 3] as const).map((number) => {
      const taken = attempts.filter(
        (attempt) => attempt.movement === movement && attempt.attempt_number === number
      );
      return {
        attempt: number,
        made: taken.filter((attempt) => attempt.is_successful).length,
        taken: taken.length,
      };
    }),
  }));
}
