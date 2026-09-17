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
| `fmt [PATH…]` | Format competition files |
| `recompute-ris` | Recalculate stored RIS scores |

Use `osl-import <command> --help` for options and defaults. Global options, including `--database-url`, `--privacy-file`, and `--verbose`, work before or after the command.

## Competition imports

```sh
osl-import competitions data/competitions --dry-run
osl-import competitions data/competitions
```

Paths can point to individual competition directories or directory trees. The importer processes overlapping paths once and validates all competition files before writing to the database. Missing paths, empty trees, and invalid files stop the import.

The importer then saves each competition in a separate transaction. If a database write fails, earlier imports remain. Fix the error and rerun the command.

With `--prune`, the importer deletes database competitions absent from the supplied files, athletes with no competition entries, and federations with no competitions. Pruning runs only after every import succeeds. Supply the complete dataset: passing a single competition would delete all others from the database.

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
| `fmt` | Files that need formatting |
| `recompute-ris` | Number of eligible stored scores; requires a database connection |

`competitions --dry-run --prune` validates files without calculating database deletions. For formatting checks in CI, use `fmt --check`: it leaves files unchanged and returns a nonzero exit status if any file needs formatting.

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

Replace `canonical PATH` and `bulk-import --directory PATH` with `competitions PATH`. Pass multiple paths as positional arguments. Use `--dry-run` in place of `--validate-only`.

`competitions --prune` deletes records after a successful import without a confirmation prompt or `--yes` flag. Supply the complete dataset. A dry run validates files without importing or pruning.

Deploy the chart and importer image together. Staging pins the chart to the application commit used to build its importer; production pins it to the release tag.
