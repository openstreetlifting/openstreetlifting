/** The four contested lifts, in the order a meet runs them. */
export const RANKING_LIFTS = [
  { value: 'muscleup', label: 'Muscle Up' },
  { value: 'pullup', label: 'Pull Up' },
  { value: 'dips', label: 'Dips' },
  { value: 'squat', label: 'Squat' },
] as const;

/**
 * Every column a rankings table can be sorted on, in table order. Total and RIS
 * lead: they are what a ranking is read for, and on a phone they are all that
 * fits beside the athlete without scrolling.
 */
export const RANKING_SORTS = [
  { value: 'total', label: 'Total' },
  { value: 'ris', label: 'RIS' },
  ...RANKING_LIFTS,
] as const;

/** The same columns for a competition that ran too few movements to have a RIS. */
export const RANKING_SORTS_NO_RIS = RANKING_SORTS.filter((sort) => sort.value !== 'ris');

export const RANKING_GENDERS = [
  { value: null, label: 'All Sex' },
  { value: 'M', label: 'Men' },
  { value: 'F', label: 'Women' },
] as const;

/** Muscle up, pull up, dips and squat. A competition contesting all four is All4. */
const ALL4_MOVEMENTS = 4;

/**
 * RIS is fitted to a four lift total, so the importer computes none for a
 * shorter event. Sorting such a competition by RIS drops every row, which is why
 * the default has to follow what the competition actually ran.
 */
export function hasRis(movementCount: number): boolean {
  return movementCount === ALL4_MOVEMENTS;
}

/** What a competition's table sorts on before anyone picks a column. */
export function defaultRankingSort(movementCount: number): string {
  return hasRis(movementCount) ? 'ris' : 'total';
}
