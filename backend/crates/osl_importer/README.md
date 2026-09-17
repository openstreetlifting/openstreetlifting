# Importer

The importer validates competition files, loads them into PostgreSQL, and manages athlete name removal.

## Run

From `backend`, run commands through Cargo:

```sh
cargo run -p osl_importer --bin import -- --help
cargo run -p osl_importer --bin import -- competitions --dry-run
```

The help text and examples below use `osl-import` for the executable. In a source checkout, replace it with `cargo run -p osl_importer --bin import --`.

| Command | Purpose |
| --- | --- |
| `competitions [PATH…]` | Import competition directories or trees |
| `instagram [FILE]` | Synchronize Instagram handles |
| `redact --name NAME` | Replace an athlete's name and remove their handle |
| `privacy` | Check competition files for suppressed names |
| `fmt [PATH…]` | Format competition files |
| `recompute-ris` | Recalculate stored RIS scores |

Use `<command> --help` for options and defaults. Global options, including `--database-url`, `--privacy-file`, and `--verbose`, work before or after the command.

## Competition imports

```sh
osl-import competitions data/competitions --dry-run
osl-import competitions data/competitions
```

Paths can name individual competitions or directory trees. Overlapping paths are deduplicated. The importer validates every competition before writing to the database; missing paths, empty trees, and invalid files stop the import.

Each competition is then imported in its own transaction. A database failure can leave earlier competitions imported. Fix the error and run the command again.

To remove database competitions absent from the supplied files, include the complete dataset and pass `--prune`:

```sh
osl-import competitions data/competitions --prune
```

Pruning also removes orphaned athletes and federations. It runs only after all imports succeed. Supplying a single competition with `--prune` would remove the others.

## Dry runs

`--dry-run` prevents writes, with checks appropriate to each command:

| Command | Checks |
| --- | --- |
| `competitions` | File format, data validity, and suppression records; no database connection |
| `instagram` | CSV format; also checks athlete matches when `DATABASE_URL` is set |
| `redact` | Identity match and planned file changes |
| `fmt` | Files that need formatting |
| `recompute-ris` | Number of eligible stored scores; requires a database connection |

`competitions --dry-run --prune` validates the files but does not calculate database deletions. `fmt --check` also leaves files unchanged and exits with an error when formatting is needed, making it suitable for CI.

## Configuration

The CLI loads `.env` from the working directory or its parents.

- `DATABASE_URL` supplies the PostgreSQL connection. Imports and RIS recomputation require it.
- `OSL_PRIVACY_KEY` verifies suppression records. A nonempty privacy list requires its existing key.
- `RUST_LOG` overrides the default log filter. `--verbose` enables debug logging when no filter is set.

For fork CI without the privacy key:

```sh
osl-import competitions --dry-run --skip-privacy-check
```

This checks public files without verifying suppressed names. It cannot write to the database. Trusted CI and deployment must check the files with the key.

## Athlete data

See [athlete data](../../data/athletes/README.md) for Instagram matching, key setup, and the name-removal procedure. Use the [staging fixtures](../../data/staging/README.md) to test redaction on a disposable copy.

## Breaking changes

Replace `canonical PATH` and `bulk-import --directory PATH` with `competitions PATH`. Pass multiple paths as positional arguments. Use `--dry-run` in place of `--validate-only`.

`competitions --prune` applies deletions after a successful import; it takes no `--yes` flag. Supply the complete dataset. A dry run validates files without importing or pruning.

Deploy the chart and importer image together. Staging pins the chart to the application commit used to build its importer; production pins it to the release tag.
