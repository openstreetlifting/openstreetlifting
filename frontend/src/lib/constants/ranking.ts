import type { RankingMetric } from '#lib/types/enums.js';

export const RANKING_LIFTS = [
  { value: 'muscleup', label: 'Muscle Up' },
  { value: 'pullup', label: 'Pull Up' },
  { value: 'dips', label: 'Dips' },
  { value: 'squat', label: 'Squat' },
] as const;

export const RANKING_SORTS = [
  { value: 'total', label: 'Total' },
  { value: 'ris', label: 'RIS' },
  ...RANKING_LIFTS,
] as const;

export const RANKING_SORTS_NO_RIS = RANKING_SORTS.filter((sort) => sort.value !== 'ris');

export const RANKING_GENDERS = [
  { value: null, label: 'All Sex' },
  { value: 'M', label: 'Men' },
  { value: 'F', label: 'Women' },
] as const;

/** Muscle up, pull up, dips and squat. A competition contesting all four is All4. */
const ALL4_MOVEMENTS = 4;

/**
 * Where a competition's RIS came from, or why it has none.
 *
 * All4 RIS is recomputed wherever bodyweight and total are available. Shorter events use published
 * scores when the meet explicitly ranks by RIS and has no weight classes.
 * A meet can carry both sources and is reported when none were recomputed.
 * `not-contested` means a shorter event has no ranking RIS.
 * `not-published` means the source lacks the score or inputs required for RIS ranking.
 */
export type RisProvenance = 'recomputed' | 'reported' | 'not-contested' | 'not-published';

type RisShape = {
  scoring?: string | null;
  movements: unknown[];
  categories: { participants: { ris_score?: string | null; ris_source?: string | null }[] }[];
};

export function risProvenance(competition: RisShape): RisProvenance {
  let hasReported = false;
  for (const category of competition.categories) {
    for (const participant of category.participants) {
      if (participant.ris_score == null) continue;
      if (participant.ris_source !== 'reported') return 'recomputed';
      hasReported = true;
    }
  }
  if (hasReported) return 'reported';
  return competition.movements.length === ALL4_MOVEMENTS || competition.scoring === 'ris'
    ? 'not-published'
    : 'not-contested';
}

/**
 * Sorting by a score no one has drops every row, so the table falls back to the
 * total the meet was actually decided on.
 */
export function defaultRankingSort(provenance: RisProvenance): RankingMetric {
  return provenance === 'recomputed' || provenance === 'reported' ? 'ris' : 'total';
}
