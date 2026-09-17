# Athlete data

These files link athletes to Instagram accounts and record requests for name removal. Run the examples from `backend`. For a source checkout, replace `osl-import` with `cargo run -p osl_importer --bin import --`.

## Instagram accounts

`instagram.csv` maps athlete names to handles. Sort it by name and omit the leading `@` from handles. Add an account only when it clearly belongs to the athlete.

```csv
Name,Sex,Country,Disambiguation,Instagram
Adrien Pelfresne,,,,dirdros
```

Name matching ignores accents and capitalization. Leave the optional identity fields empty unless the name matches more than one athlete. Add only the fields needed to distinguish them, using the values in `entries.csv`:

```csv
Name,Sex,Country,Disambiguation,Instagram
Tony Nguyen,,FR,,tony_fr
Tony Nguyen,,US,,tony_us
```

Validate, then synchronize:

```sh
osl-import instagram --dry-run
osl-import instagram
```

With `DATABASE_URL` set, the dry run also checks that each row matches one athlete. Without it, only the file is checked. An ambiguous match stops synchronization before any writes. Removing a row removes the stored handle on the next synchronization.

## Suppression records

`privacy.csv` records name-removal requests. Results remain under numbered replacement names.

```csv
Hash,Sex,Country,Disambiguation,RedactedId,KeyCheck
```

The redaction command maintains this file. `Hash` is an HMAC-SHA256 fingerprint of the normalized name, keyed by `OSL_PRIVACY_KEY`. `KeyCheck` verifies that later commands use the same key. Neither field contains the name or the key. Sex, country, and disambiguation distinguish athletes with the same name.

`RedactedId` comes from the dataset, with no reserved environment ranges. Repeated requests reuse the number, including when a later competition restores the original name. Keep suppression records and their IDs while the corresponding results remain published.

### Key setup

Generate the key once with `openssl rand -hex 32`. Keep it outside Git and back it up securely. Use the same value locally, in the GitHub Actions secret `OSL_PRIVACY_KEY`, and in the Kubernetes Secret `osl-privacy`, field `key`.

When migrating from `OSL_PRIVACY_SALT`, rename the setting and retain its value. A new key cannot match existing fingerprints. A nonempty list requires the correct key; older experimental lists without `KeyCheck` are rejected.

### Name removal

```sh
osl-import redact --name "Some Athlete" --dry-run
osl-import redact --name "Some Athlete"
```

The command replaces one athlete's name across competition entries, clears their native name, removes their Instagram entries, and records the suppression. Use `--sex`, `--country`, or `--disambiguation` to resolve an ambiguous match.

Review and commit the changed files. Deploy them and import the complete dataset with `competitions --prune` to remove the old database profile. See the [backend cache notes](../../README.md#caching) for when changes reach cached responses.

Run one redaction at a time per checkout. The command prepares all files, saves the suppression record, then replaces each result or social file atomically. An interruption can leave only some files replaced. Run the same command again to finish; the saved record blocks imports of remaining original-name entries.

### Verification

```sh
osl-import privacy
```

Imports perform the same suppression check. They reject competitions that restore a removed name; they do not rewrite those files automatically. Run redaction again to apply the existing replacement identity.

Fork CI can check public files without the secret:

```sh
osl-import competitions --dry-run --skip-privacy-check
```

This bypass cannot write to the database or approve publication. Trusted CI and deployment must verify suppression records with the key. Imports validate all files before writing, then commit each competition separately. Any import failure prevents pruning.

### Requests

Keep requests in email. Public issues and pull request descriptions must not name the requester. Rewriting these files does not remove names from earlier Git commits, third-party copies, or caches. The [Personal Data chapter](https://docs.openstreetlifting.org/PERSONAL_DATA.html) explains the procedure and its limits.
