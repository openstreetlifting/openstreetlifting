/**
 * The closed vocabularies the API publishes.
 *
 * These used to be written out inline wherever a field needed them, which meant
 * four copies of the same union and no way to notice when one of them stopped
 * matching the API. They are declared once here instead, as arrays so the UI can
 * also iterate them, and `enums.test.ts` checks them against the schema in
 * `backend/openapi.json`. Renaming a variant on the backend now fails a test
 * rather than silently breaking a filter or a label at runtime.
 */

/** Which category an athlete competes in. */
export const GENDERS = ['M', 'F', 'MX'] as const;
export type Gender = (typeof GENDERS)[number];

/**
 * The genders a ranking can be drawn for. Narrower than `Gender`: weight
 * classes are only drawn for men and women, so the board has no mixed field to
 * compare against and the API refuses `MX` on that parameter.
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

/**
 * Outcome of an athlete's participation. `competed` is the only one whose
 * result stands, so the other two are the ones the results table has to explain.
 */
export const ATHLETE_STATUSES = ['competed', 'disqualified', 'no_show'] as const;
export type AthleteStatus = (typeof ATHLETE_STATUSES)[number];

/**
 * Where a score came from. `computed` was worked out from the athlete's
 * bodyweight and total. `reported` was stated by the source, which gave no
 * bodyweight, so it cannot be restated on the formula everything else uses.
 */
export const RIS_SOURCES = ['computed', 'reported'] as const;
export type RisSource = (typeof RIS_SOURCES)[number];

/** What a ranking board can be sorted by. `total` and `ris` are not lifts. */
export const RANKING_MOVEMENTS = ['muscleup', 'pullup', 'dips', 'squat', 'total', 'ris'] as const;
export type RankingMovement = (typeof RANKING_MOVEMENTS)[number];

/** Which way a sorted list runs. */
export const SORT_DIRECTIONS = ['desc', 'asc'] as const;
export type SortDirection = (typeof SORT_DIRECTIONS)[number];

/**
 * Narrows a raw query parameter to a value the API will accept.
 *
 * A URL can say anything, and the API answers a value it does not know with a
 * 400. Dropping it instead falls back to the default view, which is what
 * someone who hand-edited the query string is more likely to have wanted than
 * an error page.
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

export function asRankingMovement(raw: string | null | undefined): RankingMovement | null {
  return narrow(RANKING_MOVEMENTS, raw);
}
