# Athlete data

## instagram.csv

```csv
Name,Sex,Country,Disambiguation,Instagram
Adrien Pelfresne,,,,dirdros
```

Sorted by name, no leading `@`. Names match the same way the importer matches
athletes, so accents and capitalisation don't matter.

`Sex`, `Country` and `Disambiguation` are left blank, because a name is enough
to find nearly everyone. They are there for the names that are not: an athlete
is identified by their name together with those three, so two people who share
a name are two athletes a name on its own cannot tell apart. Fill in as few of
them as it takes to leave one, spelled the way `entries.csv` spells them:

```csv
Name,Sex,Country,Disambiguation,Instagram
Tony Nguyen,,FR,,tony_fr
Tony Nguyen,,US,,tony_us
```

An import that cannot narrow a name down to one athlete names the candidates
and stops without writing anything, so the file is never half applied.

```sh
osl-import instagram
osl-import instagram --validate-only
```

`--validate-only` checks the file against the database when `DATABASE_URL` is
set, which is how CI catches a name that has stopped naming one person. Without
it, only the file itself is checked.

The file is the truth, so deleting a line takes the handle off the site.

Only add a handle when the account is obviously the athlete's.

## privacy.csv

Suppression records for athletes whose names have been removed. Their results
remain under a numbered replacement name.

```csv
Hash,Sex,Country,Disambiguation,RedactedId,KeyCheck
```

The redaction command writes this file. `Hash` is an HMAC-SHA256 fingerprint of
the normalised name, using the secret `OSL_PRIVACY_KEY`. `KeyCheck` verifies that
a later command has the same key. Neither field contains the name or the key.
Sex, country and disambiguation distinguish athletes who share a name.

Generate the key once with `openssl rand -hex 32`. Keep it outside Git and back
it up securely. Use the same value locally, in the GitHub Actions secret
`OSL_PRIVACY_KEY`, and in the Kubernetes Secret `osl-privacy`, field `key`.
If you previously configured `OSL_PRIVACY_SALT`, rename the setting and keep
its value. Existing fingerprints cannot be matched using a new key.

`RedactedId` is allocated from the dataset without environment-specific ranges.
Repeating a request, including after a new competition restores the name,
reuses the original number. No suppression records should be deleted or IDs
reassigned while the corresponding results remain published.

```sh
osl-import redact --name "Some Athlete" --dry-run
osl-import redact --name "Some Athlete"
```

The command finds one identity, rewrites its competition entries, clears its
native name and removes its Instagram entries. Narrow an ambiguous name with
`--sex`, `--country` or `--disambiguation`. Review and commit the resulting files,
then deploy and run the full import with `--prune --yes` to remove the old
profile from the database.

Files are prepared before any replacement. The suppression record is saved
first, then each result or social file is replaced atomically. This is not a
transaction across all files: if interrupted, run the same command again.
The saved record prevents import of any remaining original-name entries.
Run one redaction command at a time against a checkout.

```sh
osl-import privacy
```

This checks competition files against the suppression list. Imports perform
the same check and reject competitions that would restore a removed name;
they do not automatically rewrite those files. A nonempty list requires the
correct key. Old experimental CSVs without `KeyCheck` are rejected rather than
accepted with an unverifiable key.

Fork CI can validate file structure without access to the secret:

```sh
osl-import bulk-import --validate-only --skip-privacy-check
```

This bypass cannot write to a database and does not approve publication.
Trusted CI and deployment must run the checks with the key. Bulk imports
commit each competition separately and skip pruning if any competition fails.

Requests arrive by email and stay there. Never open an issue or pull request
naming the person who asked. Rewriting these files does not remove names from
previous Git commits, third-party copies or caches.
