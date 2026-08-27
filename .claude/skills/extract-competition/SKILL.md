---
name: extract-competition
description: Extract streetlifting competition results from a web page, PDF, screenshot or Instagram post into a competition directory under backend/data/competitions. Use when the user shares competition results in any form and wants them added to OpenStreetLifting, or asks to continue building an existing competition.
---

# Extract competition results

Turn source material into one competition directory per competition, ready for
`osl-import`. A competition is usually spread across several posts or pages,
so a competition is built up over several passes. Each pass adds one brick and is
reviewed as a git diff.

## Where files live

```
backend/data/competitions/<federation>/<year>/<competition-slug>/
    competition.toml      the competition, its federation, and which movements it ran
    entries.csv    one row per athlete
```

`<federation>` is the federation's name slugified, so `FNSL` is `fnsl`.
`<year>` is the year the competition starts, and `<competition-slug>` is the slug the
competition keeps forever. All three are part of the contract: the slug is never
written inside a file, it is read from the directory name, and the competition
page links to `entries.csv` at exactly this path.

One directory per competition. Never split one competition across two, and
never put two competitions in one.

## The loop

1. **Find the competition.** If `backend/data/competitions/<federation>/<year>/<slug>/` exists,
   read both files first. You are extending them, not writing them.
2. **Extract only what this source shows.** One post usually covers one
   category, sometimes one movement.
3. **Merge.** Add new rows to `entries.csv`. Fill in cells that were
   previously empty. Leave everything else exactly as it was.
4. **Format and validate**, from `backend/`:
   ```bash
   cargo run -p osl_importer --bin import -- fmt data/competitions/<federation>/<year>/<slug>
   cargo run -p osl_importer --bin import -- canonical data/competitions/<federation>/<year>/<slug> --validate-only
   ```
5. **Fix and repeat** until validation passes.
6. **Show the user `git diff`** and say what you added, what you could not
   read, and anything that looked contradictory.

Do not commit. The user reviews the diff and commits.

## Rules

**Never invent a value.** Most cells are optional. If a post shows names and
totals but no bodyweights, leave `BodyweightKg` empty. Do not write `0`, do not
guess from the weight class, do not carry a value over from another athlete. An
empty cell produces a validation warning, which is the correct outcome. A wrong
cell produces a wrong leaderboard that nobody will notice.

**Never touch a brick already laid.** If the file already has the −87 rows and
this source is about −80, the diff must contain only −80 rows. Changing
anything else means either you found a genuine error, in which case say so
explicitly and explain it, or you drifted, which is a bug.

**Stop and ask when the source is ambiguous.** Two posts disagreeing on a
weight, a name you cannot spell with confidence, an athlete who might be the
same person under a different spelling. Check the API first (see below), and if
it does not settle it, ask. Do not pick.

**Read attempts literally.** A crossed-out or red attempt is a miss, written
with an `x` suffix, not a blank cell. Failed attempts matter: they add nothing
to the total, but removing them loses real data.

**Zero is a weight, not a blank.** A muscle-up, pull-up or dip with no added
weight is a real lift, and a source may write it as `0 kg`. A successful one is
`0`, a missed one is `0x`.

A source may also print `0 kg` to mean the opposite: nothing was lifted, or the
athlete never competed. That is not an attempt and gets an empty cell. Tell
them apart the same way as any other attempt, by whether the source marks it
successful. If the source gives you no way to tell, ask.

An athlete who missed every attempt in a movement still gets those cells, each
with its `x`. Do not blank the movement and do not invent a zero for it: they
contested it and lifted nothing, which is different from lifting their
bodyweight.

## Looking things up in the public API

When a source leaves you unsure about an athlete, a competition, a federation
or a spelling, query the project's own read-only API before asking the user:

- Docs: <https://api.openstreetlifting.org/swagger-ui/>
- Base: `https://api.openstreetlifting.org/api/v1`

```
GET /athletes?page=1&page_size=50
GET /athletes/{slug}?include=competitions,records
GET /competitions?page=1&page_size=50&include=federation,movements
GET /competitions/{slug}?include=federation,results
```

Everything is a plain GET, no auth. Use it to answer: is this athlete already
known and how is their name spelled, does this competition already exist under
a slug, which movements did this federation run last year, is the person in
this post the same one who lifted in another competition.

Two limits, both important.

The API serves what has **already been imported**, which is a projection of the
files. It is not an independent record of the competition and it is not ground truth
about this source. It tells you what the project already believes.

