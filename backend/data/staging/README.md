# Staging fixtures

This directory contains fictional competition results for testing imports and
name removal. Production imports only `data/competitions`.

For a source checkout, replace `osl-import` below with
`cargo run -p osl_importer --bin import --`.

From `backend`, create a temporary copy of the data. Set `OSL_PRIVACY_KEY` to
the key used by the copied privacy list, then redact the test athlete:

```sh
fixture_root=$(mktemp -d)
cp -R data "$fixture_root/data"
osl-import --privacy-file "$fixture_root/data/athletes/privacy.csv" redact \
  --name "Fixture Delta" \
  --directory "$fixture_root/data/staging/competitions" \
  --instagram-file "$fixture_root/data/athletes/instagram.csv"
```

The command prints the replacement ID. Run it again to verify that the ID stays
the same.

Set `DATABASE_URL` to a test database and import the copy:

```sh
osl-import --privacy-file "$fixture_root/data/athletes/privacy.csv" competitions \
  "$fixture_root/data/competitions" \
  "$fixture_root/data/staging/competitions" \
  --prune
```

Keep the generated files in the temporary directory. The test athlete's
suppression record must not be added to the archive's privacy list.
