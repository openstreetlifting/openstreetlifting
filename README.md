# OpenStreetlifting

<img src="./frontend/static/logo_width.svg">

OpenStreetlifting is an **open**, **collaborative** project building a **permanent** and **traceable** archive of all Streetlifting data, freely accessible to everyone.

![CI Backend](https://github.com/openstreetlifting/openstreetlifting/actions/workflows/ci-backend.yaml/badge.svg)
![CI Frontend](https://github.com/openstreetlifting/openstreetlifting/actions/workflows/ci-frontend.yaml/badge.svg)
[![Release](https://img.shields.io/github/v/release/openstreetlifting/openstreetlifting)](https://openstreetlifting.org)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/openstreetlifting/openstreetlifting)

This Readme only cover developer documentation, if you want to know about the why and the how, please consider the [book](https://docs.openstreetlifting.org)

## Get data

There are multiple way to get data from the OpenStreetlifting project

1. Consult the [website](https://openstreetlifting.org) where you can download csv for individual competitions
2. Download the full collection (COMING SOON)
3. Use the API [swagger](https://api.openstreetlifting.org/swagger-ui/)

## Run locally

The easiest way to run locally is through the `launch_local.sh` script, which require docker for postgres, rust and pnpm.
To run the database migration, you will need to install [sqlx-cli](https://crates.io/crates/sqlx-cli).

First time launch, boot up the database, run the migration and import some datas.

```sh
cp backend/.env.example backend/.env
docker compose up -d --wait postgres
cd backend
sqlx migrate run --source crates/osl_db/migrations
cargo run -p osl_importer --bin import -- bulk-import
cd ../frontend && pnpm install
```

Then, once the environment is ready, you can simply use the script.

```sh
./launch_local.sh
```

API on <http://localhost:8080>, Swagger at `/swagger-ui/`, frontend on <http://localhost:5173>.

## Contributing

Contributions are welcome, whether you are fixing a bug, improving the codebase, or adding missing competition data.
For code contributions, fork the repository, create a branch from main, and open a pull request.

For data contributions, the entry point is the [canonical format](https://docs.openstreetlifting.org). If you have results from a competition that is not yet in the archive, add a `competition.toml` and an `entries.csv` under backend/data/competitions/{federation}/{year}/{competition-slug}/ following the existing structure and open a pull request.

Please note that I'm using Github issues to track identified work, whereas it is code or data. This can be a good starting point if you want to help me!

## Data Correction

All competition data in this archive is versioned and traceable. If you spot an error, a wrong lift result, an incorrect athlete name, a missing competition, you can report or fix it directly (see the Contributing section above).

To report an error, open an issue and include the competition slug, the athlete name, and a description of what is wrong. A source reference (official result sheet, video, federation website) is appreciated. You can also contact me at [contact@openstreetlifting.org](mailto:contact@openstreetlifting.org) I will do my best to correct the issue quickly

## Licensing

### Code

OpenStreetlifting code is free software licensed under AGPLv3.
See [LICENSE](./LICENSE) file.

### Data

OpenStreetlifting data is licensed under Creative Commons Attribution 4.0 (CC BY 4.0).
See [LICENSE-DATA](./LICENSE-DATA) file.

> [!important]
> If you use any of the data, you need to credit the project:
> Data is coming from the OpenStreetlifting project [openstreetlifting.org](https://openstreetlifting.org)

## versions

You can look at the [changelog](./CHANGELOG.md) to list all the versions of the website, and the api.
