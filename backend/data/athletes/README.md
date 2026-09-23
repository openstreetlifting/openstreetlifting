# Athlete data

These files link athletes to Instagram accounts and prevent removed names from being republished. Run the examples from `backend`. In a source checkout, replace `osl-import` with `cargo run -p osl_importer --bin import --`.

## Instagram accounts

`instagram.csv` maps athlete names to handles. Sort it by name and omit the leading `@` from handles. Add an account only when it clearly belongs to the athlete.

```csv
Name,Sex,Country,Disambiguation,Instagram
Adrien Pelfresne,,,,dirdros
```

Name matching ignores accents and capitalization. Leave `Sex`, `Country`, and `Disambiguation` empty unless the name matches more than one athlete. Fill in only the fields needed to distinguish them, using the values in `entries.csv`:

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

With `DATABASE_URL` set, the dry run also checks that each row matches exactly one athlete. Without it, the command checks only the CSV. An unknown or ambiguous athlete stops synchronization before any writes. Removing a row removes the stored handle on the next synchronization.

## Suppression records

`privacy.csv` records which names must stay out of published results. Results remain under numbered replacement names.

```csv
Hash,Sex,Country,Disambiguation,RedactedId,KeyCheck
```

The `redact` command maintains this file. `Hash` is an HMAC-SHA256 fingerprint of the normalized name, keyed by `OSL_PRIVACY_KEY`. `KeyCheck` verifies that later commands use the same key. Neither field stores the original name or key. Suppression follows the name, `Sex`, and `Disambiguation`, even if country is corrected or missing. `Country` records the selection made by `redact`; it does not limit suppression.

The command assigns each athlete the next available `RedactedId` in the privacy list. No ID ranges are reserved for particular environments. Repeated requests reuse that number, including when a later competition restores the original name. Keep each suppression record and its ID while the corresponding results remain published.

### Key setup

If the privacy list already contains records, use its existing key. For a new list, generate a key once with `openssl rand -hex 32`. Keep it outside Git and back it up securely. Set `OSL_PRIVACY_KEY` locally and use the same value in the GitHub Actions secret `OSL_PRIVACY_KEY` and the Kubernetes Secret `osl-privacy`, field `key`.

When migrating from `OSL_PRIVACY_SALT`, rename the setting and keep its value. A new key cannot match existing fingerprints. Lists with records require the original key; older experimental lists without `KeyCheck` are rejected.

### Name removal

Preview the changes, then apply them:

```sh
osl-import redact --name "Some Athlete" --dry-run
osl-import redact --name "Some Athlete"
```

The command replaces one athlete's name across competition entries, clears their native name, removes their Instagram entries, and records the suppression. Use `--sex`, `--country`, or `--disambiguation` to resolve an ambiguous match.

Review and commit the changed files. Deploy them and import the complete dataset with `competitions --prune` to remove the old database profile. The [backend cache notes](../../README.md#caching) explain when changes reach cached responses.

Run one redaction at a time per checkout. The command prepares all file changes, saves the suppression record, then replaces each results or Instagram file atomically. If interrupted, some files may still contain the original name. Rerun the same command to finish; the saved record blocks imports of those entries.

### Verification

```sh
osl-import privacy
```

Imports run the same check and reject competitions that restore a removed name. Rerun `redact` to replace that name with the athlete's existing numbered identity.

A fork's CI can check public files without the privacy key:

```sh
osl-import competitions --dry-run --skip-privacy-check
```

This command skips the check for removed names and cannot write to the database. Trusted CI and deployments must verify suppression records with the key before publication. Imports validate all files before writing, then save the batch and prune in one transaction. Any failure rolls back both.

### Requests

Keep requests in email. Public issues and pull request descriptions must not name the requester. Rewriting these files does not remove names from earlier Git commits, third-party copies, or caches. The [Personal Data chapter](https://docs.openstreetlifting.org/PERSONAL_DATA.html) explains the procedure and its limits.

## Country-independent identity migration

Identity now uses normalized name, sex, and disambiguation. The database migration
preserves existing athlete IDs, URLs, results, and social links. Entry countries
remain as recorded; a matching host country alone does not justify clearing them.

Seven name groups had separate profiles under different countries. Their entries
and database rows receive the same numbers to preserve those profiles pending
source review. These numbers do not establish that they are different people.

| Name | Disambiguation 1 | Disambiguation 2 |
| --- | --- | --- |
| Harry Twister | BA | SE |
| Denilson Monteiro | DE | FR |
| Giuseppe Cicero | IT | SM |
| Lorenzo Giorgetti | IT | SM |
| Jacopo Bartoli | IT | SM |
| Ilaria Valentini | IT | SM |
| Tony Nguyen | FR | US |

Review each group against its sources before correcting countries or merging
profiles. If the database contains other country-based identity collisions, the
migration fails instead of merging them. Resolve those collisions explicitly
before retrying.
