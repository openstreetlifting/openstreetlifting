# Backend

The Rust workspace serves the read-only API and imports competition files into PostgreSQL.

| Crate | Purpose |
| --- | --- |
| `osl_api` | Axum API, OpenAPI schema, and response caches |
| `osl_db` | Database queries, migrations, and services |
| `osl_domain` | Shared types, identity rules, and scoring formulas |
| `osl_importer` | File validation, imports, formatting, and name removal |

## Run locally

Install Rust and Docker. From `backend`:

```sh
cp .env.example .env
docker compose -f ../docker-compose.yaml up -d --wait postgres
cargo run -p osl_api
```

The API loads `.env` and runs migrations on startup. With the example settings, it listens at <http://localhost:8080>; Swagger UI is at `/swagger-ui/`.

After the database is ready, import competition data in another terminal from `backend`:

```sh
cargo run -p osl_importer --bin import -- competitions
```

A nonempty privacy list requires its existing `OSL_PRIVACY_KEY`. See the [importer guide](crates/osl_importer/README.md) and [athlete data](data/athletes/README.md).

## Configuration

| Variable | Purpose |
| --- | --- |
| `DATABASE_URL` | Required PostgreSQL connection URL |
| `HOST`, `PORT` | API bind address; `.env.example` uses `127.0.0.1:8080` |
| `RUST_LOG` | Log filter |
| `CACHE_ENABLED` | Enable API dataset caches; defaults to `false` |
| `OSL_PRIVACY_KEY` | Importer key for the suppression list |

### Caching

Helm deployments enable caching. Each API process caches the serialized RIS distribution for one hour, including its athlete and competition details. Formula endpoints are not cached.

The first request loads the dataset. Requests after expiry wait for a fresh load. Failed or cancelled loads do not populate the cache, and a failed refresh does not serve expired data.

Imports do not invalidate caches. Changes, including name removal, can take up to an hour to appear after a page reload. For an immediate refresh, restart every API replica through the deployment workflow. Open browser pages keep their loaded data until reloaded.

Use `CACHE_ENABLED=true cargo run -p osl_api` to test caching locally. A restart clears the process cache and applies configuration changes. Cache hits and refresh durations are logged at debug level under `osl_api::cache`; failures are logged at warn level.

Cache policies live in `crates/osl_api/src/cache/policy.rs`. To cache another fixed dataset, add its policy and typed slot to `AppCaches`, then wrap its loader with `get_or_try_init`. Use `Bytes` or `Arc<T>` to avoid large clones. These slots are for shared datasets with bounded memory use.

## Checks

From `backend`:

```sh
cargo fmt --all --check
SQLX_OFFLINE=true cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Database tests need a running PostgreSQL server and a role that can create test databases. Export `DATABASE_URL` from your local configuration before running:

```sh
SQLX_OFFLINE=true cargo test --workspace --locked
```

`SQLX_OFFLINE=true` uses the checked-in query metadata. After changing SQL queries, refresh it against a migrated development database with `cargo sqlx prepare --workspace`.

## Migrations and API schema

Migrations live in `crates/osl_db/migrations`. With `sqlx-cli` installed:

```sh
sqlx migrate add --source crates/osl_db/migrations <name>
sqlx migrate run --source crates/osl_db/migrations
```

After changing API routes or response types, regenerate the checked-in schema:

```sh
cargo run -p osl_api -- --dump-openapi > openapi.json
```

## Data changes

The API serves data without authentication and exposes no write endpoints. Edit competition files under `data/competitions`, validate them, then import them. Each competition has a `competition.toml` and, once results exist, an `entries.csv`.

See the [data contribution guide](docs/src/CONTRIBUTING_DATA.md) for the format and review process.
