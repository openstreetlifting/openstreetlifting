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
