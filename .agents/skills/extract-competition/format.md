# Canonical data rules

The parser and validator define the accepted schema. For fields beyond this guide, inspect `backend/crates/osl_importer/src/canonical/{competition,entries,validator}.rs` and the [data contribution guide](../../../backend/docs/src/CONTRIBUTING_DATA.md) before writing them.

## Competition metadata

Use `competition.toml` for metadata. Include at least one source URL, archived file path, or document description identifying the original evidence:

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

Competition name, start date, country, federation name, and a nonempty `sources` list are required. Omit `end_date` for a one-day event; it defaults to `start_date`. Set `venue` to the source-listed venue name when available. Omit unknown optional fields. Dates are quoted strings; country codes use ISO 3166-1 alpha-2, and regions use ISO 3166-2 subdivision names. Keep the start year consistent with the directory. Unknown keys, including a version key, are rejected.

The competition name identifies the meet without repeating its federation or year: `Dutch Streetlifting Nationals`, not `DSN Dutch Streetlifting Nationals 2026`. Keep edition numbers that are part of the identity, such as `Australian Open Event 3`. Remove years even from titles such as `EUROS 24`. The directory slug retains the identifying federation and year.

`event` lists contested movements in `MPDS` order: muscle-up, pull-up, dips, squat. Examples: `MPDS`, `DS`, `M`. Each letter appears at most once. Other movements require a schema change; report that limit before attempting an import. Any supported event may have a total across its declared movements. Only the four-movement `MPDS` event supports RIS.

Competition status is `draft`, `upcoming`, `live`, `completed`, or `cancelled`.

### Announcements

For a scheduled competition without results, write only `competition.toml` with `status = "upcoming"`. Include `event` only when the source states the format. Choose the permanent directory and slug now.

When results arrive, add `entries.csv` in that directory and change the status to `completed`. An upcoming competition cannot have an entries file; a competition without entries must be upcoming.

## Result rows

Write one row per athlete per category in `entries.csv`. The base header is one line:

```csv
Sex,WeightClassKg,FirstName,LastName,Disambiguation,Country,BodyweightKg,ReportedRis,TotalKg,Status,StatusReason,MuscleUp1Kg,MuscleUp2Kg,MuscleUp3Kg,BestMuscleUpKg,PullUp1Kg,PullUp2Kg,PullUp3Kg,BestPullUpKg,Dips1Kg,Dips2Kg,Dips3Kg,BestDipsKg,Squat1Kg,Squat2Kg,Squat3Kg,BestSquatKg
```

Add `Division` first only when needed; add `NativeName` only when an athlete has a non-Latin name. Preserve any supported provenance columns already in the file. Let `prepare` set canonical column order.

| Field | Rule |
| --- | --- |
| `Sex` | Required athlete sex: `M` or `F` |
| `CategorySex` | Optional; `MX` for a mixed contest, empty to follow `Sex` |
| `WeightClassKg` | Omit for a meet without classes; otherwise fill every row with a positive bound, such as `80` or `101+` |
| `FirstName`, `LastName` | Apply `athletes.md`; last name required, first name optional |
| `Disambiguation` | Only for distinct people sharing identity fields; positive integer |
| `Country` | Source-listed two-letter code; leave empty when unknown |
| `BodyweightKg` | Positive measured bodyweight supplied by the source |
| `ReportedRis` | Source-reported score, whether or not bodyweight is known |
| `TotalKg` | Overall total from the source; `prepare` fills it only from a complete event breakdown |
| `Status` | `competed`, `disqualified`, or `no_show`; empty means competed |
| `StatusReason` | Source-supported reason for disqualification |

Record both bodyweight and `ReportedRis` when the source supplies both. Leave unknown values empty. Set `ReportedRisEdition` only when the source formula is established. The validator compares scores at two decimal places when the edition, complete All4 total, and M or F formula are known; resolve contradictions against the source before import. Unverifiable scores remain published evidence with a validation warning. Bodyweight recovery is a separate workflow; preserve existing recovery evidence rather than recomputing or replacing it. Use `ReportedRis`; the former `Ris` header is rejected.

### Country

