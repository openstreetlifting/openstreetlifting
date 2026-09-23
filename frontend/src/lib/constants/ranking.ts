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
 * Why a competition has no RIS, which decides how the table should read.
 *
 * `not-contested` is a property of the meet: RIS is fitted to a four lift total,
 * so a shorter event never had one and the column does not apply.
 * `not-published` is a gap in the record: the meet ran all four, but its source
 * gave no bodyweight and none could be recovered, so the column applies and
 * stays visible while the score itself is missing.
 */
export type RisAvailability = 'available' | 'not-contested' | 'not-published';

type RisShape = {
  movements: unknown[];
  categories: { participants: { ris_score?: string | null }[] }[];
};

export function risAvailability(competition: RisShape): RisAvailability {
  if (competition.movements.length !== ALL4_MOVEMENTS) {
    return 'not-contested';
  }
  const scored = competition.categories.some((category) =>
    category.participants.some((participant) => participant.ris_score != null)
  );
  return scored ? 'available' : 'not-published';
}

/**
 * Sorting by a score no one has drops every row, so the table falls back to the
 * total the meet was actually decided on.
 */
export function defaultRankingSort(availability: RisAvailability): RankingMetric {
  return availability === 'available' ? 'ris' : 'total';
}
