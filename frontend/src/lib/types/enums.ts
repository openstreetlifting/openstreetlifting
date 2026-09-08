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

/**
 * Narrows a raw query parameter to a gender the API will accept.
 *
 * A URL can say anything, and the API answers a gender it does not know with a
 * 400. Dropping the value instead shows the unfiltered board, which is what
 * someone who hand-edited the query string is more likely to have wanted than
 * an error page.
 */
export function asGender(raw: string | null | undefined): Gender | null {
  return raw && (GENDERS as readonly string[]).includes(raw) ? (raw as Gender) : null;
}
