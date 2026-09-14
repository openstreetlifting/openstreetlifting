# Staging fixtures

Invented data, not part of the archive. No real competition, federation or
person.

It exists so the redaction path can be checked on a deployed site:
`Redacted Athlete #9001` competes in
`competitions/osl-staging/2026/staging-redaction-check/`.

Only staging imports it:

```sh
osl-import bulk-import \
  --directory ./data/competitions \
  --directory ./data/staging/competitions \
  --prune --yes
```

Production runs without the second `--directory`, and `--prune` removes these
competitions if they ever reach it.

Stand-in numbers from 9000 up are reserved for fixtures. `osl-import redact`
hands out numbers below that.
