# Tasks

## 1. Database migration

- [ ] 1.1 Create migration adding `athlete_metric_standings` table (PK `(metric, athlete_id)`; columns: value, global_place, global_field, country, country_place, country_field, weight_class_id, weight_class_min, weight_class_max, updated_at) with indexes on `(metric, global_place)` and `(metric, country, country_place)`; verify `sqlx migrate run` applies cleanly on a scratch database
- [ ] 1.2 Create migration adding `athlete_rivals` table (athlete_id, neighbor athlete_id, direction above/below, class context) with PK/index on `athlete_id`; verify with `sqlx migrate run`

## 2. Recompute pipeline

- [ ] 2.1 Extract the shared ranking SQL (`movement_weights` and metric-candidate derivation) into a reusable query module so the importer and the live rankings page use one source of truth; verify the existing rankings page tests still pass unchanged
- [ ] 2.2 Implement `recompute_standings` in `osl_db` (INSERT INTO ... SELECT with DISTINCT ON pools and ROW_NUMBER windows, transactional truncate-and-insert) and add a unit/integration test asserting global_place, field sizes and country places match the current `get_athlete_metric_standings` output on a seeded fixture; verify the parity test passes
- [ ] 2.3 Implement rivals computation (neighbors above/below on the total ranking within the athlete's latest weight class) and test that neighbors of a known fixture athlete are correct; verify the test passes
- [ ] 2.4 Wire the recompute as a post-import step in the `openstreetlifting-import` job (runs before the job reports success; failure fails the job) and verify by running the job against a staging-like database that standings rows exist and match the parity query
- [ ] 2.5 Add an operator-triggerable full recompute path (same function, exposed via the existing job/manual mechanism); verify a manual run rebuilds rows deterministically

## 3. API read path

- [ ] 3.1 Replace the `include=standing` read path to select from `athlete_metric_standings` (single-athlete indexed lookup, same response shape), keeping the old implementation available behind the scenes for parity checking; verify the existing handler tests pass and add a test that standings come from the table (e.g. mutate a row and observe the response)
- [ ] 3.2 Implement the `include=neighbors` read path from `athlete_rivals` (or equivalent indexed lookup) with the same response shape; verify handler tests pass
- [ ] 3.3 Implement missing-row behavior: a metric with no precomputed row is omitted/marked unknown and never triggers a full ranking computation; verify with a test on an athlete lacking standings rows
- [ ] 3.4 Run `cargo test` for `osl_db` and `osl_api` and confirm the full suite is green

## 4. Verification

- [ ] 4.1 Deploy to staging, run one import, and compare standings API responses (new table vs old computed path) for a sample of athletes; verify parity within tie-breaking tolerance
- [ ] 4.2 Verify on the OpenStreetLifting Operations Essentials dashboard that p95 of `/api/v1/athletes/{slug}` drops from ~4.9s to <0.3s and Postgres CPU trend decreases

## Workflow follow-up

- Archive the change after review requirements are satisfied and the parity verification is recorded.
