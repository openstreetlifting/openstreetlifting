# Canonical data rules

The parser and validator define the accepted schema. For fields beyond this guide, inspect `backend/crates/osl_importer/src/canonical/{competition,entries,validator}.rs` and the [data contribution guide](../../../backend/docs/src/CONTRIBUTING_DATA.md) before writing them.

## Competition metadata

Use `competition.toml` for metadata and source references:

```toml
event = "MPDS"
sources = ["https://www.instagram.com/p/xxxx/"]

[competition]
name = "Elite"
start_date = "2026-05-15"
end_date = "2026-05-17"
city = "Sevran"
region = "Île-de-France"
country = "FR"
status = "completed"

[federation]
name = "FNSL"
abbreviation = "FNSL"
country = "FR"
```

Competition name, start and end dates, country, and federation name are required. Omit unknown optional fields. Dates are quoted strings; country codes use ISO 3166-1 alpha-2, and regions use ISO 3166-2 subdivision names. Keep the start year consistent with the directory. Unknown keys, including a version key, are rejected.

The competition name identifies the meet without repeating its federation or year: `Dutch Streetlifting Nationals`, not `DSN Dutch Streetlifting Nationals 2026`. Keep edition numbers that are part of the identity, such as `Australian Open Event 3`. Remove years even from titles such as `EUROS 24`. The directory slug retains the identifying federation and year.

`event` lists contested movements in `MPDS` order: muscle-up, pull-up, dips, squat. Examples: `MPDS`, `DS`, `M`. Each letter appears at most once. Other movements require a schema change; report that limit before attempting an import. Only four-movement events have overall totals and RIS; partial events are ranked per movement.

Competition status is `draft`, `upcoming`, `live`, `completed`, or `cancelled`.

### Announcements

For a scheduled competition without results, write only `competition.toml` with `status = "upcoming"`. Include `event` only when the source states the format. Choose the permanent directory and slug now.

When results arrive, add `entries.csv` in that directory and change the status to `completed`. An upcoming competition cannot have an entries file; a competition without entries must be upcoming.

## Result rows

Write one row per athlete per category in `entries.csv`. The base header is one line:

```csv
Sex,WeightClassKg,FirstName,LastName,Disambiguation,Country,BodyweightKg,Ris,Status,StatusReason,MuscleUp1Kg,MuscleUp2Kg,MuscleUp3Kg,BestMuscleUpKg,PullUp1Kg,PullUp2Kg,PullUp3Kg,BestPullUpKg,Dips1Kg,Dips2Kg,Dips3Kg,BestDipsKg,Squat1Kg,Squat2Kg,Squat3Kg,BestSquatKg
```

Add `Division` first only when needed; add `NativeName` only when an athlete has a non-Latin name. Preserve any supported provenance columns already in the file. Let `fmt` set canonical column order.

| Field | Rule |
| --- | --- |
| `Sex` | Required: `M`, `F`, or `MX` |
| `WeightClassKg` | Required positive bound: `80` for −80, `101+` for +101 |
| `FirstName`, `LastName` | Apply `athletes.md`; last name required, first name optional |
| `Disambiguation` | Only for distinct people sharing identity fields; positive integer |
| `Country` | Required two-letter code; apply the exception below when absent |
| `BodyweightKg` | Positive measured bodyweight supplied by the source |
| `Ris` | Source-reported score when bodyweight is absent |
| `Status` | `competed`, `disqualified`, or `no_show`; empty means competed |
| `StatusReason` | Source-supported reason for disqualification |

During extraction, use either reported bodyweight or reported RIS, not both. Leave both empty if neither is supplied. Bodyweight recovery is a separate workflow; preserve existing recovery evidence rather than recomputing or replacing it.

### Country

Use the athlete's source-listed country. When the source omits it, use the host country and report every defaulted row in the summary or PR description. This is the permitted country fallback, not evidence of nationality. Missing bodyweights, attempts, and RIS do not receive defaults.

Country participates in identity: correcting it can create a different athlete. Report conflicts for a known person before applying this fallback across many rows.

### Divisions and weight classes

Add `Division` only when the competition runs separate contests for the same sex and weight class, such as Elite and Open. Use the source's label consistently and populate every row once the column exists. A meet that is itself one division needs no division column.

Placings are computed within each division. An athlete can enter multiple divisions; global rankings ignore division.

Read the weight-class bound from the source's category, even when an athlete weighs less than its limit. Keep federation-specific classes. Write `80` or `101+`, never `-80` or `+101`; leading signs can be interpreted as spreadsheet formulas.

If the source gives no class or uses a placeholder such as `D/C`, assign each athlete to the standard class containing their reported bodyweight:

- Women: `52`, `57`, `63`, `70`, `70+`.
- Men: `66`, `73`, `80`, `87`, `94`, `101`, `101+`.

Apply this only to unclassified groups. Ask when bodyweight or an applicable ladder is missing. Report that regrouping changes the source's shared standings into separate category placings.

## Attempts and status

| Cell | Meaning |
| --- | --- |
| `100` | Successful lift at 100 kg |
| `100x` | Failed attempt at 100 kg |
| `0` | Successful lift with no added weight |
| `0x` | Failed attempt with no added weight |
| Empty | Not attempted or unknown |

Crossed-out or red attempts remain in the file with an `x` suffix. Weights are nonnegative. For muscle-ups, pull-ups, and dips, zero can be a real attempt; read the success or failure marking rather than treating it as missing.

A zero used as a nonstarter placeholder is not an attempt. A row showing a zero squat, no bodyweight, and no actual lifts—often zeros across every movement—belongs to `no_show`, with empty attempt cells. Compare it with recorded failed attempts and ask if the distinction remains unclear. A no-show cannot carry a lift.

When every attempt in a contested movement failed, preserve those attempts and mark the athlete `disqualified`. Leave its best empty. An athlete who attempted lifts is not a no-show. Report conflicting status evidence; the validator rejects a bombed athlete left as competed.

`fmt` derives each `Best*` value from supplied attempts. Fill a best manually only when the source gives a best without an attempt breakdown; keep those attempt cells empty. For movements outside `event`, leave all four cells empty.

Totals, ranks, and placings are computed on import and have no CSV columns. RIS is computed on import when bodyweight is available; a source-reported score belongs in `Ris` only under the rule above.

## Validation

Resolve parser and validator errors before completing the pass. Do not turn missing evidence into fabricated values to silence warnings. Missing city, missing bodyweight/RIS, rows without lifts, and an athlete entered in multiple classes can be legitimate warnings during extraction.
