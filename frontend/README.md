# Frontend

The website presents Streetlifting results, rankings, and athlete records. For local setup, see the [repository README](../README.md#run-locally).

## Results

- Weights default to kilograms; visitors can choose pounds in the header. The choice is remembered for one year. Zero is a valid result; `-` means missing data.
- Unit conversion changes presentation only. Measurements and weight-class boundaries remain defined in kilograms; rankings and RIS scores stay the same.
- Competition formats can include different combinations of movements. Compare results within the same format and sex category.
- Competition dates keep their calendar day across time zones.
- Upcoming competitions appear before results are available. Results from ongoing competitions may be partial.

## RIS

RIS compares the combined muscle-up, pull-up, dips, and squat total across bodyweights within the same sex category. A score of 100 is a reference, not a maximum.

Scores use the current formula where possible, so they may differ from published results. A superscript **R** marks a published score that could not be recalculated. Missing bodyweights may be estimated from published RIS scores.

## Languages

The URL selects English (`/en`), French (`/fr`), German (`/de`), Italian (`/it`), or Polish (`/pl`). Switching language preserves the current page and filters.

The home page `/` follows the browser’s preferred supported language, falling back to English. Other page URLs without a language prefix redirect to English.

Athlete and competition names, published divisions, and event codes retain their source wording. Movement names are translated; RIS, All4, and MPDS stay unchanged.
