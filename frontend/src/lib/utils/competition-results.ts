import type { CategoryDetail, Participant } from '#lib/types/competition.js';
import { type Gender, type Movement, type RankBy, type RankingMetric } from '#lib/types/enums.js';

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

const movements: Record<Exclude<RankingMetric, 'total' | 'ris'>, Movement> = {
  muscleup: 'Muscle-up',
  pullup: 'Pull-up',
  dips: 'Dips',
  squat: 'Squat',
};

/** The leaderboard omits missing scores; the competition page still lists those entries. */
export function hasRankingResult(participant: Participant, metric: RankingMetric): boolean {
  if (participant.status !== 'competed') return false;
  if (metric === 'ris') return participant.ris_score != null;
  if (metric === 'total') return participant.total != null;
  return participant.lifts.some(
    (lift) => lift.movement_name === movements[metric] && lift.best_weight != null
  );
}

export type CompetitionDivision = { name: string; rankBy: RankBy; entries: number };

/** The largest division opens first; a meet without two named divisions has one board. */
export function competitionDivisions(categories: CategoryDetail[]): CompetitionDivision[] {
  const divisions = new Map<string, CompetitionDivision>();
  for (const { category, participants } of categories) {
    if (!category.division) continue;
    const division = divisions.get(category.division) ?? {
      name: category.division,
      rankBy: category.rank_by,
      entries: 0,
    };
    division.entries += participants.length;
    divisions.set(category.division, division);
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

const FIELD_ORDER: Gender[] = ['F', 'M', 'MX'];

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
