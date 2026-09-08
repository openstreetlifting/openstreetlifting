/**
 * The closed vocabularies the API publishes, checked against its schema by
 * enums.test.ts.
 */

/** Which category an athlete competes in. */
export const GENDERS = ['M', 'F', 'MX'] as const;
export type Gender = (typeof GENDERS)[number];

/**
 * The genders a ranking can be drawn for. Weight classes are only drawn for men
 * and women, so the API refuses `MX` on that parameter.
 */
export const RANKED_GENDERS = ['M', 'F'] as const;
export type RankedGender = (typeof RANKED_GENDERS)[number];

/** Where a competition is in its life. */
export const COMPETITION_STATUSES = [
  'draft',
  'upcoming',
  'live',
  'completed',
  'cancelled',
] as const;
export type CompetitionStatus = (typeof COMPETITION_STATUSES)[number];

/** Outcome of an athlete's participation. Only `competed` stands as a result. */
export const ATHLETE_STATUSES = ['competed', 'disqualified', 'no_show'] as const;
export type AthleteStatus = (typeof ATHLETE_STATUSES)[number];

/**
 * Where a score came from. `reported` was stated by the source without a
 * bodyweight, so it cannot be restated on the formula everything else uses.
 */
export const RIS_SOURCES = ['computed', 'reported'] as const;
export type RisSource = (typeof RIS_SOURCES)[number];

/** The four lifts, spelled the way the API names them. */
export const MOVEMENTS = ['Muscle-up', 'Pull-up', 'Dips', 'Squat'] as const;
export type Movement = (typeof MOVEMENTS)[number];

/** What a ranking board can be sorted by. `total` and `ris` are not lifts. */
export const RANKING_METRICS = ['muscleup', 'pullup', 'dips', 'squat', 'total', 'ris'] as const;
export type RankingMetric = (typeof RANKING_METRICS)[number];

/** Which way a sorted list runs. */
export const SORT_DIRECTIONS = ['desc', 'asc'] as const;
export type SortDirection = (typeof SORT_DIRECTIONS)[number];

/**
 * Narrows a raw query parameter to a value the API accepts. A URL can say
 * anything, and falling back beats showing an error page.
 */
function narrow<T extends string>(
  vocabulary: readonly T[],
  raw: string | null | undefined
): T | null {
  return raw && (vocabulary as readonly string[]).includes(raw) ? (raw as T) : null;
}

export function asRankedGender(raw: string | null | undefined): RankedGender | null {
  return narrow(RANKED_GENDERS, raw);
}

export function asRankingMetric(raw: string | null | undefined): RankingMetric | null {
  return narrow(RANKING_METRICS, raw);
}