So it never supplies a value. It can tell you that `Timothée MERANDON` is the
existing spelling, or that two athletes really do share a name and need
`Disambiguation`. It cannot give you a bodyweight, an attempt or a country that
your source does not show. Nothing read from the API gets written into a file
as if the source had printed it. **Never invent a value** still holds, and an
API lookup that contradicts the source is a reason to stop and ask, not to
overwrite either one.

## competition.toml

Everything not marked required is optional and should be left out entirely when
unknown. There is no version key, and a key that is not one of these is
rejected by name, so a typo fails the import instead of going quiet.

```toml
event = "MPDS"                    # which movements were contested, see below
sources = [                       # where the results came from
  "https://www.instagram.com/p/xxxx/",
]

[competition]                     # required
name = "FNSL Elite 2026"          # required
start_date = "2026-05-15"         # required, quoted
end_date = "2026-05-17"           # required, quoted
city = "Sevran"
region = "Île-de-France"          # ISO 3166-2 subdivision name
country = "FR"                    # required, ISO 3166-1 alpha-2
status = "completed"              # draft | upcoming | live | completed | cancelled

[federation]                      # required
name = "FNSL"                     # required
abbreviation = "FNSL"
country = "FR"
```

Dates are **quoted strings**, not bare TOML dates.

`event` is the movements the competition contested, as letters in this fixed order:

| Letter | Movement  |
|--------|-----------|
| `M`    | Muscle-up |
| `P`    | Pull-up   |
| `D`    | Dips      |
| `S`    | Squat     |

So a full four-movement competition is `MPDS`, a squat-and-dips competition is `DS`, a
muscle-up-only competition is `M`. The letters must stay in `MPDS` order. These four
are the only movements that exist; a competition contesting anything else cannot be
imported without a schema change, so stop and say so.

Only a `MPDS` competition gets a total and a RIS. The formula is fitted to four-lift
totals, so a partial event is ranked per movement and nothing else.

## entries.csv

One row per athlete, 26 columns (27 with `Division`), always in this order:

```
Sex,WeightClassKg,FirstName,LastName,Disambiguation,Country,BodyweightKg,Ris,Status,StatusReason,
MuscleUp1Kg,MuscleUp2Kg,MuscleUp3Kg,BestMuscleUpKg,
PullUp1Kg,PullUp2Kg,PullUp3Kg,BestPullUpKg,
Dips1Kg,Dips2Kg,Dips3Kg,BestDipsKg,
Squat1Kg,Squat2Kg,Squat3Kg,BestSquatKg
```

(on disk that is one header line, not four)

| Column | Meaning |
|---|---|
| `Division` | Only when the competition ran divisions. See below |
| `Sex` | `M`, `F` or `MX`. Required |
| `WeightClassKg` | `80` for −80, `101+` for +101. Required |
| `FirstName` `LastName` | As the source spells them. Required |
| `Disambiguation` | Only to separate two real people sharing a name |
| `Country` | Required, ISO 3166-1 alpha-2 |
| `BodyweightKg` | Never set alongside `Ris` |
| `Ris` | Only when the source gives a score and no bodyweight |
| `Status` | `competed` or `disqualified`. Empty means competed |
| `StatusReason` | Why they were disqualified |

`Country` is part of who an athlete *is*: the same name with a different
country imports as a different person. Use the country the source lists them
under, and keep it consistent for one person across competitions.

The category is not stored. It is derived from `Division`, `Sex` and
`WeightClassKg`, so `M` + `80` renders as "Men -80kg", `M` + `101+` as
"Men +101kg", and `Elite` + `M` + `80` as "Elite Men -80kg".

### Division

Most competitions run one contest per sex and weight class, and those files have **no
`Division` column at all**. Leave it out rather than adding an empty one.

Add it as the **first** column, before `Sex`, only when the competition ran the same
weight class more than once, for example an Elite board and an Open board
lifting -80kg separately. Those are two contests with two winners, and without
the column they collapse into one and a winner disappears.

A competition that is *itself* one division, like FNSL Elite 2026, does not need the
column. There is nothing to tell apart, and the competition name already says Elite.

```
Division,Sex,WeightClassKg,FirstName,...
Elite,M,80,Ana,...
Open,M,80,Cy,...
```

The value is free text, whatever the competition called it. There is no fixed list,
and names are not standardised between federations. Use the source's own
wording, and keep it identical for every row in that division. Every row needs
a value once the column exists.

