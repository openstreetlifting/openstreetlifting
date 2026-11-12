# Design

## Context

See proposal.md. `get_athlete_metric_standings` and `get_athlete_total_neighbors` (in `backend/crates/osl_db/src/repository/ranking.rs`) rebuild a global ranking per request by aggregating every participant x lift via `movement_weights(None)`, then applying `DISTINCT ON` pools and global `ROW_NUMBER()` windows. Postgres sustains ~0.36 cores because of this; the athlete page (`include=standing,neighbors`) triggers two such computations concurrently. The backend is Rust + Axum + SQLX against PostgreSQL (CloudNativePG), with a single import job (`openstreetlifting-import`) as the only writer of performance data. An in-memory cache layer already exists (`osl_db`/`osl_api` `cache/policy.rs`, used by `ris_distribution`, 1h TTL).

## Goals / Non-Goals

**Goals:**
- Flat, request-independent latency for athlete standings and rivals (tens of ms).
- Cost moved off the request path into the import pipeline (already batch, already async).
- No API contract change; frontend untouched.
- Deterministic, rebuildable standings (full recompute must yield identical results from the same data).

**Non-Goals:**
- Changing the public rankings page behavior (`/api/v1/rankings` stays computed on the fly; it is fast and legitimately needs full ranking).
- Incremental/streaming recompute (full recompute per import is sufficient at current data volume; revisit later).
- Caching athlete detail responses (separate, optional follow-up).

## Decisions

1. **Materialized standings table, recomputed at import time** (vs. request-time bounded queries, vs. response caching).
   - Standings only change when competition data changes; imports are the sole writer and run infrequently. A full recompute per import is O(same work as today) but once per import instead of once per page view.
   - Alternatives considered: bounding the query per request (keeps ~seconds latency ceiling and complex SQL on hot path); in-memory TTL cache (hides but does not fix cold requests and costs memory in a 256MB container).

2. **One table, standings and rivals in one model.**
   `athlete_metric_standings`: one row per (metric, athlete) with `value, global_place, global_field, country, country_place, country_field, weight_class_id, weight_class_min, weight_class_max, updated_at`. PK `(metric, athlete_id)`; index `(metric, global_place)` for neighbor lookups; index `(metric, country, country_place)`.
   Rivals do not need a separate table: with `(metric, global_place)` stored, neighbors of place *n* are rows at *n-1* and *n+1* — but the rivals section is specifically the **total** ranking in the athlete's latest weight class, so rivals are derived at recompute time by the same query that assigns total places and stored in a small `athlete_rivals` table (or as `total_place` + neighbor athlete ids on the total rows). Prefer explicit `athlete_rivals` rows (athlete_id, direction above/below, neighbor id/value) for a trivially indexed read.

3. **Recompute reuses the existing SQL pipeline.** The recompute step reuses `movement_weights` / metric-candidate logic as a `INSERT INTO ... SELECT` with window functions, executed in one transaction per recompute (truncate-and-insert or upsert). Rationale: the query is already correct and tested against production semantics; duplicating logic in Rust would risk divergence with the rankings page.
   Alternative considered: computing standings incrementally per touched athlete — rejected for now (cross-athlete rank changes when any athlete improves; per-metric full recompute is simpler and correct).

4. **Trigger point: end of the import job, before success is reported.** The importer already runs as a Job (panel-52 in the ops dashboard tracks it). The recompute becomes a post-import step in the same job run. On failure, the job fails and old standings remain (they are simply stale, never wrong relative to visible data if recomputation is transactional).

5. **No fallback computation.** If a row is missing, the API omits the metric (spec: degrade, don't compute). This keeps worst-case latency bounded and avoids resurrecting the full scan under failure conditions.

## Risks / Trade-offs

- [Staleness between import completion and recompute] → recompute runs inside the import job before completion; standings lag data at most by the recompute duration.
- [Full recompute duration grows with data] → acceptable for years at current volumes; the import already runs minutes-long jobs. If it becomes slow, move to incremental per-metric/class recompute.
- [Duplicate ranking semantics] (movement_weights used by both live rankings page and recompute) → extract the shared SQL so both paths stay consistent; regression risk covered by comparing standings output against the old query during rollout.
- [Behavior drift between computed and precomputed values (tie-breaking, class selection)] → snapshot-test: run old query and new table side by side in CI/staging on the same dataset; parity check task included.

## Migration Plan

1. Add migration creating `athlete_metric_standings` (+ `athlete_rivals`) with indexes.
2. Implement recompute step in the import job; run one full backfill recompute at deploy time (manual job run or startup hook).
3. Switch athlete API read paths to the new tables behind the existing handler (same response shape).
4. Verify on the ops dashboard: p95 of `/api/v1/athletes/{slug}` and Postgres CPU drop.
5. Rollback: revert the API change to the old computed path (kept until parity is confirmed); the tables are additive and harmless if unused.
