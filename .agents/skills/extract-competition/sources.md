# Source-specific checks

## Instagram carousels

If a text fetch fails or returns only the cover, try Playwright with an installed headless browser. Public carousels may render without login. The HTML `og:image` preview can show the cover regardless of `img_index`.

1. Open `?img_index=1`, wait for images, and dismiss the cookie dialog with `Decline optional cookies` and any dismissible signup overlay. If a login wall or challenge blocks access, request the images or another official source.
2. Capture each slide, advancing with `page.getByRole('button', { name: 'Next', exact: true })`. Verify the visible heading and `img_index` after each transition, including when navigating directly to an index.
3. Save a screenshot and download the visible image's `currentSrc` through the browser context's request client when accessible. Instagram preloads adjacent slides: select the image inside the carousel viewport. Wait for `complete` and a nonzero `naturalWidth`, then inspect the downloaded image.
4. Keep captures and a slide-to-file manifest outside the canonical directory. Read the caption and every requested slide. Deduplicate athletes repeated in summary and category tables.

Use the stable Instagram post URL in `competition.toml` sources; CDN image URLs expire. The publication date is not the competition date. Report source discrepancies such as RIS cells repeating totals or a bodyweight exceeding the displayed class.

For totals without lift results, follow [Published totals](format.md#published-totals). Empty rows need status evidence before assigning disqualification or no-show. Record a value as a total only when the source or user identifies it as one.

## FinalRep screenshots

FinalRep screenshots often cover one weight class per image. Read sex and weight class from the header, such as `Female -52kg` or `Male -94kg`.

| Appearance | Meaning |
| --- | --- |
| Green attempt | Successful, superseded by a later attempt |
| Orange boxed attempt | Counted best for the movement |
| Red attempt | Failed attempt |
| All attempts red in one movement, pink row, `Dis` place | Disqualified after bombing the movement |

Inspect small tables at sufficient resolution. Overlapping enlarged views help distinguish quarter-kilogram increments such as `26.25` and `26.75`.

These screenshots typically show attempts, totals, and RIS without bodyweight. Record the supplied RIS and leave bodyweight empty when absent. Ask for the full spelling of truncated names. Leave `Country` empty for a grey globe or missing flag, as described in `format.md`.

Placings can disagree with total order. Reconcile the successful bests with the printed total; the importer computes placing. A `0 kg` squat, especially a row of zeros across all movements with no bodyweight or real attempts, can represent a nonstarter. Use the no-show rules in `format.md` and resolve ambiguous markings before writing attempts.

## Third-party aggregators

Use official organiser or federation results as extraction sources. Third-party rankings sites, including Official Streetlifting, Streetliftings, and Calibase, can cross-check the extraction but do not replace official evidence.

If only an aggregator is available, ask for official results or offer to draft a request to the organiser. Sending that request requires the user's instruction.

Check for omitted attempts or nationalities, placeholder zero bodyweights and RIS scores, best-only summaries, and failed maxima presented as successful lifts. Inspect the HTML when an aggregator's JSON omits fields. Report disagreements against the official source; keep aggregator-only values out of the canonical files.
