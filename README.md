# OpenStreetlifting

<img src="./frontend/static/logo_width.svg" alt="OpenStreetlifting">

OpenStreetlifting is an **open**, **collaborative** project building a **permanent** and **traceable** archive of all Streetlifting data, freely accessible to everyone.

![CI Backend](https://github.com/openstreetlifting/openstreetlifting/actions/workflows/ci-backend.yaml/badge.svg)
![CI Frontend](https://github.com/openstreetlifting/openstreetlifting/actions/workflows/ci-frontend.yaml/badge.svg)
[![Release](https://img.shields.io/github/v/release/openstreetlifting/openstreetlifting)](https://openstreetlifting.org)

If you want to contribute data, or want to learn more about the project, please read the [book](https://docs.openstreetlifting.org)

AI has read-write permissions, read more at [0rwx.com](https://0rwx.com/)
![./images/AI-W-dark.png]

## Run locally

If you have Docker, [rustup](https://rustup.rs), Node and pnpm 12.

```sh
./launch_local.sh
```

## Import data

The database is initially empty. You can import all competitions

```sh
cd backend && cargo run -p osl_importer --bin import -- competitions data/competitions
```

## Contribute

- [GitHub issues](https://github.com/openstreetlifting/openstreetlifting/issues)

To contribute competition data, follow the [data contribution guide](https://docs.openstreetlifting.org/CONTRIBUTING_DATA.html).

## Corrections

Report data errors or missing competitions in an issue or pull request. Please Describe the problem and link to a source. You can also email me at [contact@openstreetlifting.org](mailto:contact@openstreetlifting.org).

## Licensing

The code is licensed under [AGPLv3](LICENSE). Data (`backend/data/`) is licensed in the public domain under [CC0 1.0](LICENSE-DATA). Credit is appreciated but not required.

Sample attribution text:

> This page uses data from the OpenStreetlifting project, <https://openstreetlifting.org>

The [Licensing chapter](https://docs.openstreetlifting.org/LICENSING.html) covers third-party material and data contribution terms.

## Releases

Read through the [changelog](CHANGELOG.md)
