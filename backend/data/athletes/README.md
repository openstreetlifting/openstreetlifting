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
