import type { CategoryDetail, Participant } from '#lib/types/competition.js';
import {
  GENDERS,
  type Gender,
  type Movement,
  type RankBy,
  type RankingMetric,
} from '#lib/types/enums.js';

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
  classed: boolean;
  rankBy: RankBy | null;
};

const shared = <T>(values: T[]): T | null =>
  values.length > 0 && values.every((value) => value === values[0]) ? values[0] : null;

function summarize(name: string | null, categories: CategoryDetail[]): DivisionSummary {
  const groups = GENDERS.flatMap((gender) => {
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
  return {
    name,
    groups,
    classed: groups.some((group) => group.classes.length > 0),
    rankBy: shared(categories.map(({ category }) => category.rank_by)),
  };
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

/** The rule every contest shares, or null when divisions or sexes were ranked differently. */
export function sharedRankBy(summary: DivisionSummary[]): RankBy | null {
  return shared(summary.map((division) => division.rankBy));
}

export type RankScope = 'class' | 'sex' | null;

/** Contests are split by weight class where any exist, otherwise by sex unless men and women met in one field. */
export function rankScope(groups: ContestGroup[]): RankScope {
  if (groups.some((group) => group.classes.length > 0)) return 'class';
  if (groups.length > 1 && groups.every((group) => group.gender !== 'MX')) return 'sex';
  return null;
}
