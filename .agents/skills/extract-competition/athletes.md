# Athlete identities

A spelling difference or swapped name silently creates a second athlete, so look up every athlete before writing them.

## Lookup

Search one name part at a time (surname, then first name) to catch swapped order, paging as needed:

```sh
curl -s 'https://api.openstreetlifting.org/api/v1/rankings?q=chevillard&page_size=50'
```

Each hit's `athlete` carries `first_name`, `last_name`, `country`, and `athlete_id`; details at `/athletes/{slug}?include=competitions,records`. Use the API for identity and country only, never to fill results. Report conflicts with the source, and report an unreachable API as unresolved checks.

| Finding | Action |
| --- | --- |
| Same person | Use the archive's names and, under the national default, its country |
| Names swapped in the source | Put each part in its column |
| Differs only in accents, case, punctuation | Keep the archive spelling |
| Different letters, transliteration, or extra names | Ask which spelling is canonical |
| Two archive IDs for one person | Report the split; confirm before treating as duplicates |
| No match | Use the source name |

## Spelling

- Normal case, including both hyphenated parts: `Anne-Sophie`, `Morin B`, `Spigner IV`, `Jr`.
- Keep internal capitals and evidenced accents: `McDonald`, `D'Almeida`, `Clément`. Lowercase particles: `Martina de Iturbe`.
- Latin letters, spaces, hyphens, apostrophes only; strip nicknames, handles, titles, emoji.
- A single name goes in `LastName`. A truncated name needs its full spelling first.
- Non-Latin names: established transliteration in the name columns, original in `NativeName` (added only when a row needs it, empty for redacted athletes). Ask about an uncertain transliteration.

## Disambiguation

Identity is folded name + sex + `Disambiguation`. Number confirmed different people with the same name and sex from `1`, keeping existing numbers. Spelling variants of one person and `CategorySex` never warrant it. Ask when it is unclear whether rows are one person.
