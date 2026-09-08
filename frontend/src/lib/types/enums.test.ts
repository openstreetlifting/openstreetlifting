import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

import {
  ATHLETE_STATUSES,
  COMPETITION_STATUSES,
  GENDERS,
  RANKED_GENDERS,
  RANKING_MOVEMENTS,
  RIS_SOURCES,
  SORT_DIRECTIONS,
} from './enums';

/**
 * The API's own description of itself, written by
 * `cargo run -p osl_api -- --dump-openapi > openapi.json` and kept current by a
 * test in `osl_api`'s `main.rs`.
 */
const schema = JSON.parse(
  readFileSync(fileURLToPath(new URL('../../../../backend/openapi.json', import.meta.url)), 'utf8')
);

/** Every schema component that is a closed list of strings. */
const published = new Map<string, readonly string[]>(
  Object.entries(schema.components?.schemas ?? {})
    .filter(([, definition]) => Array.isArray((definition as { enum?: unknown }).enum))
    .map(([name, definition]) => [name, (definition as { enum: string[] }).enum])
);

/** The TypeScript restatement of each, under the name the schema gives it. */
const restated = new Map<string, readonly string[]>([
  ['AthleteStatus', ATHLETE_STATUSES],
  ['CompetitionStatus', COMPETITION_STATUSES],
  ['Direction', SORT_DIRECTIONS],
  ['Gender', GENDERS],
  ['Movement', RANKING_MOVEMENTS],
  ['RankedGender', RANKED_GENDERS],
  ['RisSource', RIS_SOURCES],
]);

describe('the vocabularies the API publishes', () => {
  /**
   * The frontend cannot import Rust enums, so it restates them. This is what
   * stops the restatement from drifting: if it fails, either regenerate
   * backend/openapi.json or fix the array in enums.ts, whichever is behind.
   */
  it.each([...restated.keys()])('%s matches what the frontend expects', (name) => {
    expect(published.get(name), `${name} is not in the published schema`).toBeDefined();
    expect(published.get(name)).toEqual([...restated.get(name)!]);
  });

  /**
   * Listing them one by one would guard only the ones someone remembered to
   * add, which is the same gap the restatement itself has. A new enum on the
   * backend fails here until the frontend either restates it or is told, in
   * this file, that it does not need to.
   */
  it('has no vocabulary the frontend has not accounted for', () => {
    expect([...published.keys()].sort()).toEqual([...restated.keys()].sort());
  });
});
