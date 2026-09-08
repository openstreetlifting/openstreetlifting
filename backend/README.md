# OpenStreetlifting Backend

OpenStreetlifting is a open collection of services powering the backend of the [openstreetlifting](https://openstreetlifting.org) website.

## Workspace Structure

- `storage` - Database models, migrations, and repository layer
- `web` - Actix Web REST API server
- `importer` - CLI tool for importing competition data from external sources

## Setup

There are multiple way of making openstreetlifting_backend run, you can go

- localhost : create your own postgresql instance and run rust command through local rustup install
- docker : a [compose](./docker-compose.yaml) file is available to launch all the necessary services
- hybrid : you can only start the postgres service, and use local rust for ease of development : `docker-compose up -d postgres`

## Development Commands

```sh
# Run web API (http://localhost:8080)
cargo run --bin web

# Run importer CLI
cargo run --bin import -- --database-url "postgresql://..." liftcontrol <event-slug>

# Linting, Formatting
cargo clippy
cargo fmt

# SQLx compile-time verification
cargo sqlx prepare --workspace
```

## Configuration

> [!tip]
> The default inside .env.example are localhost ready, meaning you can just copy and launch. for docker specific setup, some overrides or provided inside the [compose](./docker-compose.yaml) file.

| Variable       | Description                  | Default     |
| -------------- | ---------------------------- | ----------- |
| `DATABASE_URL` | PostgreSQL connection string | Required    |
| `HOST`         | Server bind address          | `127.0.0.1` |
| `PORT`         | Server port                  | `8080`      |
| `RUST_LOG`     | Logging level                | `info`      |
| `CACHE_ENABLED` | Enable API dataset caches (`true` or `false`) | `false` |

### Dataset caching

`CACHE_ENABLED` is the only environment switch. It defaults to `false` so local
page reloads always read current data. Helm deployments explicitly enable it with
`backend.config.cacheEnabled: true`.

Freshness policies live in `crates/osl_api/src/cache/policy.rs`. RIS distribution
is cached for one hour, including both categories and athlete/competition details.
The cache stores the serialized JSON, avoiding repeated queries and serialization.
Formula endpoints are inexpensive code-defined constants and are not cached.

Each API process owns one bounded snapshot per dataset. The first request loads
it; fresh requests reuse it; the first request after expiry refreshes it. Concurrent
requests wait for that refresh and reuse its successful result. Expiry starts when
the load succeeds. Errors and cancelled loads do not populate the cache, and
expired data is not served on failure. A waiting request can retry after a failed
refresh; there is no background refresh or error cache.

Imports do not actively invalidate snapshots. Updates and deletions can take up
to an hour to appear on the next page load. Existing browser pages retain their
loaded data until reloaded. Replicas have independent snapshots and expiry times.
Restarting an API process clears its cache; for an immediate production refresh,
all replicas must be restarted through the normal deployment workflow.

To test caching locally, run `CACHE_ENABLED=true cargo run -p osl_api`. Restart
the process to force a fresh snapshot, or return to `CACHE_ENABLED=false` for
uncached development. Changing the setting requires a restart.

To cache another dataset, add its policy and typed slot to `AppCaches`, then wrap
its loader with `get_or_try_init`. Prefer `Bytes` or `Arc<T>` values to avoid large
clones. This cache is for fixed shared datasets, not unbounded per-user/query keys.
No additional environment variables or external cache services are needed.

Cache hits and successful refresh durations are logged at debug level under
`osl_api::cache`; refresh failures are logged at warn level.

## Writing data

The API is read-only and needs no authentication. Competition data comes from
the files in `data/competitions/{federation}/{year}/{slug}/`, loaded with the `import`
binary. Each competition is a `competition.toml` describing the competition and an
`entries.csv` holding one row per athlete. Correcting results means editing a
file and importing it again.

## API Documentation

Swagger UI available at `http://localhost:8080/swagger-ui/` when running localhost, or docker.

## Database Migrations

Migrations are in `crates/storage/migrations/` and run automatically on web server startup. For manual control:

```sh
# Create new migration
sqlx migrate add <name>

# Run migrations
sqlx migrate run --database-url "postgresql://..."

# Revert last migration
sqlx migrate revert --database-url "postgresql://..."
```