Use the athlete's source-listed country. Keep the `Country` header and leave the cell empty when the source omits it. The host country is not evidence of nationality.

Country is independent of identity. Check existing entries for the same athlete: conflicting known countries block import. Resolve the source conflict and update the affected entries together.

### Divisions and weight classes

Add `Division` only when the competition runs separate contests for the same sex and weight class, such as Elite and Open. Use the source's label consistently and populate every row once the column exists. A meet that is itself one division needs no division column.

Placings are computed within each division. An athlete can enter multiple divisions; global rankings ignore division.

Read the weight-class bound from the source's category, even when an athlete weighs less than its limit. Keep federation-specific classes. Write `80` or `101+`, never `-80` or `+101`; leading signs can be interpreted as spreadsheet formulas.

When the competition has no weight classes, omit `WeightClassKg`. Preserve its
shared standings. A placeholder such as `D/C` does not establish a weight class;
resolve its meaning against the source before assigning a category.

### Mixed contests

Read athlete sex and contest membership separately. Keep `Sex=M` or `Sex=F` on
every entry; use `CategorySex=MX` for mixed entries. An omitted or empty
`CategorySex` follows `Sex`. Confirm missing athlete sex from a source rather
than inferring it from a name or mixed-category label.

For mixed contests, record `scoring = "total"` or `scoring = "ris"` under
`[competition]`. This rule applies to all contests in the competition. Establish
it from the source; if contests use different methods or an unsupported formula,
report the unsupported format before importing. The [data reference](../../../backend/docs/src/DATA_REFERENCE.md#athlete-sex-and-mixed-contests)
defines OSL's placing and tie rules.

## Attempts and status

| Cell | Meaning |
| --- | --- |
| `100` | Successful lift at 100 kg |
| `100x` | Failed attempt at 100 kg |
| `0` | Successful lift with no added weight |
| `0x` | Failed attempt with no added weight |
| Empty | Not attempted or unknown |

Crossed-out or red attempts remain in the file with an `x` suffix. Weights are nonnegative. For muscle-ups, pull-ups, and dips, zero can be a real attempt; read the success or failure marking rather than treating it as missing.

A zero used as a nonstarter placeholder is not an attempt. A row showing a zero squat, no bodyweight, and no actual lifts—often zeros across every movement—belongs to `no_show`, with empty attempt cells. Compare it with recorded failed attempts and ask if the distinction remains unclear. A no-show cannot carry a lift, total, or positive RIS. A published zero RIS may stay as source evidence.

When every attempt in a contested movement failed, preserve those attempts and mark the athlete `disqualified`. Leave that movement's best and the overall total empty. Preserve any published RIS as source evidence; the result is unranked. An athlete who attempted lifts is not a no-show. Report conflicting status evidence; the validator rejects a bombed athlete left as competed.

`prepare` derives each `Best*` value from supplied attempts. Fill a best manually only when the source gives a best without an attempt breakdown; keep those attempt cells empty. For movements outside `event`, leave all four cells empty.

### Published totals

For a competed result, record the published `TotalKg` alongside any known attempts or bests. `prepare` fills a missing total only when every event movement has a successful best; it rejects contradictions without replacing the supplied total. Leave unknown totals empty when the breakdown is incomplete. Import requires a stored total for complete breakdowns. Rankings and pages use the stored total; All4 results with bodyweight can also receive calculated RIS. Preserve a published `ReportedRis` even when it cannot be verified.

### Bodyweight recovery

When the user requests bodyweight recovery, use the published total or complete lift breakdown with the recovery tool in `backend/scripts/recover-bodyweight`. Establish the RIS edition from the source or by checking known bodyweight–total–score combinations. Write the recovered estimate to `BodyweightKg` and preserve `ReportedRis`, `ReportedRisEdition`, and `BodyweightSource=recovered`. Describe recovered weights as estimates because published scores are rounded. Keep `competition.toml` sources limited to source references.

## Validation

Resolve parser and validator errors before completing the pass. Do not turn missing evidence into fabricated values to silence warnings. Missing city, missing bodyweight/RIS, rows without lifts, and an athlete entered in multiple classes can be legitimate warnings during extraction.
