---
name: extract-competition
description: Extract streetlifting competition results from a web page, PDF, screenshot or Instagram post into a canonical JSON file under backend/imports. Use when the user shares competition results in any form and wants them added to OpenStreetLifting, or asks to continue building an existing competition file.
---

# Extract competition results

Turn source material into one canonical JSON file per competition, ready for
`osl-import`. A competition is usually spread across several posts or pages,
so a file is built up over several passes. Each pass adds one brick and is
reviewed as a git diff.

## Where files live

```
backend/imports/<competition-slug>/<competition-slug>.json
```

One file per competition, named after the slug. Never split one competition
across several files, and never put two competitions in one file.

## The loop

1. **Find the file.** If `backend/imports/<slug>/` exists, read the file
   first. You are extending it, not writing it.
2. **Extract only what this source shows.** One post usually covers one
   category, sometimes one movement.
3. **Merge into the file.** Add new categories and athletes. Fill in fields
   that were previously absent. Leave everything else exactly as it was.
4. **Format and validate**, from `backend/`:
   ```bash
   cargo run -p osl_importer --bin import -- fmt <file>
   cargo run -p osl_importer --bin import -- canonical <file> --validate-only
   ```
5. **Fix and repeat** until validation passes.
6. **Show the user `git diff`** and say what you added, what you could not
   read, and anything that looked contradictory.

Do not commit. The user reviews the diff and commits.

## Rules

**Never invent a value.** Most fields are optional. If a post shows names and
totals but no bodyweights, omit `bodyweight` entirely. Do not write `0`, do
not guess from the weight class, do not carry a value over from another
athlete. A missing field produces a validation warning, which is the correct
outcome. A wrong field produces a wrong leaderboard that nobody will notice.

**Never touch a brick already laid.** If the file already has category −87 and
this source is about −80, the diff must contain only −80. Changing anything
else means either you found a genuine error, in which case say so explicitly
and explain it, or you drifted, which is a bug.

**Stop and ask when the source is ambiguous.** Two posts disagreeing on a
weight, a name you cannot spell with confidence, an athlete who might be the
same person under a different spelling. Ask. Do not pick.

**Read attempts literally.** A crossed-out or red attempt is
`"is_successful": false`, not a missing attempt. Failed attempts matter: they
are part of the result and they affect nothing in the total, but removing
them loses real data.

## Format

`format_version` is `1.5.0`. Required fields have no marker; `?` means
optional and should be omitted when unknown.

```jsonc
{
  "format_version": "1.5.0",
  "source": {
    "type": "html",              // api | html | pdf | csv | image | manual
    "url": "https://...",        // ?
    "extracted_at": "2026-08-10T10:00:00Z",
    "extractor": "extract-competition-skill",
    "original_filename": "..."   // ?
  },
  "competition": {
    "name": "Annecy 4 Lift 2025",
    "slug": "annecy-4-lift-2025",
    "federation": {
      "name": "4Lift",
      "slug": "...",             // ?
      "abbreviation": "4L",      // ?
      "country": "FR"            // ?
    },
    "start_date": "2025-11-01",
    "end_date": "2025-11-02",
    "venue": "Oski Crossfit",    // ?
    "city": "Annecy",            // ?
    "country": "FR",             // ISO 3166-1 alpha-2
    "number_of_judges": 3,       // ? must be 1 or 3
    "status": "completed"        // ? draft|upcoming|live|completed|cancelled
  },
  "movements": [
    { "name": "Muscle-up", "order": 1, "is_required": true }  // order >= 1
  ],
  "categories": [
    {
      "name": "Catégorie -80",
      "gender": "M",             // M, F or MX
      "weight_class_slug": "M-80",   // standard class, see list below
      "athletes": [
        {
          "first_name": "Timothée",
          "last_name": "MERANDON",
          "disambiguation": 2,       // ? only to separate two real people
                                     //   who share a name, see below
          "gender": "M",         // ?
          "country": "FR",           // country represented, ISO alpha-2
          "nationality": "FR",       // ? citizenship if it differs
          "team": "...",             // ?
          "bodyweight": "88.7",      // ?
          "ris": "84.21",            // ? only when the source gives a
                                     //   score and no bodyweight
          "status": "competed",      // competed | disqualified
          "status_reason": "...",    // ? why disqualified
          "lifts": [
            {
              "movement": "Muscle-up",
              "attempts": [
                {
                  "attempt_number": 1,        // 1 to 3
                  "weight": "12.5",
                  "is_successful": true,
                  "judge_note": "[Autre]"     // ?
                }
              ]
            }
          ]
        }
      ]
    },
    {
      // A class outside the standard ladder, stated as bounds.
      "name": "Catégorie +87",
      "gender": "M",
      "weight_class_min": "87",      // above 87, no upper limit
      "athletes": []
    }
  ]
}
```

Every category needs a weight class, in one of two ways.

Use `weight_class_slug` for a standard class. It is one of `F-52` `F-57`
`F-63` `F-70` `F+70` `M-66` `M-73` `M-80` `M-87` `M-94` `M-101` `M+101`, and
it already carries the bounds, so a category using it must not also set
`weight_class_min` or `weight_class_max`.

Use the raw bounds for anything else: a meet running -75, or an open class
like +87 that merges the top of the ladder. `weight_class_min` is the lower
bound and `weight_class_max` the upper, and a class may set one or both.
`-75` is `weight_class_max: "75"`, `+87` is `weight_class_min: "87"`.

Read the bound off the category name, not off the athletes. A class called
+87 has a minimum of 87 even when the lightest athlete in it weighs 91.

Weights and bodyweights are JSON **strings**, not numbers. Countries are ISO
3166-1 alpha-2, so `FR` and never `France` or `FRA`.

### Names and who is who

Write the name the way the source spells it. Matching already ignores accents,
capitalisation and punctuation, so `MERANDON`, `Mérandon` and `merandon` are
one person however the source wrote it, and so are `Jean-Luc` and `Jean Luc`.
Never "fix" a name to make it match one you have seen before.

`disambiguation` is for the opposite case: two **different** people who share a
name, gender and country. Leave the first without it and number the rest from
2, the way OpenPowerlifting writes `John Doe #1` and `John Doe #2`. Without it
they merge into one athlete and their results pool together.

Only reach for it when the source makes clear these are two people, such as two
entries in one category with different bodyweights and different results. Two
spellings of one name are not that. If you cannot tell, stop and ask.

### What the validator rejects

Wrong `format_version`, empty `extractor`, a country that is not two letters,
a gender outside `M` `F` `MX`, a status outside the five listed, a judge count
other than 1 or 3, `end_date` before `start_date`, a duplicate or unnamed
movement, a movement `order` below 1, a category setting both the slug and the
raw bounds, a `weight_class_min` above its `weight_class_max`, an athlete setting both
`bodyweight` and `ris`, a lift naming a
movement not in `movements`, a lift with no attempts, an `attempt_number`
outside 1 to 3, a negative weight.

### What it only warns about

Missing venue, city or judges. An athlete with neither bodyweight nor ris. A category with no weight class at
all. A category with no athletes. An athlete with no lifts. These are normal for a file still being built, so
warnings are expected mid-construction and are not something to fix by
inventing data.

## Things that are not your job

- Ranks. Computed from the lifts on import, so no rank field exists.
- RIS scores. Computed on import.
- Totals. Derived from the best successful attempt per movement.
- Importing to the database. The user runs that.
