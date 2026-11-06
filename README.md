# OpenStreetlifting

<img src="./frontend/static/logo_width.svg" alt="OpenStreetlifting">

OpenStreetlifting is an **open**, **collaborative** project building a **permanent** and **traceable** archive of all Streetlifting data, freely accessible to everyone.

![CI Backend](https://github.com/openstreetlifting/openstreetlifting/actions/workflows/ci-backend.yaml/badge.svg)
![CI Frontend](https://github.com/openstreetlifting/openstreetlifting/actions/workflows/ci-frontend.yaml/badge.svg)
[![Release](https://img.shields.io/github/v/release/openstreetlifting/openstreetlifting)](https://openstreetlifting.org)

To contribute data, Licensing, or just to learn more about this project, please read the [book](https://docs.openstreetlifting.org)

## Run locally

Install Docker, Rust, Node.js 24, pnpm 12, and [sqlx-cli](https://crates.io/crates/sqlx-cli). Clone the repository, then prepare the database and frontend from the repository root:

```sh
cp backend/.env.example backend/.env
docker compose up -d --wait postgres
cd backend
cargo sqlx migrate run --source crates/osl_db/migrations
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

- frontend: `localhost:5173`
- backend: `localhost:8080`

## Contribute

Fork the repository, create a branch from `main`, and open a pull request.
You can read through [GitHub issues](https://github.com/openstreetlifting/openstreetlifting/issues) to find work to do.

To contribute competition data, follow the [data contribution guide](https://docs.openstreetlifting.org/CONTRIBUTING_DATA.html).

## Corrections

Report data errors or missing competitions in an issue or pull request. Describe the problem and link to a source. You can also email me at [contact@openstreetlifting.org](mailto:contact@openstreetlifting.org).

## Licensing

The code is licensed under [AGPLv3](LICENSE). Data (`backend/data/`) is licensed in the public domain under [CC0 1.0](LICENSE-DATA). Credit is appreciated but not required.

Sample attribution text:

> This page uses data from the OpenStreetlifting project, <https://openstreetlifting.org>

The [Licensing chapter](https://docs.openstreetlifting.org/LICENSING.html) covers third-party material and data contribution terms.

## Releases

Read through the [changelog](CHANGELOG.md)
