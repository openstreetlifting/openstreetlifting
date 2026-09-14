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

Athletes who asked to be taken off the site. Their results stay in the archive
under a stand-in name, so the rankings and the history do not move.

```csv
Hash,Sex,Country,Disambiguation,RedactedId
3f9a1c...,,,,1
```

`Hash` is the athlete's folded name under HMAC-SHA256, keyed by
`OSL_PRIVACY_SALT`. The name itself is never written down: a public list of
everyone who asked to be forgotten would publish what it exists to remove, and
would be easier to search than the results it took down.

`Sex`, `Country` and `Disambiguation` narrow the same way they do in
`instagram.csv`, and stay in the clear because they name nobody on their own.

`RedactedId` is the number in `Redacted Athlete #1`, handed out once so the
athlete page keeps its URL across re-imports. Numbers from 9000 up belong to
the staging fixtures in `../staging/`.

Redact with:

```sh
osl-import redact --name "Some Athlete" --dry-run
osl-import redact --name "Some Athlete"
```

It rewrites every `entries.csv` that names them, drops their `instagram.csv`
line, and records the redaction here. Nothing is written unless the name lands
on exactly one athlete; narrow it with `--sex`, `--country` or
`--disambiguation` when it does not.

```sh
osl-import privacy
```

checks that no canonical file names anyone on the list, which is what stops the
next competition they enter from putting the name back. Without the salt it
warns and skips, so it cannot run on a fork; `--require-salt` makes that a
failure, and the deploy passes it.

Requests arrive by email and stay there. Never open an issue or a pull request
naming the person who asked.