Placings are computed per division, so a lifter can appear in two divisions of
one competition. Global rankings ignore division entirely, since a 200 kg squat is a
200 kg squat either way.

Weight classes are written **bound first, plus as a suffix**: `80`, not `-80`,
and `101+`, not `+101`. Nothing may start with `+` or `-`, because a
spreadsheet reads that as a formula and silently eats the file. Read the bound
off the category name, not off the athletes: a class called +101 has a minimum
of 101 even when the lightest athlete in it weighs 105.

The standard ladder is `52` `57` `63` `70` `70+` for women and `66` `73` `80`
`87` `94` `101` `101+` for men. A competition running something else, like `75`, is
fine and stores its bound directly.

### Attempt cells

| Cell | Meaning |
|---|---|
| `100` | Good lift at 100 kg |
| `100x` | Missed at 100 kg |
| `0` | Good lift at bodyweight |
| `0x` | Missed at bodyweight |
| *(empty)* | Not attempted, or unknown |

Never write a negative weight. `-100` is rejected with a message pointing at
`100x`.

`BestMuscleUpKg` and friends are **derived**. When the attempt cells are
filled, `fmt` computes the best from them and overwrites whatever is there, so
the two can never disagree and you never have to fill it yourself. An athlete
who missed every attempt in a movement gets an empty best, which is what makes
a bombed movement different from a lift of 0.

Fill it by hand only for a source that publishes a best per movement with **no
attempt breakdown**. Leave the three attempt cells empty and put the weight in
the best cell; that is the one case where it is real data rather than a
summary.

A movement outside the competition's `event` must have all four cells empty.

There is no Total, no Rank and no Place column. Those are computed on import.

## Announcing a competition nobody has lifted yet

A competition with a date and a venue and no results is **`competition.toml` on its own, with
no `entries.csv` at all**, and `status = "upcoming"`:

```toml
sources = ["https://..."]

[competition]
name = "FNSL Nationals 2027"
start_date = "2027-06-12"
end_date = "2027-06-12"
city = "Sevran"
country = "FR"
status = "upcoming"

[federation]
name = "FNSL"
country = "FR"
```

It lives at `backend/data/competitions/fnsl/2027/fnsl-nationals-2027/`. Include
`event` only if the calendar actually states the format.

Results land by **adding `entries.csv` to this same directory**, never by
starting a second one, so the slug has to be the one the competition will keep. When
they do, move `status` to `completed` in the same edit. No `entries.csv` and a
status other than `upcoming` is rejected, and so is an `entries.csv` sitting
next to `status = "upcoming"`.

## Names and who is who

Write the name the way the source spells it. Matching already ignores accents,
capitalisation and punctuation, so `MERANDON`, `Mérandon` and `merandon` are
one person however the source wrote it, and so are `Jean-Luc` and `Jean Luc`.
Never "fix" a name to make it match one you have seen before.

`Disambiguation` is for the opposite case: two **different** people who share a
name, gender and country. Leave the first empty and number the rest from 2, the
way OpenPowerlifting writes `John Doe #1` and `John Doe #2`. Without it they
merge into one athlete and their results pool together.

Only reach for it when the source makes clear these are two people, such as two
rows in one class with different bodyweights and different results. Two
spellings of one name are not that. If you cannot tell, stop and ask.

## What the validator rejects

An unknown key in `competition.toml`; a missing competition or federation name; a
country that is not two letters; a `Sex` outside `M` `F` `MX`; a status outside
the five listed; `end_date` before `start_date`; an `event` with an unknown
letter, a repeated letter, or letters out of `MPDS` order; a missing, unknown or
duplicated column in `entries.csv`; a negative weight; a cell filled for a
movement outside the event; a
`Disambiguation` below 1; both `BodyweightKg` and `Ris` on one row; a
bodyweight of zero or less; the same athlete twice in one class; and a
directory whose year does not match `start_date`.

No `entries.csv` unless `status` is `upcoming`, and an `entries.csv` when it is.

## What it only warns about

A missing city. A row with neither bodyweight nor ris. An athlete with no lifts
at all. One athlete appearing in two classes. These are normal for a competition still
being built, so warnings are expected mid-construction and are not something to
fix by inventing data.

## Things that are not your job

- Ranks. Computed from the lifts on import, so no rank column exists.
- RIS scores. Computed on import, unless the source states one.
- Totals. Derived from the best successful attempt per movement.
- Importing to the database. The user runs that.
