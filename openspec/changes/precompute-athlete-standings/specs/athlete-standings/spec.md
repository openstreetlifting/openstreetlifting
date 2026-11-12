# Spec Delta

## Purpose

Provides precomputed per-athlete metric standings (global and country places, field sizes) and rivals (adjacent athletes in the ranking), refreshed by the import pipeline, so athlete profile pages do not recompute rankings against the entire database on every request.

## ADDED Requirements

### Requirement: Athlete standings are served from precomputed data
The athlete detail API (`GET /api/v1/athletes/{slug}` with `include=standing`) SHALL return each metric's standing (value, global place, global field size, country, country place, country field size, weight class bounds) from precomputed rows, not from an on-the-fly ranking of all athletes.

#### Scenario: Athlete page requests standings
- **WHEN** a client requests an athlete with `include=standing`
- **THEN** the response contains one standing entry per metric the athlete has a scored result for, with global and country place and field sizes, and completes in well under one second

#### Scenario: Standing shape is unchanged
- **WHEN** the standings are served from precomputed data
- **THEN** the response shape and field semantics are identical to the previous computed response, so existing clients (frontend included) work without modification

### Requirement: Rivals are served from precomputed data
The athlete detail API SHALL return the athlete's current rivals (neighboring athletes immediately above and below in the total ranking, per the athlete's latest weight class) from precomputed rows.

#### Scenario: Athlete page requests neighbors
- **WHEN** a client requests an athlete with `include=neighbors`
- **THEN** the response contains the adjacent athletes above and below the athlete in the current total ranking, in the same shape as before

### Requirement: Standings are recomputed after each import
The import pipeline SHALL recompute the precomputed standings for all affected metrics and athletes after successfully importing competition data, before the import is reported as complete.

#### Scenario: Import updates standings
- **WHEN** an import job successfully persists new competition results
- **THEN** the stored standings reflect the newly imported results, and subsequent athlete API reads reflect them

#### Scenario: Import failure does not corrupt standings
- **WHEN** an import job fails before completing
- **THEN** previously stored standings remain intact and consistent with the data already visible to users

### Requirement: Missing precomputed standings degrade gracefully
When no precomputed standing exists for an athlete and metric, the API SHALL return that metric as absent (or explicitly unknown) rather than computing a full-database ranking on the fly or returning an error.

#### Scenario: Newly created athlete before any recompute
- **WHEN** an athlete has results but no precomputed standing rows yet
- **THEN** the standings include section omits or marks those metrics as unknown, the request succeeds, and no full ranking computation is triggered

### Requirement: Precomputed standings stay consistent with a manual recompute
The system SHALL provide a way to trigger a full standings recompute outside the import flow (e.g. via the import job or an operator command) so that standings can be rebuilt if the computation logic changes.

#### Scenario: Operator triggers full recompute
- **WHEN** a full recompute is triggered
- **THEN** all standings and rivals rows are rebuilt from current competition data with a deterministic result independent of request traffic
