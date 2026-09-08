import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

import { ATHLETE_STATUSES, COMPETITION_STATUSES, GENDERS, RIS_SOURCES } from './enums';

/**
 * The API's own description of itself, written by
 * `cargo run -p osl_api -- --dump-openapi > openapi.json`.
 */
const schema = JSON.parse(
  readFileSync(fileURLToPath(new URL('../../../../backend/openapi.json', import.meta.url)), 'utf8')
);

function published(name: string): readonly string[] {
  const definition = schema.components?.schemas?.[name];

  expect(definition, `${name} is not in the published schema`).toBeDefined();

  return definition.enum;
}

describe('the vocabularies the API publishes', () => {
  /**
   * The frontend cannot import Rust enums, so it restates them. This is what
   * stops the restatement from drifting: if it fails, either regenerate
   * backend/openapi.json or fix the array in enums.ts, whichever is behind.
   */
  it.each([
    ['Gender', GENDERS],
    ['CompetitionStatus', COMPETITION_STATUSES],
    ['AthleteStatus', ATHLETE_STATUSES],
    ['RisSource', RIS_SOURCES],
  ])('%s matches what the frontend expects', (name, expected) => {
    expect(published(name)).toEqual([...expected]);
  });
});
