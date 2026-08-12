# Openstreetlifting

Openstreetlifting is an **open**, **collaborative** project building a **permanent** and **traceable** archive of all Streetlifting data, freely accessible to everyone.

![CI Backend](https://github.com/openstreetlifting/openstreetlifting/actions/workflows/ci-backend.yaml/badge.svg)
![CI Frontend](https://github.com/openstreetlifting/openstreetlifting/actions/workflows/ci-frontend.yaml/badge.svg)
[![Release](https://img.shields.io/github/v/release/openstreetlifting/openstreetlifting)](https://openstreetlifting.org)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/openstreetlifting/openstreetlifting)

This Readme only cover developer documentation, if you want to know about the why and the how, please consider the [book](https://docs.openstreetlifting.org)

## Run locally

Requires docker, rust with `sqlx-cli`, and pnpm.

```sh
cp backend/.env.example backend/.env
docker compose up -d --wait postgres
cd backend
sqlx migrate run --source crates/osl_db/migrations
cargo run -p osl_importer --bin import -- bulk-import
cd ../frontend && pnpm install
```

```sh
./launch_local.sh
```

API on <http://localhost:8080>, Swagger at `/swagger-ui/`, frontend on <http://localhost:5173>.

Reset the database, then rerun the migration and import:

```sh
cd backend && sqlx database drop -y && sqlx database create
```

Everything in containers:

```sh
docker compose up -d --build
```

## Contributing

Contributions are welcome, whether you are fixing a bug, improving the codebase, or adding missing competition data.
For code contributions, fork the repository, create a branch from main, and open a pull request. Commits must follow the Conventional Commits specification, as releases are automated via google release-please.
For data contributions, the entry point is the [canonical format](https://docs.openstreetlifting.org). If you have results from a competition that is not yet in the archive, create a canonical JSON file under backend/imports/{competition-slug}/ following the existing structure and open a pull request.

## Importing in the cluster

The canonical files are the data, so every deploy makes the database match them.
The chart runs an import Job as a `post-upgrade` hook, which Argo CD runs as a
PostSync once the release is healthy, so the import lands behind the backend that
applies migrations. The importer image carries the whole `backend/imports` tree,
which is why a merged data pull request reaches production on its own. Deletions
are included: a competition no file claims is removed, along with the athletes
that leaves without a result.

To have a release import without deleting, drop `--yes` from `importer.args` and
it only reports what it would remove.

To run an import outside a deploy, render the on demand Job:

```sh
helm template charts -s templates/importer-job.yaml \
  --set importer.job.enabled=true \
  | kubectl create -f -
```

Any other importer command works the same way, for example a single file:

```sh
helm template charts -s templates/importer-job.yaml \
  --set importer.job.enabled=true \
  --set-json 'importer.args=["canonical","imports/fnsl-elite-2026/fnsl-elite-2026.json"]' \
  | kubectl create -f -
```

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
