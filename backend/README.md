# Backend

The backend provides a read-only API and tools to import competition files into PostgreSQL.

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
cargo sqlx migrate run --source crates/osl_db/migrations
cargo run -p osl_api
```

The API loads `.env` and runs database migrations on startup. With the example settings, the API is at <http://localhost:8080> and Swagger UI is at <http://localhost:8080/swagger-ui/>.

After pulling schema changes, run the migration command before compiling the API
or importer. SQLx checks queries against the database during compilation, so new
columns must already exist. `SQLX_OFFLINE=true` uses the checked-in query metadata
for compilation; the database still needs the migrations before running imports.

Once the API has started, import competition data in another terminal from `backend`. If the privacy list contains records, set `OSL_PRIVACY_KEY` to the key used to create them; see the [athlete data guide](data/athletes/README.md).

```sh
cargo run -p osl_importer --bin import -- competitions
```

See the [importer guide](crates/osl_importer/README.md) for validation, formatting, and other commands.

## Configuration

| Variable | Purpose |
| --- | --- |
| `DATABASE_URL` | Required PostgreSQL connection URL |
| `HOST`, `PORT` | Required API bind address and port; `.env.example` uses `127.0.0.1:8080` |
| `RUST_LOG` | Log filter; defaults to `info` |
| `LOG_FORMAT` | Set to `json` for JSON logs; otherwise uses text |
| `CACHE_ENABLED` | Enable API dataset caches; defaults to `false` |
| `OSL_PRIVACY_KEY` | Importer key for records that prevent removed names from being republished |

### Caching

Helm deployments enable caching. Each API process caches the RIS distribution response for one hour, including its athlete and competition details. Formula endpoints are not cached.

The first request loads the dataset. After the cache expires, requests wait for a fresh load. Failed or cancelled loads do not update the cache. A failed refresh returns an error instead of expired data.

Imports do not clear caches, so changes, including name removal, can take up to an hour to reach API responses. For an immediate refresh, restart every API replica through the deployment workflow. Reload open browser pages to fetch the updated data.

To test caching locally, run `CACHE_ENABLED=true cargo run -p osl_api`. Restart the API to clear its cache or apply configuration changes. The `osl_api::cache` log target reports cache hits and refresh durations at `debug` level and failures at `warn` level.

To cache another fixed dataset, add a policy in [`cache/policy.rs`](crates/osl_api/src/cache/policy.rs) and a typed slot in `AppCaches`, then wrap its loader with `get_or_try_init`. Use `Bytes` or `Arc<T>` to avoid copying large values. Keep memory use bounded: these slots hold shared datasets.

## Checks

From `backend`:

```sh
cargo fmt --all --check
SQLX_OFFLINE=true cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Database tests need a running PostgreSQL server and a role that can create test databases. Export `DATABASE_URL` from your local configuration, then run:

```sh
SQLX_OFFLINE=true cargo test --workspace --all-features --locked
```

`SQLX_OFFLINE=true` lets builds use the checked-in query metadata; database tests still connect to PostgreSQL. After changing SQL queries, refresh the metadata against a migrated development database with `cargo sqlx prepare --workspace`.

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

To change data, edit competition files under `data/competitions`, validate them, then import them. Each competition has a `competition.toml` and, once results are available, an `entries.csv`. The API serves this data without authentication and exposes no write endpoints.

See the [data contribution guide](docs/src/CONTRIBUTING_DATA.md) for the format and review process.
