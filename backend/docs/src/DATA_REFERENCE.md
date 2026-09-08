# Data Reference

<!-- The contract. Anything not written here is not guaranteed. -->

## How the format changes

<!-- The files carry no version key. The importer that reads them ships in the
     same commit, so the column list and the competition.toml keys are the contract,
     and both reject anything they do not know. New optional shape is added,
     and a change that invalidates existing files rewrites them in the same
     commit and is called out with `!`. -->

## Directory layout

## competition.toml

### [competition]

### [federation]

### event

<!-- Movement codes and what MPDS means. -->

### status

### sources

## entries.csv

### Column overview

<!-- The full ordered list, and the rule that unknown columns are rejected. -->

### Identity columns

<!-- Sex, FirstName, LastName, Disambiguation, Country. -->

### Division

<!-- Optional. Free text. Present only when a competition ran one class more than
     once. -->

### WeightClassKg

<!-- Bound-first notation, why nothing may start with + or -, the standard
     ladder, and non-standard classes. -->

### Bodyweight and Ris

`BodyweightKg` records a source-provided or recovered bodyweight. `Ris` records the
original published score. Normally provide one or the other.

Recovery may preserve both by adding `BodyweightSource=recovered` and
`ReportedRisEdition` (the edition that produced the original score). These optional
columns are omitted when unused. A bodyweight without a source marker is treated
as `reported`; this describes its origin, not independent verification of a weigh-in.

A recovered row must have a complete four-movement performance and reproduce its
original RIS under the recorded source edition. Import stores the original score
separately and computes the active ranking score using the current RIS edition.
See [recovering missing bodyweights](./CONTRIBUTING_DATA.md#recovering-missing-bodyweights)
for the authoring workflow.

### Attempt and best-lift columns

<!-- The 100 / 100x notation, and when to use BestXKg instead of attempts. -->

### Status and StatusReason

## How the files are interpreted

### Contests and placings

<!-- A contest is a competition, a weight class and a division. Placings are
     computed, never stated in the file. -->

### Totals and events

### RIS scoring

Import computes RIS for eligible four-movement performances with a bodyweight,
using the current edition (2026). Global rankings use this computed score, including
when the bodyweight was recovered from a score published under an older edition.

The original `Ris`, its `ReportedRisEdition`, and the bodyweight's origin are
preserved separately in the database. Re-importing or running `recompute-ris`
updates the ranking score without overwriting that evidence. Import must run first
to load CSV changes; `recompute-ris` only reads the database.

Rows without a bodyweight retain their reported score. Recalculation skips these
rows, disqualified athletes, no-shows, and events without all four movements.

## Validation rules

### Errors

### Warnings

## Data licence

<!-- What contributors agree to, and how the archive may be reused. -->
