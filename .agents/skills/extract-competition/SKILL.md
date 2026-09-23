---
name: extract-competition
description: Extract Streetlifting results or competition announcements from web pages, PDFs, screenshots, or Instagram into backend/data/competitions. Use when adding a competition or extending its existing files from new source material.
---

# Extract competition data

Create or extend one canonical directory per competition. Each pass must produce a reviewable diff supported by the supplied sources.

## Workflow

1. **Locate the competition.** Search `backend/data/competitions` for an existing directory and read its files. Keep its slug. For a new competition, use `<federation>/<start-year>/<competition-slug>/`; the federation is slugified, and the competition slug must distinguish it across federations and years. The directory name supplies the slug, so keep it out of `competition.toml`.

2. **Read the applicable rules.** Read [format.md](format.md) before writing competition metadata or results. For results, also read [athletes.md](athletes.md) before assigning identities. For Instagram carousels, FinalRep screenshots, or third-party aggregators, read [sources.md](sources.md). For an announcement without results, follow the announcement section in `format.md`.

3. **Extract the evidence.** Record only values supported by the source, subject to the unclassified-group rules in `format.md`. Leave unknown optional values empty. Keep failed attempts. Resolve unclear digits, colours, names, and conflicting sources before writing the affected values. Ask the user when the available evidence cannot settle them; continue with independent rows.

4. **Resolve athlete identities.** Look up every athlete introduced by this source using the procedure in `athletes.md`. Complete this before writing their name. Report unresolved matches and existing duplicate identities. The API is a projection of imported files, not evidence for missing results.

5. **Merge the new material.** Add rows and fill previously empty cells. Preserve existing data outside the source's scope. If a source establishes an error in an existing value, explain the correction and its evidence. Append source references to `competition.toml` without dropping earlier ones.

6. **Prepare and validate.** From `backend`, substitute the competition's actual path:

   ```sh
   cargo run -p osl_importer --bin import -- prepare data/competitions/<federation>/<year>/<slug>
   cargo run -p osl_importer --bin import -- competitions data/competitions/<federation>/<year>/<slug> --dry-run
   ```

   Fix errors and repeat. Warnings about missing evidence can remain; report them instead of inventing values. A nonempty suppression list requires its existing `OSL_PRIVACY_KEY`. If it is unavailable, use `--dry-run --skip-privacy-check` for structural validation and report that suppression checks remain outstanding. Never generate a replacement key for an existing list or bypass a reported suppression match.

7. **Reconcile the results.** For each source row with a total, verify `TotalKg` against it. Keep any available lift breakdown; a complete breakdown must agree with the total. Account for every row in the supplied source: imported, already present, or unresolved. Report mismatches without adjusting weights to force agreement. Check the diff for changes outside the intended competition and source categories.

8. **Report the diff.** Show the changed files and summarize added categories or athletes, validation and total checks, missing countries, regrouped weight classes, and unresolved evidence. Leave the files ready for review. Database imports and commits are outside this extraction workflow unless the user explicitly requests them.

## Completion

A pass is complete when every supplied row is accounted for, all written values have evidence or a documented allowed default, validation errors are resolved, and remaining warnings or blocked rows are reported. If required evidence is missing, identify exactly what would resolve it.
