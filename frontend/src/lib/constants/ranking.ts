import type { RankingMetric } from '$lib/types/enums';

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
 * `recomputed` and `reported` both follow the bodyweight: a score is recomputed
 * wherever the archive holds one, and the federation's published score is kept
 * where it does not, which the table marks row by row. A meet can carry both,
 * and is reported only when no score in it could be recomputed.
 * `not-contested` is a property of the meet, since RIS is fitted to a four lift
 * total and a shorter event never had one.
 * `not-published` is a gap in the record: all four were contested, but the
 * source gave neither bodyweight nor score.
 */
export type RisProvenance = 'recomputed' | 'reported' | 'not-contested' | 'not-published';

type RisShape = {
  movements: unknown[];
  categories: { participants: { ris_score?: string | null; ris_source?: string | null }[] }[];
};

export function risProvenance(competition: RisShape): RisProvenance {
  if (competition.movements.length !== ALL4_MOVEMENTS) {
    return 'not-contested';
  }
  const scored = competition.categories.flatMap((category) =>
    category.participants.filter((participant) => participant.ris_score != null)
  );
  if (scored.length === 0) {
    return 'not-published';
  }
  return scored.some((participant) => participant.ris_source !== 'reported')
    ? 'recomputed'
    : 'reported';
}

/**
 * Sorting by a score no one has drops every row, so the table falls back to the
 * total the meet was actually decided on.
 */
export function defaultRankingSort(provenance: RisProvenance): RankingMetric {
  return provenance === 'recomputed' || provenance === 'reported' ? 'ris' : 'total';
}
