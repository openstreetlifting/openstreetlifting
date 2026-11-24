import type { CategoryDetail, Participant } from '#lib/types/competition.js';
import type { Gender, Movement, RankingMetric } from '#lib/types/enums.js';

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

export type CompetitionDivision = { name: string; rankBy: RankingMetric; entries: number };

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
