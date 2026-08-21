# Athlete data

## social-instagram.csv

```csv
Name,Instagram
Adrien Pelfresne,dirdros
```

Sorted by name, no leading `@`. Names match the same way the importer matches
athletes, so accents and capitalisation don't matter.

```sh
osl-import instagram
osl-import instagram --validate-only
```

The file is the truth, so deleting a line takes the handle off the site.

Only add a handle when the account is obviously the athlete's.
