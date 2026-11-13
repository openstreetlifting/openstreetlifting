# Data Reference

OpenStreetlifting stores each competition in two files: `competition.toml` for
its details and `entries.csv` for its results.

You can browse a real example in the
[FinalRep Worlds 2023 folder on GitHub](https://github.com/openstreetlifting/openstreetlifting/tree/main/backend/data/competitions/finalrep/2023/worlds-2023).

## Competition Location

```text
backend/data/competitions/{federation}/{year}/{slug}/
├── competition.toml
└── entries.csv
```

| Part           | What to enter                                                    |
| -------------- | ---------------------------------------------------------------- |
| `{federation}` | The federation name in lowercase with hyphens.                   |
| `{year}`       | The year the competition started.                                |
| `{slug}`       | A unique competition name such as `worlds-2023`; used in its URL. |

## competition.toml

```toml
event = "MPDS"
sources = ["https://final-rep.com/world-championship/"]

[competition]
name = "Worlds"
series = "finalrep-worlds"
start_date = "2023-09-30"
end_date = "2023-10-01"
city = "Cologne"
country = "DE"
status = "completed"

[federation]
name = "FinalRep"
```

| Field                     | What to enter                                                             |
| ------------------------- | ------------------------------------------------------------------------- |
| `event`                   | See [Event](#event). Required with results.                               |
| `sources`                 | See [Sources](#sources).                                                  |
| `competition.name`        | The competition's name.                                                   |
| `competition.series`      | Optional stable identifier shared by editions; see [Editions](#editions). |
| `competition.start_date`  | The first day, as `YYYY-MM-DD`.                                           |
| `competition.end_date`    | The last day; defaults to `start_date` when omitted.                      |
| `competition.venue`       | The venue name, if known; omit it when unknown.                           |
| `competition.city`        | The city, if known.                                                       |
| `competition.region`      | The region or state, if known.                                            |
| `competition.country`     | An ISO 3166-1 alpha-2 country code, such as `FR` or `DE`.                 |
| `competition.scoring`     | `total` or `ris`; required for mixed contests.                            |
| `competition.status`      | See [Status](#status) below.                                              |
| `federation.name`         | The federation's name, written consistently across competitions.          |
| `federation.abbreviation` | Its abbreviation, if used.                                                |
| `federation.country`      | Its ISO 3166-1 alpha-2 country code, if applicable.                       |

### Editions

Give editions of the same competition the same `competition.series`, for example
`finalrep-worlds`.

- Lowercase ASCII letters and digits separated by single hyphens.
- Prefixed with the organizer or federation; no years or edition numbers.
- Stable when an edition changes name, venue or organizer.
- Separate contests, such as All4 and Classic championships, get separate identifiers.

### Event

| `event` | Also called | Movements                       |
| ------- | ----------- | ------------------------------- |
| `MPDS`  | **All4**    | Muscle-up, pull-up, dips, squat |
| `PD`    | **Classic** | Pull-up, dips                   |

| Letter | Movement  |
| ------ | --------- |
| `M`    | Muscle-up |
| `P`    | Pull-up   |
| `D`    | Dips      |
| `S`    | Squat     |

Other combinations of these four lifts, such as `PDS`, use the same letter order.

### Status

| `status`    | Meaning                                         | Files                         |
| ----------- | ----------------------------------------------- | ----------------------------- |
| `upcoming`  | A planned competition without results.          | `competition.toml` only       |
| `live`      | A competition in progress with partial results. | Both; only published results  |
| `completed` | A competition with final results.               | Both                          |

Change `live` to `completed` when final results are available.

### Sources

Include at least one source link, archived file path, or description in `sources`,
above `[competition]`, for both announcements and results. Include every source
used to compile the results. Empty lists and blank references are rejected.

## entries.csv

Each row records one athlete's results in one contest. Use kilograms for weights
and a decimal point for fractions, such as `72.5`.

```csv
Sex,FirstName,LastName,Disambiguation,Country,BodyweightKg,ReportedRis,TotalKg,Status,StatusReason,MuscleUp1Kg,MuscleUp2Kg,MuscleUp3Kg,BestMuscleUpKg,PullUp1Kg,PullUp2Kg,PullUp3Kg,BestPullUpKg,Dips1Kg,Dips2Kg,Dips3Kg,BestDipsKg,Squat1Kg,Squat2Kg,Squat3Kg,BestSquatKg
```

| Header                                                                | Presence                                                  |
| --------------------------------------------------------------------- | --------------------------------------------------------- |
| `Division`                                                            | Optional; first column, when separate divisions are needed. |
| `WeightClassKg`                                                       | Optional; after `Sex`, for competitions with weight classes. |
| `FirstName`, `Disambiguation`, `StatusReason`                         | Optional; `prepare` restores them, keeping existing values. |
| `NativeName`, `CategorySex`, `BodyweightSource`, `ReportedRisEdition` | Optional.                                                 |
| All other base headers, including `TotalKg`                           | Required, even when their cells are empty.                |

Keep the columns for all four movements, leaving cells empty for movements
outside the event. Unknown or misspelled headers are rejected.

### Athlete and result columns

| Column               | What to enter                                                                    |
| -------------------- | -------------------------------------------------------------------------------- |
| `Division`           | A separate contest for the same weight class, such as `Open` or `Elite`.         |
| `Sex`                | The athlete's sex: `M` or `F`. `MX` is not an athlete sex.                       |
| `WeightClassKg`      | See [WeightClassKg](#weightclasskg). Leave it out for a meet with none.          |
| `FirstName`          | The athlete's first name. Leave empty for a single name.                         |
| `LastName`           | The surname, or the full name for an athlete known by a single name. Required.   |
| `NativeName`         | The name written in another script, if any.                                      |
| `Disambiguation`     | Leave empty unless different athletes share a name; see [Names](#names).         |
| `Country`            | ISO 3166-1 alpha-2 code; leave empty when the source omits it.                   |
| `CategorySex`        | See [Athlete sex and mixed contests](#athlete-sex-and-mixed-contests).           |
| `BodyweightKg`       | The athlete's bodyweight, if known.                                              |
| `BodyweightSource`   | `reported` (default when bodyweight is supplied) or `recovered`.                 |
| `ReportedRis`        | The source's published RIS score, whether or not bodyweight is known.            |
| `ReportedRisEdition` | The source's formula year, when established.                                     |
| `TotalKg`            | Overall total from the source, or filled by `prepare` from a complete breakdown. |
| `Status`             | `competed`, `disqualified` or `no_show`. Empty means `competed`.                 |
| `StatusReason`       | A short explanation for `disqualified` or `no_show`. Leave empty for `competed`. |

### Names

Use the athlete's usual spelling and capitalization, including accents and
hyphens. Use a Latin spelling in `FirstName` and `LastName`.

When different athletes share a name and sex, use `Disambiguation` numbers
starting at `1`.

### Athlete sex and mixed contests

| `CategorySex`     | Contest                  |
| ----------------- | ------------------------ |
| Empty or omitted  | The athlete's `Sex`      |
| `M` or `F`        | Must match `Sex`         |
| `MX`              | Mixed contest            |

Mixed contests need `scoring` under `[competition]`:

| `scoring` | Ranking                                                                 |
| --------- | ----------------------------------------------------------------------- |
| `total`   | `TotalKg`                                                               |
| `ris`     | RIS. All4 uses each athlete's own formula when its inputs are available. |

### WeightClassKg

| Sex   | Standard classes                             |
| ----- | -------------------------------------------- |
| Men   | `66`, `73`, `80`, `87`, `94`, `101`, `101+`  |
| Women | `52`, `57`, `63`, `70`, `70+`                |

Other positive limits are accepted. Record the class used by the competition,
using the same notation.

### Attempts

| Movement  | Attempt columns                             | Best-lift column |
| --------- | ------------------------------------------- | ---------------- |
| Muscle-up | `MuscleUp1Kg`, `MuscleUp2Kg`, `MuscleUp3Kg` | `BestMuscleUpKg` |
| Pull-up   | `PullUp1Kg`, `PullUp2Kg`, `PullUp3Kg`       | `BestPullUpKg`   |
| Dips      | `Dips1Kg`, `Dips2Kg`, `Dips3Kg`             | `BestDipsKg`     |
| Squat     | `Squat1Kg`, `Squat2Kg`, `Squat3Kg`          | `BestSquatKg`    |

| Cell    | Meaning                                    |
| ------- | ------------------------------------------ |
| `100`   | Successful attempt at 100 kg               |
| `100x`  | Missed attempt                             |
| Empty   | Not taken or unknown                       |
| `0`     | A recorded attempt, not a missing value    |

- Best lift: the heaviest successful attempt. May be left empty; if filled, it
  must agree with the attempts.
- If only the best lift is known, fill that column alone.

### Status rules

| `Status`       | Lifts                        | `TotalKg` | `ReportedRis`  | Ranked |
| -------------- | ---------------------------- | --------- | -------------- | ------ |
| `competed`     | Allowed                      | Allowed   | Allowed        | Yes    |
| `disqualified` | Allowed; must be consistent  | Empty     | Allowed        | No     |
| `no_show`      | Empty                        | Empty     | Empty or `0`   | No     |

Mark an athlete who missed every attempt at a movement as `disqualified`, with a
reason such as `Bombed the squat`.

### Totals and preparation

- Existing totals must equal the sum of a complete breakdown, or be at least the
  known subtotal when the breakdown is incomplete.
- Negative totals are rejected.
- When both the total and part of the breakdown are unknown, leave them empty.
  Missing lifts are never treated as zero.
- Preserve a published total when available.
- The former `ReportedTotalKg` header is rejected.

| Command                                  | Effect                                                                                                   |
| ---------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `osl-import prepare <directory>`         | Fills a missing total when every event movement has a successful best lift, validates, formats the files. |
| `osl-import prepare --check <directory>` | Leaves files unchanged and fails if preparation is needed.                                               |

Import requires a stored total whenever the breakdown is complete; it never
fills one.

### Bodyweight and reported RIS

Enter each value the source provides: `BodyweightKg`, `ReportedRis`, or both.
Leave unknown values empty. The competition year alone does not establish
`ReportedRisEdition`.

- All4, complete, `competed`, with bodyweight and `M` or `F`: `ReportedRis` is
  checked against its edition, rounded to two decimal places. A disagreement
  blocks import. An unknown edition or missing result gives a warning.
- `BodyweightSource=recovered` (All4 only) requires `BodyweightKg`,
  `ReportedRis`, `ReportedRisEdition` and a complete total; the estimate must
  reproduce the original score.
- The former `Ris` header is rejected. Rename it to `ReportedRis`, preserving
  its values.

### RIS for fewer than four lifts

Some competitions, such as FinalRep Underground 3L, rank athletes by a published
RIS score across pull-up, dips and squat. Record the actual event as `PDS`.

For any supported event with one, two or three movements, RIS ranking requires:

- `scoring = "ris"` under `[competition]`, supported by the organizer's rules.
- No weight classes anywhere in the competition. Omit `WeightClassKg`; a meet
  containing even one weight-classed contest is ineligible for shorter-event RIS.
- The organizer's published score in `ReportedRis` for each athlete who receives
  a RIS placing.

For a mixed contest, retain each athlete's `Sex` and set `CategorySex=MX`.

A missing score produces a warning and leaves that athlete without a RIS
placing, even when total and bodyweight are known. OSL does not verify a
shorter-event score against the All4 formula or recover bodyweight from it.

## Validation

Errors must be fixed before import. Warnings flag details to review.

Follow [Validate it locally](./CONTRIBUTING_DATA.md#validate-it-locally) to check
your files. A successful check confirms that the format is accepted; compare the
results with the source too.
