# OpenStreetlifting

<img src="./frontend/static/logo_width.svg" alt="OpenStreetlifting">

OpenStreetlifting is an **open**, **collaborative** project building a **permanent** and **traceable** archive of all Streetlifting data, freely accessible to everyone.

![CI Backend](https://github.com/openstreetlifting/openstreetlifting/actions/workflows/ci-backend.yaml/badge.svg)
![CI Frontend](https://github.com/openstreetlifting/openstreetlifting/actions/workflows/ci-frontend.yaml/badge.svg)
[![Release](https://img.shields.io/github/v/release/openstreetlifting/openstreetlifting)](https://openstreetlifting.org)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/openstreetlifting/openstreetlifting)

The [project book](https://docs.openstreetlifting.org) explains how the archive works and how to use and contribute to it. This README introduces the project and helps you get started.

## Access the data

- Browse competition results on the [website](https://openstreetlifting.org). CSV downloads for individual competitions are **coming soon**.
- Query the [API](https://api.openstreetlifting.org/swagger-ui/).

## Run locally

Install Docker, Rust, Node.js 24, pnpm 12, and [sqlx-cli](https://crates.io/crates/sqlx-cli). Clone the repository, then prepare the database and frontend from the repository root:

```sh
cp backend/.env.example backend/.env
docker compose up -d --wait postgres
cd backend
sqlx migrate run --source crates/osl_db/migrations
cd ../frontend
pnpm install --frozen-lockfile
cd ..
```

Import the competition files and start both servers:

```sh
cd backend
cargo run -p osl_importer --bin import -- competitions
cd ..
./launch_local.sh
```

The frontend runs at <http://localhost:5173>, the API at <http://localhost:8080>, and Swagger UI at <http://localhost:8080/swagger-ui/>.

See the [backend](backend/README.md), [frontend](frontend/README.md), and [importer](backend/crates/osl_importer/README.md) READMEs for configuration and checks.

## Contribute

To contribute code, fork the repository, create a branch from `main`, and open a pull request. [GitHub issues](https://github.com/openstreetlifting/openstreetlifting/issues) track code and data work.

To contribute competition data, follow the [data contribution guide](https://docs.openstreetlifting.org/CONTRIBUTING_DATA.html).

## Corrections

Report data errors or missing competitions in an issue or pull request. Include the competition slug if one exists, describe the problem, and link to a source where available. You can also email [contact@openstreetlifting.org](mailto:contact@openstreetlifting.org).

## Licensing

The code is licensed under [AGPLv3](LICENSE). Data under `backend/data/` is dedicated to the public domain under [CC0 1.0](LICENSE-DATA). Credit is appreciated but not required.

The [Licensing chapter](https://docs.openstreetlifting.org/LICENSING.html) covers third-party material and the terms for contributing data.

## Releases

See the [changelog](CHANGELOG.md) for website and API releases.
