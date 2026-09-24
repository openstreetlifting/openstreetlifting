# Importer

The importer validates competition files, loads them into PostgreSQL, and manages athlete name removal.

## Run

From `backend`, run commands through Cargo:

```sh
cargo run -p osl_importer --bin import -- --help
cargo run -p osl_importer --bin import -- competitions --dry-run
```

The examples below use `osl-import`, the name shown in the CLI help. In a source checkout, replace it with `cargo run -p osl_importer --bin import --` and run from `backend`.

| Command | Purpose |
| --- | --- |
| `competitions [PATH…]` | Import competition directories or trees |
| `instagram [FILE]` | Synchronize Instagram handles |
| `redact --name NAME` | Replace an athlete's name and remove their handle |
| `privacy` | Check competition files for suppressed names |
| `prepare [PATH…]` | Fill missing totals, validate and format competition files |
| `recompute-ris` | Recalculate stored RIS scores |

Use `osl-import <command> --help` for options and defaults. Global options, including `--database-url`, `--privacy-file`, and `--verbose`, work before or after the command.

## Competition imports

```sh
osl-import prepare data/competitions
osl-import competitions data/competitions --dry-run
osl-import competitions data/competitions
```

Paths can point to individual competition directories or directory trees. The importer processes overlapping paths once and validates all competition files before writing to the database. Missing paths, empty trees, and invalid files stop the import.

Every competition needs at least one source reference. For one-day events,
`end_date` may be omitted; `venue` is optional. Mixed contests require athlete
`Sex`, `CategorySex=MX`, and an explicit `competition.scoring`. See the
[data reference](../../docs/src/DATA_REFERENCE.md) for accepted values.

The importer saves the whole batch in one transaction, including pruning. Conflicting countries for one athlete or a failed write roll back the batch. Correct country evidence across the affected competitions and import them together.

With `--prune`, the importer deletes database competitions absent from the supplied files, athletes with no competition entries, and federations with no competitions. Pruning runs after the imports, within the same transaction. Supply the complete dataset: passing a single competition would delete all others from the database.

```sh
osl-import competitions data/competitions --prune
```

## Dry runs

`--dry-run` checks the input without changing files or database records:

| Command | Checks |
| --- | --- |
| `competitions` | File format, data validity, and suppression records; no database connection |
| `instagram` | CSV format; also checks athlete matches when `DATABASE_URL` is set |
| `redact` | Identity match and planned file changes |
| `prepare` | Files that need preparation |
| `recompute-ris` | Number of eligible stored scores; requires a database connection |

`competitions --dry-run --prune` validates files without calculating database deletions. For preparation checks in CI, use `prepare --check`: it leaves files unchanged and returns a nonzero exit status if any file needs preparation.

## Configuration

The CLI loads `.env` from the working directory or its parents.

- `DATABASE_URL` supplies the PostgreSQL connection URL. Competition imports, Instagram synchronization, and RIS recomputation require it. You can also pass `--database-url`.
- `OSL_PRIVACY_KEY` creates and verifies suppression records. If the privacy list contains records, use the key that created them.
- `RUST_LOG` overrides the default log filter. `--verbose` enables debug logging when no filter is set.

To validate files in a fork's CI without the privacy key:

```sh
osl-import competitions --dry-run --skip-privacy-check
```

This checks public files but skips the check for removed names. The flag requires `--dry-run`, so it cannot write to the database. Trusted CI and deployments must verify suppression records with the key.

## Athlete data

See the [athlete data guide](../../data/athletes/README.md) for Instagram matching, key setup, and name removal. Use the [staging fixtures](../../data/staging/README.md) to test name removal on a temporary copy.

## Migrate from older commands

Replace `fmt PATH` with `prepare PATH`; preparation also fills missing totals from complete lift results and rejects inconsistent data.

Replace `canonical PATH` and `bulk-import --directory PATH` with `competitions PATH`. Pass multiple paths as positional arguments. Use `--dry-run` in place of `--validate-only`.

`competitions --prune` deletes records after a successful import without a confirmation prompt or `--yes` flag. Supply the complete dataset. A dry run validates files without importing or pruning.

Deploy the chart and importer image together. Staging pins the chart to the application commit used to build its importer; production pins it to the release tag.
