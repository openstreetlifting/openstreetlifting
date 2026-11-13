---
name: extract-competition
description: Extract Streetlifting results or announcements from web pages, PDFs, screenshots, or Instagram into backend/data/competitions. Use when adding a competition or extending one from new sources.
---

# Extract competition data

Every written value is backed by the source or by a default stated in these files. When evidence cannot settle a value, ask and carry on with independent rows.

1. **Locate.** Search `backend/data/competitions` for the competition and read its files, keeping its slug. A new one goes in `<federation-slug>/<start-year>/<competition-slug>/`; the directory supplies the slug.
2. **Read the rules.** [format.md](format.md) always; [athletes.md](athletes.md) for results; [sources.md](sources.md) for Instagram carousels, FinalRep screenshots, or third-party aggregators.
3. **Extract** each row from the source, keeping failed attempts.
4. **Resolve identities** of every athlete the source introduces, per `athletes.md`, before writing their name or country.
5. **Merge.** Add rows and fill empty cells; leave existing data outside the source's scope untouched. Explain any correction with its evidence. Append to `sources`, keeping earlier entries.
6. **Validate** from `backend` until there are no errors:

   ```sh
   cargo run -p osl_importer --bin import -- prepare data/competitions/<path>
   cargo run -p osl_importer --bin import -- competitions data/competitions/<path> --dry-run
   ```

   Warnings about missing evidence stay and are reported. A nonempty suppression list needs the existing `OSL_PRIVACY_KEY`; without it, add `--skip-privacy-check` and report suppression checks as outstanding. Never generate a new key or bypass a suppression match.
7. **Reconcile.** Every source row is imported, already present, or reported as unresolved. Each `TotalKg` matches the source total and any complete breakdown; report mismatches as they are. The diff touches only this competition.
8. **Report** changed files, added categories and athletes, validation and total checks, defaulted or missing countries, mixed-contest scoring, and unresolved evidence. Import and commit only on request.
9. **PR, on request:** one branch and PR per competition, with the one-line body `Import [<competition>](<source URL>) (closes #<issue>).`, dropping the closing reference without an issue.
