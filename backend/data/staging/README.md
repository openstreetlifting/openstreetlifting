# Staging fixtures

This directory contains fictional competition results for testing imports and name removal.

Run these commands from the `backend` directory.

## Test name removal

Set `OSL_PRIVACY_KEY` to the key used by `data/athletes/privacy.csv`. If the list has no records, generate a test key as described in the [athlete data guide](../athletes/README.md#key-setup). Then copy the data to a temporary directory and remove the test athlete's name:

```sh
fixture_root=$(mktemp -d)
cp -R data "$fixture_root/data"
cargo run -p osl_importer --bin import -- \
  --privacy-file "$fixture_root/data/athletes/privacy.csv" redact \
  --name "Fixture Delta" \
  --directory "$fixture_root/data/staging/competitions" \
  --instagram-file "$fixture_root/data/athletes/instagram.csv"
```

The command prints the replacement ID. Run it again to check that it reuses the same ID.

## Test the import

Set `DATABASE_URL` to a disposable test database and import the copy. The `--prune` flag deletes database competitions absent from these files, athletes with no competition entries, and federations with no competitions.

```sh
cargo run -p osl_importer --bin import -- \
  --privacy-file "$fixture_root/data/athletes/privacy.csv" competitions \
  "$fixture_root/data/competitions" \
  "$fixture_root/data/staging/competitions" \
  --prune
```

Keep the generated files in the temporary directory, including the test athlete's suppression record. Leave the archive's privacy list unchanged.
