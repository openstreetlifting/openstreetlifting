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
