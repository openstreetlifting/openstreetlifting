export const RANKING_MOVEMENTS = [
  { value: 'muscleup', label: 'Muscle Up' },
  { value: 'pullup', label: 'Pull Up' },
  { value: 'dips', label: 'Dips' },
  { value: 'squat', label: 'Squat' },
  { value: 'total', label: 'Total' },
] as const;

/** Every column a rankings table can be sorted on, in table order. */
export const RANKING_SORTS = [...RANKING_MOVEMENTS, { value: 'ris', label: 'RIS' }] as const;

export const RANKING_GENDERS = [
  { value: null, label: 'All Sex' },
  { value: 'M', label: 'Men' },
  { value: 'F', label: 'Women' },
] as const;
