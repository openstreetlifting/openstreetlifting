# Openstreetlifting

Openstreetlifting is an **open**, **collaborative** project building a **permanent** and **traceable** archive of all Streetlifting data, freely accessible to everyone.

![CI Backend](https://github.com/openstreetlifting/openstreetlifting/actions/workflows/ci-backend.yaml/badge.svg)
![CI Frontend](https://github.com/openstreetlifting/openstreetlifting/actions/workflows/ci-frontend.yaml/badge.svg)
[![Release](https://img.shields.io/github/v/release/openstreetlifting/openstreetlifting)](https://openstreetlifting.org)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/openstreetlifting/openstreetlifting)

This Readme only cover developer documentation, if you want to know about the why and the how, please consider the [book](https://docs.openstreetlifting.org)

## Run locally

The project architecture is a monorepo, containing the Rust backend (ETL Pipeline, API, DB, Domain), and the Svelte Kit Frontend.
You can either run the project by using the [docker compose](./docker-compose.yaml)

```sh
docker compose up -d --build
```

Or by running each service individually

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
