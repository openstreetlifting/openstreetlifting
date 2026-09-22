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
| `competition.end_date`    | The last day. For a one-day event, repeat the start date.        |
| `competition.city`        | The city, if known.                                              |
| `competition.region`      | The region or state, if known.                                   |
| `competition.country`     | An ISO 3166-1 alpha-2 country code, such as `FR` or `DE`.         |
| `competition.status`      | See [Status](#status) below.                                     |
| `federation.name`         | The federation's name, written consistently across competitions. |
| `federation.abbreviation` | Its abbreviation, if used.                                       |
| `federation.country`      | Its ISO 3166-1 alpha-2 country code, if applicable.              |

The names, dates and competition country are required. Optional fields can be
left out.

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

Put source links or short descriptions in the `sources` list, above
`[competition]`. Include every source used to compile the results. This field is
optional in the format, but sources help others check and correct the archive.

## entries.csv

Each row records one athlete's results in one contest. Keep the column names
exactly as shown below. Use kilograms for weights and a decimal point for
fractions, such as `72.5`.

All columns are required in the header except `Division`, `NativeName`,
`BodyweightSource` and `ReportedRisEdition`. A required column may still contain
empty cells, as described below. Keep the columns for all four movements, leaving
cells empty for movements outside the event.

### Athlete and result columns

| Column           | What to enter                                                                    |
| ---------------- | -------------------------------------------------------------------------------- |
| `Sex`            | `M` for men, `F` for women or `MX` for a mixed category.                         |
| `WeightClassKg`  | The weight class, such as `80` or `101+`. Leave it out for a meet with none.      |
| `FirstName`      | The athlete's first name. Leave empty for a single name.                         |
| `LastName`       | The surname, or the full name for an athlete known by a single name. Required.   |
| `Disambiguation` | Leave empty unless different athletes share a name; see [Names](#names).         |
| `Country`        | The athlete's ISO 3166-1 alpha-2 country code. Required.                           |
| `BodyweightKg`   | The athlete's bodyweight, if known.                                              |
| `Ris`            | The published RIS score when bodyweight is unavailable.                          |
| `Status`         | `competed`, `disqualified` or `no_show`. Empty means `competed`.                 |
| `StatusReason`   | A short explanation for `disqualified` or `no_show`. Leave empty for `competed`. |

### Names

Use the athlete's usual spelling and capitalization, including accents and
hyphens. Write `Anne-Sophie`, for example, rather than `ANNE-SOPHIE`. The importer
checks names but does not fix them for you.

Use a Latin spelling in `FirstName` and `LastName`. The optional `NativeName`
column preserves a name written in another script.

When different athletes share a name, use `Disambiguation` numbers starting at
`1` to tell them apart. Keep each person's number consistent across competitions;
check existing entries before assigning one.

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
reason such as `Bombed the squat`. A `no_show` must have no attempts or best lifts.

### Bodyweight and Ris

Enter `BodyweightKg` when the source provides it. Otherwise, enter the published
`Ris` score. Fill only one of these columns, or leave both empty if neither is
known.

I handle recovering missing bodyweights from RIS scores when possible. You only
need to provide the published data. Existing rows may contain both values after
recovery; keep them when making other corrections.

## Validation

The importer rejects unknown fields, missing required columns and inconsistent
results. Errors must be fixed before import. Warnings flag details to review,
such as a missing city, missing bodyweight and RIS, or a name appearing in several
contests.

Follow [Validate it locally](./CONTRIBUTING_DATA.md#validate-it-locally) to check
your files. A successful check confirms that the format is accepted; compare the
results with the source too.
