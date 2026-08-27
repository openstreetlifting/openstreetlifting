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

<!-- Why the two are mutually exclusive. -->

### Attempt and best-lift columns

<!-- The 100 / 100x notation, and when to use BestXKg instead of attempts. -->

### Status and StatusReason

## How the files are interpreted

### Contests and placings

<!-- A contest is a competition, a weight class and a division. Placings are
     computed, never stated in the file. -->

### Totals and events

### RIS scoring

<!-- Computed vs reported, and when neither is possible. -->

## Validation rules

### Errors

### Warnings

## Data licence

<!-- What contributors agree to, and how the archive may be reused. -->
