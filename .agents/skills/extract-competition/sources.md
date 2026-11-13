# Source-specific checks

## Instagram carousels

When a text fetch returns only the cover (`og:image` always shows it), use headless Playwright:

1. Open `?img_index=1`, click `Decline optional cookies`, and close signup overlays. A login wall means asking the user for the images.
2. Advance with `getByRole('button', { name: 'Next', exact: true })`, checking the heading and `img_index` after each move.
3. Instagram preloads neighbours: take the image inside the carousel viewport, wait for `complete` and nonzero `naturalWidth`, then download its `currentSrc` and screenshot it.
4. Keep captures and a slide-to-file manifest outside the repo. Read the caption and every slide; deduplicate athletes repeated across summary and category tables.

Cite the post URL, not CDN URLs, which expire. The post date is not the competition date. Report anomalies such as RIS cells repeating totals or bodyweights above the class.

## FinalRep screenshots

One weight class per image; sex and class come from the header (`Female -52kg`). No bodyweight, so record the RIS and leave bodyweight empty.

| Appearance | Meaning |
| --- | --- |
| Green | Success, superseded |
| Orange box | Counted best |
| Red | Failure |
| All red in a movement, pink row, `Dis` | Bomb-out |

Zoom in to tell quarter kilos apart (`26.25` vs `26.75`). A grey globe means unknown country. Ask for truncated names. Printed placings can disagree with totals; reconcile bests against the total, since the importer computes placing.

## Third-party aggregators

Official Streetlifting, Streetliftings, Calibase and similar sites only cross-check. When nothing official exists, ask the user for official results or offer to draft a request to the organiser, sent only on their say. Watch for missing attempts or countries, placeholder zero bodyweights and RIS, best-only summaries, and failed maxima shown as successes; inspect the HTML when the JSON omits fields.
