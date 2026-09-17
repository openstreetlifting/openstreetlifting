# Staging fixtures

Invented results for testing imports. These competitions are not part of the
archive. Importing this directory is a deployment choice; the importer has no
staging-specific identity rules or reserved IDs.

To exercise redaction, work on a disposable copy of the dataset and use the
normal command. From `backend`, with `OSL_PRIVACY_KEY` set to the key for the
copied suppression list:

```sh
fixture_root=$(mktemp -d)
cp -R data "$fixture_root/data"
osl-import --privacy-file "$fixture_root/data/athletes/privacy.csv" redact \
  --name "Fixture Delta" \
  --directory "$fixture_root/data/staging/competitions" \
  --instagram-file "$fixture_root/data/athletes/instagram.csv"
```

The allocated replacement ID is printed by the command. Import the copy into
a disposable or staging database:

```sh
osl-import --privacy-file "$fixture_root/data/athletes/privacy.csv" bulk-import \
  --directory "$fixture_root/data/competitions" \
  --directory "$fixture_root/data/staging/competitions" \
  --prune --yes
```

Repeat the redaction command to check that it keeps the same ID. Do not commit
the generated fixture suppression record to the archive's privacy list.
Production imports only `data/competitions`.
