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
4. **Look up every athlete the source introduces** against the API, so a
   reversed or misspelled name does not become a second athlete. See
   [Before writing a new athlete, look them up](#before-writing-a-new-athlete-look-them-up).
5. **Format and validate**, from `backend/`:
   ```bash
   cargo run -p osl_importer --bin import -- fmt data/competitions/<federation>/<year>/<slug>
   cargo run -p osl_importer --bin import -- canonical data/competitions/<federation>/<year>/<slug> --validate-only
   ```
6. **Fix and repeat** until validation passes.
7. **Show the user `git diff`** and say what you added, what you could not
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

One reading settles most of these without asking: **a 0 kg squat is not a lift
anyone can attempt.** So a row showing `0 kg` on the squat, and especially a row
showing `0 kg` across all four movements, is a placeholder for an athlete who
never lifted, not twelve misses at bodyweight. Compare it against a genuine bomb
in the same competition, which prints the real weights the athlete called and failed
(`20 / 50 / 90 / 160`, all missed). Placeholder rows get empty attempt cells and the status
`no_show`, which exists for exactly this: an entrant who never lifted at all, with
no bodyweight and no attempts behind it. They are not `disqualified`, which is for
someone who took attempts and was ruled out. The validator rejects a `no_show`
carrying any lift.

An athlete who missed every attempt in a movement still gets those cells, each
with its `x`. Do not blank the movement and do not invent a zero for it: they
contested it and lifted nothing, which is different from lifting their
bodyweight.

Bombing a movement ends their competition, so that athlete is `disqualified`,
never `competed` and never `no_show`: they turned up and lifted, the standing is
what changed. Sources say the same thing in their own way, printing a dash
for the place and the total. The importer refuses a bombed athlete left as
`competed` rather than correcting it for you, because a status is something the
source states and not something to infer. Their missed attempts stay in the
file: the lifts are real, only the standing changes.

## Looking things up in the public API

When a source leaves you unsure about an athlete, a competition, a federation
or a spelling, query the project's own read-only API before asking the user:

- Docs: <https://api.openstreetlifting.org/swagger-ui/>
- Base: `https://api.openstreetlifting.org/api/v1`

```
GET /rankings?q=<name>&page_size=50
GET /athletes?page=1&page_size=50
GET /athletes/{slug}?include=competitions,records
GET /competitions?page=1&page_size=50&include=federation,movements
GET /competitions/{slug}?include=federation,results
```

Everything is a plain GET, no auth. Use it to answer: is this athlete already
known and how is their name spelled, does this competition already exist under
a slug, which movements did this federation run last year, is the person in
this post the same one who lifted in another competition.

`/rankings?q=` is the only name search there is: `/athletes` takes nothing but
pagination, and `/athletes/{slug}` needs the slug you are trying to work out.
See [Before writing a new athlete, look them up](#before-writing-a-new-athlete-look-them-up).

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

## Reading a FinalRep app screenshot

FinalRep meets are usually handed over as one PNG per weight class, named for the
class (`-73.png`). They are the best source there is for these competitions: three
attempts per movement, the counted best, the total and a provided RIS.

The colour code is the whole thing:

- **green** — made, but superseded by a later attempt
- **orange, boxed** — the counted best for that movement
- **red** — failed

All three red on a movement is a bomb; the row turns pink and the Place column reads
`Dis`. The header pill states sex and class outright (`Female -52kg`, `Male -94kg`), so
nothing needs inferring — and the two ladders are disjoint anyway.

What they do **not** carry is bodyweight. That only comes from the organiser, so these
meets are `Ris`-only rows.

Two traps. The app **truncates long names** in the display (`Camila Valenzuela Za…`),
and the cut-off surname is exactly what becomes the athlete's permanent key, so a
truncated name is something to ask about, never to write as-is. And its **Place column
is not always total order** — at the Chilean Open 2026 the two highest totals in `-73`
were listed 5th and 6th. Placings are recomputed on import, so this costs nothing, but
do not use the column to sanity-check your reading.

Screenshots are small. Crop the table into overlapping bands and upscale before
reading them, or the quarter-kilo increments (`26.25` vs `26.75`) blur.

## Check your reading before showing the diff

Every source that prints a total gives you a free proof: **the four `Best*` columns
`fmt` derived from your attempt cells must sum to the total the source printed**, row
by row. Run it over the whole file after `fmt`. It catches a misread colour, a dropped
attempt and a transposed digit in one pass, and it is the difference between "I read
the screenshots" and "the numbers reconcile".

Report the result in the summary. A row that will not reconcile is a finding to raise,
not a number to nudge.

## Aggregators are a cross-check, never a source

Third-party sites republish these results — `rankings.officialstreetlifting.com`,
`streetliftings.fr` / `.com`, calibase and friends. Ask the organiser or federation for
the official results instead, and offer to draft that message.

They are worth reading as a *cross-check*, and worth distrusting when they disagree
with the source. Both of the above have been caught being wrong in ways that do not
announce themselves:

- silently dropping an athlete's third attempt, leaving a total that still looks
  self-consistent
- printing a whole class's RIS as `0.00`
- printing `0.00` for bodyweight and `0` for RIS, meaning "not recorded"
- publishing best-per-movement with no attempt breakdown
- listing the *attempted* maxima of a disqualified athlete as if they had been made
- losing nationalities that the official source shows plainly

Their JSON endpoints are usually thinner than their HTML, so parse the page.

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
name = "Elite"                    # required, no federation and no year
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

### The name is the meet, nothing else

`name` carries the competition's own name and no more. The federation lives in
`[federation]` and the year lives in `start_date`, so repeating either in the name
duplicates a field the importer already has and makes the same meet read differently
from one season to the next.

| Instead of | Write |
| --- | --- |
| `DSN Dutch Streetlifting Nationals 2026` | `Dutch Streetlifting Nationals` |
| `SLI Italian National 2026` | `Italian National` |
| `Bodystrong UK & IRE Nationals 2026` | `UK & IRE Nationals` |
| `Brazil Open 2026` | `Brazil Open` |

The directory keeps both, because a slug has to stay unique across seasons and
federations: `dsn/2026/dsn-dutch-streetlifting-nationals-2026`. Only `name` is
trimmed.

Keep a number that is part of the meet's identity rather than its date, such as
`Australian Open Event 3` or `USA Streetlifting Nationals 3`. A source that brands the
year into the title, like `EUROS 24`, still gets the year dropped.

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
| `FirstName` `LastName` | Spelled the way the name is read, see below. `LastName` is required, `FirstName` is not |
| `Disambiguation` | Only to separate two real people sharing a name |
| `NativeName` | Only for a name not written in Latin. Omit the column otherwise |
| `Country` | Required, ISO 3166-1 alpha-2 |
| `BodyweightKg` | Never set alongside `Ris` |
| `Ris` | Only when the source gives a score and no bodyweight |
| `Status` | `competed`, `disqualified` or `no_show`. Empty means competed. A bombed movement means `disqualified`; an entrant who never lifted is `no_show` |
| `StatusReason` | Why they were disqualified |

`Country` is part of who an athlete *is*: the same name with a different
country imports as a different person. Use the country the source lists them
under, and keep it consistent for one person across competitions.

When the source gives an athlete **no** country at all — a grey globe in the
FinalRep app, an empty flag cell — write the **host country of the meet** rather
than holding the row back. `Country` is required, so the alternative is dropping a
real result, and at a national open the large majority are locals. Say in the PR body
which rows were defaulted, so the guess is on the record. This is the one column that
gets a default: bodyweights, attempts and RIS stay empty when unknown. Note the
consequence before doing it at scale — because the country is part of the identity
key, a later correction creates a new athlete rather than editing the existing one.

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

The standard ladder is the FinalRep one: `52` `57` `63` `70` `70+` for women and
`66` `73` `80` `87` `94` `101` `101+` for men. A competition running something
else, like `75`, is fine and stores its bound directly.

### A group the source does not classify

Some sources give a group no class at all, or a placeholder standing in for one.
USA Streetlifting print `D/C` on their early meets, over a group whose
bodyweights span several standard classes. Then, and only then, read the bound
off each athlete's bodyweight and put them in the standard class it falls in, so
a man at 95.9 kg goes in `101` and a woman at 69.9 kg in `70`.

Never do this to a class the source names. A lifter may enter above their own
class, so a 58.2 kg man the source lists under -73 stays in `73`, and a
federation running its own ladder keeps it.

Splitting an unclassified group has a cost worth saying out loud in the summary:
the source ranked those athletes against each other, and once they sit in
different classes each becomes their own winner, so the placings stop matching
the source's page. Say so rather than letting the user find it on the site.

A bound must be greater than zero, so there is no way to write a class that is
open at the bottom. Resolving per athlete against the ladder avoids ever needing
one.

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
name = "Nationals"
start_date = "2027-06-12"
end_date = "2027-06-12"
city = "Sevran"
country = "FR"
status = "upcoming"

[federation]
name = "FNSL"
country = "FR"
```

It lives at `backend/data/competitions/fnsl/2027/fnsl-nationals-2027/`, so the
year and federation live in the path while `name` stays `Nationals`. Include
`event` only if the calendar actually states the format.

Results land by **adding `entries.csv` to this same directory**, never by
starting a second one, so the slug has to be the one the competition will keep. When
they do, move `status` to `completed` in the same edit. No `entries.csv` and a
status other than `upcoming` is rejected, and so is an `entries.csv` sitting
next to `status = "upcoming"`.

## Names and who is who

Write the name the way it is **read**, not the way the source formatted it. A
registration sheet shouts `MERANDON`, an Instagram export styles it, a form
leaves it `merandon`; all three are written `Mérandon` here. The importer no
longer re-cases anything, so the file is the spelling that reaches the site, and
the validator rejects a name that breaks the rules below.

| Rule | Write it as |
|---|---|
| Every word starts with a capital, on both sides of a hyphen | `Anne-Sophie`, not `Anne-sophie` |
| Capitals inside a word are kept | `DeFrancesco`, `McDonald`, `D'Almeida` |
| Never in full capitals | `Gitton`, not `GITTON` |
| Particles stay lowercase away from the front | `Martina de Iturbe`, `Franck da Silva` |
| Accents are kept | `Mérandon`, `Clément`, `Bărbieru` |
| Latin letters only | a name in another alphabet goes in `NativeName`, see below |
| Only letters, space, `-`, `'` | no digits, `_`, brackets, emoji, styled or full-width letters |
| No nickname, handle or job title | `Loan Bernard-Bodier`, not `Loan "Seraf" Bernard-Bodier`; drop a trailing `PT` |
| Suffixes without a period, numerals in capitals | `Spigner IV`, `Morin B`, never `Jr.` |

Matching still ignores accents, capitalisation and punctuation, so a name
already in the database is not split by a source that spells it differently.
That is what makes it safe to write the correct spelling: never "fix" a name to
make it match one you have seen before.

### Before writing a new athlete, look them up

The folded name is the athlete's permanent key, and everything folding does not
cover splits a person in two. A reversed pair of columns or one wrong letter
does not fail the validator: it quietly creates a second athlete, halves both
their histories, and shows up in the diff as an ordinary new row. This has
already happened — `Chevillard Aubin` lifted a whole meet next to
`Aubin Chevillard`, and `Kuecuekyareli` spent three FinalRep meets apart from
`Kücükyareli`.

The rankings search runs over `first_name || ' ' || last_name`, so querying
**one half of the name** finds the athlete whichever way round they are stored:

```bash
curl -s "https://api.openstreetlifting.org/api/v1/rankings?q=chevillard&page_size=50"
```

Search the surname on its own, and the first name on its own when the surname
is the half you are unsure of. Read `athlete.first_name`, `athlete.last_name`
and `athlete.athlete_id` off every hit, then:

- **Same person, same order** — write the spelling the database already has.
- **Same person, reversed** — the source has its columns swapped. Write
  `FirstName` and `LastName` round the right way, matching the existing athlete.
- **Same person, spelled differently** — accents, case and punctuation fold
  together already and need nothing. Anything else is a real split: a
  transliterated `ue` for `ü`, a swapped letter, a middle name present in one
  meet and absent in the next. Say which spellings you found and ask which one
  is canonical.
- **A different person who shares the name** — that is what `Disambiguation`
  is for, under the rules below.
- **No hit** — a new athlete. Write what the source shows.

Two hits carrying the same name in different orders, under **different
`athlete_id`s**, are one person already split in two. That is a finding to
raise, not a reason to add a third spelling.

Worth doing for every athlete a source introduces, and not optional when the
source prints a name in one block (`CHEVILLARD Aubin`) and you are the one
deciding which half is the surname.

### A name that is not written in Latin

`FirstName` and `LastName` are always Latin, because identity, search and the
URL are built on them: nobody types `Радован Репац` into a search box, and the
day a meet romanises him he becomes a second athlete. Write the transliteration
there and keep the original in the optional `NativeName` column, which is
displayed beside it and never used to match anyone.

```
Sex,WeightClassKg,FirstName,LastName,Country,...,NativeName
M,80,Radovan,Repac,RS,...,Радован Репац
```

Leave the column out of a file where no athlete needs it. Cyrillic, Greek, Han,
Japanese and Korean are recognised, and the script is read off the characters
rather than declared. Transliteration is a judgement about a real person, so ask
rather than invent one: Serbian and Greek are mechanical, Chinese and Japanese
are not.

When the source writes surname first — `LEFEVRE Siméon` — put each half in its
own column the right way round. `FirstName` and `LastName` decide identity, so
swapping them makes a different person.

A source sometimes gives only one name. Put it in `LastName` and leave
`FirstName` empty, so the athlete reads and sorts as that one name. Never
repeat it in both fields: identity is decided by the folded name, so an invented
surname becomes their permanent key and splits their history the day a source
spells them in full.

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
bodyweight of zero or less; the same athlete twice in one class; a name that
breaks the spelling rules above; and a
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
