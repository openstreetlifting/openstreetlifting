# Openstreetlifting

Openstreetlifting is an **open**, **collaborative** project building a **permanent** and **traceable** archive of all Streetlifting data, freely accessible to everyone.

![CI Backend](https://github.com/openstreetlifting/openstreetlifting/actions/workflows/ci-backend.yaml/badge.svg)
![CI Frontend](https://github.com/openstreetlifting/openstreetlifting/actions/workflows/ci-frontend.yaml/badge.svg)
[![Release](https://img.shields.io/github/v/release/openstreetlifting/openstreetlifting)](https://openstreetlifting.org)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/openstreetlifting/openstreetlifting)

This Readme only cover developer documentation, if you want to know about the why and the how, please consider the [book](https://docs.openstreetlifting.org)

## Run locally

The project is a monorepo: a Rust backend (ETL pipeline, API, DB, domain) and a SvelteKit frontend.

Postgres runs in a container so it matches CI. The backend and frontend run natively, because
building Rust inside a container throws away the incremental cache on every change.

Requirements: docker, rust with `sqlx-cli`, pnpm.

```sh
cp backend/.env.example backend/.env      # defaults match the compose file
docker compose up -d --wait postgres
cd backend && sqlx migrate run --source crates/osl_db/migrations
cargo run -p osl_importer --bin import -- bulk-import   # loads backend/imports/*.json
cd ../frontend && pnpm install
```

Then start both servers:

```sh
./launch_local.sh
```

The API is on <http://localhost:8080> (Swagger at `/swagger-ui/`) and the frontend on
<http://localhost:5173>. `cargo install cargo-watch` to get backend reload.

To start over, drop the database and repeat from the migration step:

```sh
cd backend && sqlx database drop -y && sqlx database create
```

The full stack in containers is still available, and is what CI and production build:

```sh
docker compose up -d --build
```

## Contributing

Contributions are welcome, whether you are fixing a bug, improving the codebase, or adding missing competition data.
For code contributions, fork the repository, create a branch from main, and open a pull request. Commits must follow the Conventional Commits specification, as releases are automated via google release-please.
For data contributions, the entry point is the [canonical format](https://docs.openstreetlifting.org). If you have results from a competition that is not yet in the archive, create a canonical JSON file under backend/imports/{competition-slug}/ following the existing structure and open a pull request.

## Data Correction

All competition data in this archive is versioned and traceable. If you spot an error, a wrong lift result, an incorrect athlete name, a missing competition, you can report or fix it directly.
To report an error, open an issue and include the competition slug, the athlete name, and a description of what is wrong. A source reference (official result sheet, video, federation website) is appreciated.

## Licensing

### Code

All OpenStreetLifting code is free software licensed under AGPLv3.
See LICENSE file.

### Data

OpenStreetLifting data is licensed under Creative Commons Attribution 4.0 (CC BY 4.0).
<https://creativecommons.org/licenses/by/4.0/>

If you use this data, please credit:
OpenStreetLifting (openstreetlifting.org)
