# Contributing Data

<!-- One paragraph: the files are the data, a contribution is a pull request
     that adds or corrects a file. No account, no admin panel. -->

## Before you start

### What counts as a source

<!-- Live stream, federation PDF, scoresheet photo, Instagram post. What makes
     a source good enough to import, and what does not. -->

### Check the competition is not already there

<!-- How to search the archive first. -->

## How a competition is stored

<!-- data/competitions/{federation}/{year}/{slug}/ holding competition.toml and entries.csv, and
     why the path matters. -->

## Adding a competition

### 1. Create the directory

### 2. Write competition.toml

### 3. Write entries.csv

### 4. Validate it locally

<!-- The import binary, --validate-only, and what a clean run looks like. -->

### 5. Open a pull request

<!-- What to put in the description, especially the source link. -->

## Correcting results that are already published

<!-- Edit the file and re-import. What happens to rows removed from a file. -->

### Recovering missing bodyweights

When a source gives a RIS but no bodyweight, recovery can reverse the edition that
produced that score. Choose the source edition explicitly; the competition year
does not establish it. From `backend`, preview a competition before writing:

```sh
SQLX_OFFLINE=true cargo run -p recover-bodyweight -- --edition 2024 --check data/competitions/finalrep/2023/worlds-2023
```

Remove `--check` to write accepted recoveries. The command also accepts several
competition directories or a tree containing them.

The performance must have a valid result in all four movements, consistent best
lifts and attempts, a positive total and RIS, and status `competed`. Existing
bodyweights are left alone. A successful zero-kilogram lift is valid.

Recovered weights must reproduce the original score, fall within 35–200 kg, and
fit the class bounds with 0.5 kg tolerance. There is no candidate minimum. If more
than 20% of eligible candidates fail, valid recoveries in that competition are
withheld. Output distinguishes ineligible, rejected, withheld and accepted rows.
These checks do not prove the source edition or bound rounding uncertainty.

Accepted rows retain their original `Ris` and gain `BodyweightKg`,
`BodyweightSource=recovered`, and `ReportedRisEdition`. Review the CSV changes and
validate them with `cargo run -p osl_importer --bin import -- bulk-import --validate-only`.
After migrations and import, rankings use the current RIS edition while source
evidence remains preserved; see [RIS scoring](./DATA_REFERENCE.md#ris-scoring).

## Things people get wrong

### Athletes who share a name

<!-- Disambiguation, and why country is part of identity. -->

### Competitions that ran more than one division

<!-- When the Division column is needed and when it is not. -->

### Weight classes outside the standard ladder

## Review checklist

<!-- Short list a contributor can self-check against before opening the PR. -->

## Getting help

<!-- Where to ask. Issue tracker, contact address. -->
