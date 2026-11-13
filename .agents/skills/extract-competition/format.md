# Canonical data rules

The schema lives in `backend/crates/osl_importer/src/canonical/{competition,entries,validator}.rs` and [CONTRIBUTING_DATA.md](../../../backend/docs/src/CONTRIBUTING_DATA.md); check them for any field not covered here.

## competition.toml

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

- Required: competition name, start date, country, federation name, `sources`. Omit unknown optional fields and `end_date` for one-day events. Add `venue` when the source names it.
- Country codes are ISO 3166-1 alpha-2; regions are ISO 3166-2 subdivision names.
- `name` drops the federation and year (`Dutch Streetlifting Nationals`, `EUROS` from `EUROS 24`) but keeps identifying edition numbers (`Australian Open Event 3`).
- `event` lists contested movements in `MPDS` order: `MPDS` (All4), `PD` (Classic), also `PDS`, `DS`, `M`. Uncontested movement cells stay empty.
- RIS: calculated for `MPDS`. A shorter event ranks by published RIS only with `scoring = "ris"` and no weight classes anywhere; see [shorter-event RIS](../../../backend/docs/src/DATA_REFERENCE.md#ris-for-fewer-than-four-lifts).
- `status`: `draft`, `upcoming`, `live`, `completed`, `cancelled`.

**Announcements** get only `competition.toml` with `status = "upcoming"`, and `event` only when stated. Choose the permanent slug now. When results arrive, add `entries.csv` and set `completed`.

## entries.csv

One row per athlete per category:

```csv
Sex,WeightClassKg,FirstName,LastName,Disambiguation,Country,BodyweightKg,ReportedRis,TotalKg,Status,StatusReason,MuscleUp1Kg,MuscleUp2Kg,MuscleUp3Kg,BestMuscleUpKg,PullUp1Kg,PullUp2Kg,PullUp3Kg,BestPullUpKg,Dips1Kg,Dips2Kg,Dips3Kg,BestDipsKg,Squat1Kg,Squat2Kg,Squat3Kg,BestSquatKg
```

| Field | Rule |
| --- | --- |
| `Sex` | `M` or `F`, from the source, never inferred from a name |
| `CategorySex` | `MX` for a mixed contest; empty follows `Sex` |
| `WeightClassKg` | Category bound as `80` or `101+`, never `-80` or `+101` |
| `FirstName`, `LastName`, `Disambiguation`, `NativeName` | See `athletes.md` |
| `Country` | See [Country](#country) |
| `BodyweightKg` | Measured bodyweight from the source |
| `ReportedRis` | Source score, with or without bodyweight; `ReportedRisEdition` only when the formula is established |
| `TotalKg` | Published total; `prepare` fills it only from a complete breakdown |
| `Status` | `competed` (or empty), `disqualified`, `no_show` |
| `StatusReason` | Source reason for a disqualification |

Keep existing provenance columns; `prepare` sets column order.

### Country

Use the source-listed country. When the source omits it:

- **Domestic competition** (any event of a national federation or organiser in its own country: nationals, regionals, series, qualifiers, local classics such as FNSL Interrégionales, AESL Ragnarok Battles or SLI Street): default every athlete to the host country. An athlete whose archive country (API `athlete.country` or existing entries) differs keeps that country.
- **International competition** (opens, undergrounds, Euros, Worlds, expo events such as Arnold Classic): leave the cell empty. The host country is not nationality.

Conflicting countries for one athlete block import; resolve against the source and update the affected entries together.

### Categories

- Read the class from the source category, not the bodyweight. Keep federation-specific classes. A placeholder such as `D/C` is not a class; resolve it.
- No classes anywhere: omit `WeightClassKg`. Classes in some contests only: keep the column, leave unclassed rows empty.
- `Division` (first column) only for separate contests sharing sex and class, such as Elite and Open; fill it on every row. A meet that is one division needs none. Placings are per division.
- Mixed contests: real `Sex` on every row, `CategorySex=MX`, and `scoring = "total"` or `"ris"` under `[competition]`, established from the source. Report contests using different or unsupported scoring before importing. Placing rules: [DATA_REFERENCE.md](../../../backend/docs/src/DATA_REFERENCE.md#athlete-sex-and-mixed-contests).

## Attempts and status

| Cell | Meaning |
| --- | --- |
| `100` | Success at 100 kg |
| `100x` | Failure at 100 kg |
| `0` / `0x` | Success / failure with no added weight |
| Empty | Not attempted or unknown |

- Red or crossed-out attempts are failures (`x`). A bodyweight-only `0` on M, P, or D is a real attempt.
- **No-show**: a zero squat with no bodyweight and no real lifts, often zeros everywhere. `Status=no_show`, attempt cells empty, no total or positive RIS. Ask when it could be failed attempts instead.
- **Bomb-out**: every attempt in a contested movement failed. Keep the attempts, `Status=disqualified`, leave that best and `TotalKg` empty, keep any published RIS.
- `prepare` derives `Best*` from attempts. Write a best by hand only when the source gives bests without attempts.
- Record a published `TotalKg` alongside known attempts. A value is a total only when the source or user says so.

## Bodyweight recovery

Only on request: run `backend/scripts/recover-bodyweight` with the total or full breakdown and an established RIS edition. Write `BodyweightKg` with `BodyweightSource=recovered`, keep `ReportedRis` and `ReportedRisEdition`, and call the results estimates. Preserve existing recovery evidence.
