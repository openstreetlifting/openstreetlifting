# Proposal

## Why

`GET /api/v1/athletes/{slug}` has a p50 of ~3.5s and a p95 of ~4.9s in production. Comes from `get_athlete_metric_standings` and `get_athlete_total_neighbors`, which scan entire database (every participant x lift aggregation, global `ROW_NUMBER()` window sorts) on every athlete page view to answer a single-athlete question. Latency is flat ~2.5-4s around the clock and grows with database size, so it will keep degrading as data accumulates.

## What Changes

- Precompute athlete metric standings (global place/field and country place/field per metric: Muscle-up, Pull-up, Dips, Squat, Total, RIS) at import time into a new `athlete_metric_standings` table, instead of ranking the whole database per request.
- Precompute the "current rivals" neighbor data (adjacent athletes above/below in the total ranking) at import time as well, replacing the second per-request full computation.
- Switch the athlete-detail handler (`GET /api/v1/athletes/{slug}` with `include=standing,neighbors`) to read the precomputed rows via indexed single-athlete lookups.
- Recompute standings for affected athletes/metrics after each import job run (the import pipeline is the only writer of performance data).
- The public rankings page keeps its existing on-the-fly computation (it legitimately ranks everyone, is paginated, and is fast at ~120ms p50 because filters prune the candidate set).

## Capabilities

### New Capabilities

- `athlete-standings`: Precomputed per-athlete metric standings and rivals (global/country places, field sizes, neighbors), refreshed by the import pipeline and served by the athlete detail API.

### Modified Capabilities

<!-- None: no existing specs in this repository yet (fresh OpenSpec root). -->

## Impact

- **Backend DB** (`backend/crates/osl_db/`): new migration for the `athlete_metric_standings` table (+ indexes on `(athlete_id, metric)`, `(metric, global_place)`); new/changed repository functions replacing `get_athlete_metric_standings` and `get_athlete_total_neighbors` read paths (`ranking.rs`).
- **Import pipeline** (`openstreetlifting-import` job): new post-import recompute step; full recompute initially, incremental (only touched athletes/classes) later if needed.
- **API** (`backend/crates/osl_api/src/athlete/`): handler stays backward compatible (same response shape for `standing` and `neighbors` includes); latency expected to drop from ~3.5s to tens of milliseconds.
- **Observability**: verify via the "OpenStreetLifting Operations Essentials" dashboard (p95 for `/api/v1/athletes/{slug}`) and Postgres CPU (`osl-postgres` currently ~0.36 cores sustained).
- No frontend changes, no API contract changes, no data backfill risk (data is derived; a stale/missing row falls back to "unknown standing" rather than wrong values).
