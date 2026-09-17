# OpenStreetlifting

<img src="./frontend/static/logo_width.svg">

OpenStreetlifting is an **open**, **collaborative** project building a **permanent** and **traceable** archive of all Streetlifting data, freely accessible to everyone.

![CI Backend](https://github.com/openstreetlifting/openstreetlifting/actions/workflows/ci-backend.yaml/badge.svg)
![CI Frontend](https://github.com/openstreetlifting/openstreetlifting/actions/workflows/ci-frontend.yaml/badge.svg)
[![Release](https://img.shields.io/github/v/release/openstreetlifting/openstreetlifting)](https://openstreetlifting.org)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/openstreetlifting/openstreetlifting)

The [project book](https://docs.openstreetlifting.org) explains the archive, its data format, and its policies. This README covers development and contributions.

## Access the data

- Browse the [website](https://openstreetlifting.org) and download individual competition results as CSV.
- Read the [source files](backend/data/competitions), including each competition's references and revision history.
- Query the [API](https://api.openstreetlifting.org/swagger-ui/).

## Run locally

Install Docker, Rust, Node.js 24, pnpm 12, and [sqlx-cli](https://crates.io/crates/sqlx-cli). From the repository root, prepare the database and frontend:

```sh
cp backend/.env.example backend/.env
docker compose up -d --wait postgres
cd backend
sqlx migrate run --source crates/osl_db/migrations
cd ../frontend
pnpm install --frozen-lockfile
cd ..
```

Import the competition files from `backend`. If the privacy list contains records, set `OSL_PRIVACY_KEY` to its existing key; see [athlete data](backend/data/athletes/README.md).

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

To add competition results, follow the [data contribution guide](https://docs.openstreetlifting.org/CONTRIBUTING_DATA.html). Each competition has a directory under `backend/data/competitions/<federation>/<year>/<competition-slug>/`, with a `competition.toml` and an `entries.csv`. Include the sources so others can check the results.

## Corrections and name removal

Report incorrect results, athlete names, or missing competitions in an issue or pull request. Include the competition slug, the error, and a source where available. You can also email [contact@openstreetlifting.org](mailto:contact@openstreetlifting.org).

Athletes can request removal of their name by emailing [contact@openstreetlifting.org](mailto:contact@openstreetlifting.org). Their results remain in the archive. Please keep these requests out of public issues. The [Personal Data chapter](https://docs.openstreetlifting.org/PERSONAL_DATA.html) explains the process and its limits.

## Licensing

The code is licensed under [AGPLv3](LICENSE). Data under `backend/data/` is dedicated to the public domain under [CC0 1.0](LICENSE-DATA). Credit is appreciated but not required.

The [Licensing chapter](https://docs.openstreetlifting.org/LICENSING.html) covers third-party material and the terms for contributing data.

## Releases

See the [changelog](CHANGELOG.md) for website and API releases.
