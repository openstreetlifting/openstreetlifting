# Data Reference

OpenStreetlifting stores each competition in two files: `competition.toml` for
its details and `entries.csv` for its results. This page explains what to put in
each file. For the steps to submit them, see [Contributing Data](./CONTRIBUTING_DATA.md).

You can browse a real example in the
[FinalRep Worlds 2023 folder on GitHub](https://github.com/openstreetlifting/openstreetlifting/tree/main/backend/data/competitions/finalrep/2023/worlds-2023).

## Competition Location

```text
backend/data/competitions/{federation}/{year}/{slug}/
├── competition.toml
└── entries.csv
```

Use the federation name in lowercase with hyphens, the year the competition
started and a unique competition name such as `worlds-2023`. This last part, the
_slug_, also identifies the competition on the website. Keep it unchanged when
correcting results.

## competition.toml

This file describes the competition metadata

For example:

```toml
event = "MPDS"
sources = ["https://final-rep.com/world-championship/"]

[competition]
name = "Worlds"
start_date = "2023-09-30"
end_date = "2023-10-01"
city = "Cologne"
country = "DE"
status = "completed"

[federation]
name = "FinalRep"
```

### Competition and federation details

| Field                     | What to enter                                                    |
| ------------------------- | ---------------------------------------------------------------- |
| `competition.name`        | The competition's name.                                          |
| `competition.start_date`  | The first day, as `YYYY-MM-DD`.                                  |
| `competition.end_date`    | The last day; defaults to `start_date` when omitted.        |
| `competition.venue`       | The venue name, if known; omit it when unknown.                  |
| `competition.city`        | The city, if known.                                              |
| `competition.region`      | The region or state, if known.                                   |
| `competition.country`     | An ISO 3166-1 alpha-2 country code, such as `FR` or `DE`.         |
| `competition.status`      | See [Status](#status) below.                                     |
| `federation.name`         | The federation's name, written consistently across competitions. |
| `federation.abbreviation` | Its abbreviation, if used.                                       |
| `federation.country`      | Its ISO 3166-1 alpha-2 country code, if applicable.              |

The names, start date and competition country are required. Omit `end_date` for
a one-day competition. `prepare` removes an end date equal to the start date;
it preserves a later end date. Optional fields can be left out.

### Event

`event` lists the movements contested, using these letters in this order:

| Letter | Movement  |
| ------ | --------- |
| `M`    | Muscle-up |
| `P`    | Pull-up   |
| `D`    | Dips      |
| `S`    | Squat     |

Use `MPDS` for all four movements or `PD` for pull-up and dips. Include `event`
whenever you add results.

### Status

- `upcoming`: a planned competition without results. Add only `competition.toml`.
- `completed`: a competition with results. Include `entries.csv` and set `event`.

### Sources

Include at least one source link, archived file path, or description in `sources`,
above `[competition]`, for both announcements and results. Identify the original
publication or document so someone else can check it. Empty lists and blank
references are rejected. Include every source used to compile the results.

## entries.csv

Each row records one athlete's results in one contest. Keep the column names
exactly as shown below. Use kilograms for weights and a decimal point for
fractions, such as `72.5`.

Use this standard base header when starting a file:

```csv
Sex,FirstName,LastName,Disambiguation,Country,BodyweightKg,ReportedRis,TotalKg,Status,StatusReason,MuscleUp1Kg,MuscleUp2Kg,MuscleUp3Kg,BestMuscleUpKg,PullUp1Kg,PullUp2Kg,PullUp3Kg,BestPullUpKg,Dips1Kg,Dips2Kg,Dips3Kg,BestDipsKg,Squat1Kg,Squat2Kg,Squat3Kg,BestSquatKg
```

Add `WeightClassKg` after `Sex` for competitions with weight classes, and
`Division` first when separate divisions are needed. `NativeName`,
`BodyweightSource` and `ReportedRisEdition` are optional
additional columns.

The parser also accepts files without `FirstName`, `Disambiguation` or
`StatusReason`: an omitted column means an empty value for every entry. `prepare`
always restores these three columns in the standard base header, so formatted
files provide a consistent starting point for contributors. It preserves any
values already supplied. It fills an empty `TotalKg` only when every event
movement has a successful best lift.

All other base headers, including `TotalKg`, are required even when their cells
may be empty. Keep
the columns for all four movements, leaving cells empty for movements outside
the event. Unknown or misspelled headers are rejected.

### Athlete and result columns

| Column           | What to enter                                                                    |
| ---------------- | -------------------------------------------------------------------------------- |
| `Sex`            | `M` for men, `F` for women or `MX` for a mixed category.                         |
| `WeightClassKg`  | The weight class, such as `80` or `101+`. Leave it out for a meet with none.      |
| `FirstName`      | The athlete's first name. Leave empty for a single name.                         |
| `LastName`       | The surname, or the full name for an athlete known by a single name. Required.   |
| `Disambiguation` | Leave empty unless different athletes share a name; see [Names](#names).         |
| `Country`        | The athlete's ISO 3166-1 alpha-2 country code, if the source gives it.                           |
| `BodyweightKg`   | The athlete's bodyweight, if known.                                              |
| `ReportedRis`    | The source's published RIS score, whether or not bodyweight is known.            |
| `TotalKg` | Overall total from the source, or filled by `prepare` from a complete breakdown. |
| `Status`         | `competed`, `disqualified` or `no_show`. Empty means `competed`.                 |
| `StatusReason`   | A short explanation for `disqualified` or `no_show`. Leave empty for `competed`. |

### Names

Use the athlete's usual spelling and capitalization, including accents and
hyphens. Write `Anne-Sophie`, for example, rather than `ANNE-SOPHIE`. The importer
checks names but does not fix them for you.

Use a Latin spelling in `FirstName` and `LastName`. The optional `NativeName`
column preserves a name written in another script.

Identity uses the normalized name, sex, and `Disambiguation`. Correcting or
clearing `Country` preserves the athlete's profile and URL.

When different athletes share a name and sex, use `Disambiguation` numbers
starting at `1`, even if their countries differ. Keep each person's number
consistent across competitions; check existing entries before assigning one.

### Country

Keep the `Country` header. Leave its cell empty when the source omits the
athlete's country; the host country is not evidence of nationality.

The profile uses the known country from that athlete's entries. Empty cells
leave that evidence intact. If all entries have empty cells, the profile shows
“Country not recorded” and retains global rankings, records, and competition
history. National rankings require a known country.

Conflicting known countries stop preparation and import. Correct the affected
entries together and import them in one batch. Use `Disambiguation` only when
the entries belong to different people. A genuine change of sporting nationality
needs a separate reporting policy; do not split one person to bypass this check.

### Division

Add `Division` when a competition runs separate contests for the same weight
class, such as `Open` and `Elite`. Use the source's division names, with consistent
spelling. If you use this column, fill it for every athlete. Otherwise, leave it
out.

### WeightClassKg

Write `80` for a class up to 80 kg and `101+` for a class above 101 kg. Put the
number first, without `kg` or a leading sign.

The standard classes are:

- Men: `66`, `73`, `80`, `87`, `94`, `101`, `101+`.
- Women: `52`, `57`, `63`, `70`, `70+`.

Other positive limits are accepted. Record the class used by the competition,
using the same notation.

Some competitions rank on RIS alone and run no weight classes. Leave the column
out for those rather than working a class back from bodyweight. If you use the
column, fill it for every athlete.

### Attempts

Each movement has three attempt columns and one best-lift column:

| Movement  | Attempt columns                             | Best-lift column |
| --------- | ------------------------------------------- | ---------------- |
| Muscle-up | `MuscleUp1Kg`, `MuscleUp2Kg`, `MuscleUp3Kg` | `BestMuscleUpKg` |
| Pull-up   | `PullUp1Kg`, `PullUp2Kg`, `PullUp3Kg`       | `BestPullUpKg`   |
| Dips      | `Dips1Kg`, `Dips2Kg`, `Dips3Kg`             | `BestDipsKg`     |
| Squat     | `Squat1Kg`, `Squat2Kg`, `Squat3Kg`          | `BestSquatKg`    |

Write `100` for a successful attempt at 100 kg and `100x` for a missed attempt.
Leave the cell empty if the attempt was not taken or is unknown. A zero is a
recorded attempt, not a missing value.

When attempts are provided, the best lift is the heaviest successful one. You
may leave the best-lift cell empty; if you fill it, it must agree with the
attempts. If only the best lift is known, fill that column alone.

Mark an athlete who missed every attempt at a movement as `disqualified`, with a
reason such as `Bombed the squat`. A `no_show` must have no attempts, best lifts, total, or positive RIS. A published
zero RIS may be kept as source evidence. Disqualified rows may keep their lifts
and published RIS, but receive no total, calculated RIS, or ranking. Their lift
values must still be consistent.

### Totals and preparation

Use `TotalKg` for the overall result. Keep the total and any available lift
results together. The former `ReportedTotalKg` header is rejected.

Run `osl-import prepare <directory>` before importing. It fills a missing total
from successful best lifts only when every movement in the event is present,
validates the result, and formats the files. Existing totals must equal the sum
of a complete breakdown, or be at least the known subtotal when the breakdown
is incomplete. Negative totals and totals on disqualified or no-show rows are
rejected. A successful zero-weight lift counts as a recorded result.

When both the total and part of the breakdown are unknown, leave them empty.
The validator warns that the result has no total or calculated RIS. Missing
lifts are never treated as zero. Preserve a published total when available;
it supports total rankings and, for All4 with bodyweight, RIS calculation even
without a full breakdown.

`prepare` validates every selected competition before writing any files.
`prepare --check` leaves files unchanged and fails if preparation is needed.
Import requires a stored total whenever the breakdown is complete; it never
fills one. Rankings, pages and RIS calculations use that stored value.

### Bodyweight and reported RIS

Enter each value the source provides: `BodyweightKg`, `ReportedRis`, or both.
Leave unknown values empty. Use `ReportedRisEdition` for the source's formula
year when established; the competition year alone does not establish it.

For a complete, competed All4 result with bodyweight and a known M or F formula,
the validator checks the published score against its stated edition, rounded to
two decimal places. A disagreement blocks import. When the edition or required
results are unknown, both values are preserved and a warning explains why the
score could not be checked. Calculated ranking scores are stored separately;
they never replace `ReportedRis` in the CSV. A published RIS can still be used
when bodyweight is known but the total is missing. Scores published for shorter
events remain source evidence; OSL does not recompute them with the All4 formula.

Missing bodyweights can still be recovered from a published score and a complete
total. The recovery tool writes the estimate to `BodyweightKg`, marks it
`BodyweightSource=recovered`, and preserves `ReportedRis` and
`ReportedRisEdition`. All four fields are required for a recovered bodyweight;
the estimate must reproduce the original score. A reported weigh-in uses
`BodyweightSource=reported`, which is the default when bodyweight is supplied.

The former `Ris` header is rejected. Rename it to `ReportedRis`, preserving its
values. Existing archive files already use the new name.

## Validation

The importer rejects unknown fields, missing required columns and inconsistent
results. Errors must be fixed before import. Warnings flag details to review,
such as a missing city, missing bodyweight and RIS, or a name appearing in several
contests.

Follow [Validate it locally](./CONTRIBUTING_DATA.md#validate-it-locally) to check
your files. A successful check confirms that the format is accepted; compare the
results with the source too.
