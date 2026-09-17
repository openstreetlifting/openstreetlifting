# Source-specific checks

## FinalRep screenshots

FinalRep screenshots often cover one weight class per image. Read sex and weight class from the header, such as `Female -52kg` or `Male -94kg`.

| Appearance | Meaning |
| --- | --- |
| Green attempt | Successful, superseded by a later attempt |
| Orange boxed attempt | Counted best for the movement |
| Red attempt | Failed attempt |
| All attempts red in one movement, pink row, `Dis` place | Disqualified after bombing the movement |

Inspect small tables at sufficient resolution. Overlapping enlarged views help distinguish quarter-kilogram increments such as `26.25` and `26.75`.

These screenshots typically show attempts, totals, and RIS without bodyweight. Record the supplied RIS and leave bodyweight empty when absent. Ask for the full spelling of truncated names. Treat a grey globe or missing flag using the country default in `format.md`.

Placings can disagree with total order. Reconcile the successful bests with the printed total; the importer computes placing. A `0 kg` squat, especially a row of zeros across all movements with no bodyweight or real attempts, can represent a nonstarter. Use the no-show rules in `format.md` and resolve ambiguous markings before writing attempts.

## Third-party aggregators

Use official organiser or federation results as extraction sources. Third-party rankings sites, including Official Streetlifting, Streetliftings, and Calibase, can cross-check the extraction but do not replace official evidence.

If only an aggregator is available, ask for official results or offer to draft a request to the organiser. Sending that request requires the user's instruction.

Check for omitted attempts or nationalities, placeholder zero bodyweights and RIS scores, best-only summaries, and failed maxima presented as successful lifts. Inspect the HTML when an aggregator's JSON omits fields. Report disagreements against the official source; keep aggregator-only values out of the canonical files.
