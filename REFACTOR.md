# Backend Refactor Guide

## Goal

Clean architecture with clear boundaries between crates:

- `osl_domain` — shared vocabulary only (enums, RIS logic, errors)
- `osl_db` — DB rows, queries, repositories
- `osl_api` — HTTP handlers, DTOs, OpenAPI schemas
- `osl_importer` — canonical format, source adapters, transformer

## Current State

- `osl_domain/src/models/` contains mixed structs with `FromRow + ToSchema + Serialize` all together
- `osl_db/src/dto/` contains DTOs that belong in `osl_api`
- DTOs have already been moved to `osl_api` with vertical slicing

## What osl_domain Should Contain (final state)

Keep only:
- `weight_class.rs` — `WeightClassSlug` enum + `WeightClass` struct (done)
- `athlete_status.rs` — `AthleteStatus` enum (done)
- `ris.rs` — RIS formula computation
- `error.rs` — shared error types

Delete `osl_domain/src/models/` entirely once all row structs are created in `osl_db`.

## Step 1: Create Row Structs in osl_db

For each model in `osl_domain/src/models/`, create a corresponding `XxxRow` in `osl_db/src/rows/`.

Rules for row structs:
- Derive only `FromRow` and `Debug`
- Keep all DB columns: foreign keys, `created_at`, `created_by`, DB-specific names
- Add `From<XxxRow> for Xxx` only if a domain struct exists that needs it
- For now most rows just need `FromRow`, no conversion needed

Order to migrate (bottom up by dependency):

    1. attempt
    2. movement
    3. social
    4. rulebook
    5. ris_formula
    6. lift
    7. federation
    8. ris_score
    9. record
    10. athlete
    11. athlete_social
    12. competition_movement
    13. category
    14. competition
    15. competition_participant

Example for `attempt`:

```rust
// osl_db/src/rows/attempt.rs
use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct AttemptRow {
    pub group_id: Uuid,
    pub athlete_id: Uuid,
    pub movement_name: String,
    pub attempt_number: i16,
    pub weight: Decimal,
    pub is_successful: bool,
    pub passing_judges: Option<i16>,
    pub no_rep_reason: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub created_by: Option<String>,
}
```

Add each new row module to `osl_db/src/rows/mod.rs`:

```rust
pub mod attempt;
pub mod movement;
// etc.
```

## Step 2: Update Repositories

For each `query_as!(SomeDomainStruct, ...)` call in `osl_db/src/repository/`,
update to use `XxxRow` instead:

```rust
// before
sqlx::query_as!(Competition, "SELECT ...")

// after
sqlx::query_as!(CompetitionRow, "SELECT ...")
```

Repositories return row structs. If the handler needs a response DTO,
the conversion happens in the handler, not the repository.

## Step 3: Update osl_db lib.rs

Remove the re-export of `osl_domain::models` once all rows are migrated:

```rust
// remove this line from osl_db/src/lib.rs
pub use osl_domain::{error, models};

// keep only
pub use osl_domain::error;
pub mod rows;
pub mod repository;
pub mod services;
```

## Step 4: Update osl_api main.rs

Remove all `osl_db::models::Xxx` references from the OpenAPI schema registration.
Replace with the new `crate::` DTO paths:

```rust
// before
osl_db::models::Competition,
osl_db::models::Athlete,

// after - these are now in osl_api DTOs
crate::competitions::dto::CompetitionResponse,
crate::athletes::dto::AthleteResponse,
```

## Step 5: Delete osl_domain/src/models/

Once all repositories use `XxxRow` and compile cleanly, delete the models directory.
Update `osl_domain/src/lib.rs` to remove the `pub mod models` line.

## Step 6: Update osl_db/src/dto/

Delete `osl_db/src/dto/` entirely. DTOs have moved to `osl_api`.
Update `osl_db/src/lib.rs` to remove `pub mod dto`.

## Canonical Format (osl_importer)

Separate task, do not mix with this refactor. The canonical format changes
(1.1.0 upgrade) are tracked in their own tickets.

## Compile Check After Each Step

After each model migration run:

```bash
cd backend && cargo check --workspace
```

Never move to the next model until the current one compiles.

## What NOT to Do

- Do not add business logic to row structs
- Do not add `ToSchema` to row structs
- Do not add `FromRow` to osl_domain types
- Do not create domain structs for concepts you do not have business logic for yet
- Do not mix the canonical format refactor with this DB refactor
