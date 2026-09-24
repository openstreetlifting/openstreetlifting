# Athlete identities

## Look up each introduced athlete

Use the read-only API at `https://api.openstreetlifting.org/api/v1`. Search one part of the name to catch reversed first and last names:

```sh
curl -s 'https://api.openstreetlifting.org/api/v1/rankings?q=chevillard&page_size=50'
```

Search the surname first; try the first name when the surname is uncertain. Inspect `athlete.first_name`, `athlete.last_name`, and `athlete.athlete_id` on each hit. Check additional pages when needed. `/rankings?q=` supports name searches; `/athletes` supports pagination only. Athlete details are available at `/athletes/{slug}?include=competitions,records`.

| Finding | Action |
| --- | --- |
| Same person and spelling | Use the established first and last names |
| Source reverses the names | Put each part in the correct column |
| Difference only in accents, case, or punctuation | Keep the supported display spelling; matching folds these differences |
| Different letters, transliteration, or added/omitted names | Report both spellings and ask which is canonical |
| Confirmed different people with the same identity fields | Use disambiguation as described below |
| No matching athlete | Use the source's name |
| API unavailable | Report the lookup failure and unresolved identity checks |

A spelling difference or reversed name can create a second athlete without a validation error. Two IDs with reversed names may reveal an existing split; confirm the person before treating them as duplicates.

The API records what the project has imported. Use it to resolve identities and locate competitions, not to fill missing bodyweights, attempts, RIS scores, or countries. If it conflicts with the source, report the conflict and ask. The API reference is at <https://api.openstreetlifting.org/swagger-ui/>.

## Display spelling

The importer preserves the file's spelling. Write names in their normal readable form:

- Capitalize words and both parts of hyphenated names: `Anne-Sophie`.
- Preserve internal capitals and supported accents: `DeFrancesco`, `McDonald`, `D'Almeida`, `Clément`.
- Use lowercase particles inside names: `Martina de Iturbe`, `Franck da Silva`.
- Convert all-capital names to normal case; retain uppercase initials and suffix numerals such as `Morin B` and `Spigner IV`.
- Use Latin letters, spaces, hyphens, and apostrophes. Remove nicknames, handles, titles, emoji, and decorative typography.
- Write suffixes without periods, such as `Jr`.

Preserve accents established by the evidence; changing case does not justify inventing an accent. Keep a single supplied name in `LastName` and leave `FirstName` empty. A truncated name needs the missing text before it can become an identity.

## Native names

Use an established Latin transliteration in `FirstName` and `LastName`. Store the original spelling in the optional `NativeName` column when it uses Cyrillic, Greek, Han, Japanese, or Korean characters. Omit that column when no row needs it.

`NativeName` is displayed but does not determine identity. Ask for an uncertain transliteration instead of creating a new spelling. Keep native names empty for redacted athletes.

## Disambiguation

Identity uses the folded name, sex, and disambiguation number. Check existing entries and preserve assigned numbers. For confirmed different people with the same name and sex, number them from `1`, regardless of country. A mixed-category entry keeps the athlete's sex and identity; `CategorySex` does not create another person.

Two spellings of one person's name do not justify disambiguation. If the evidence does not establish whether the rows represent different people, ask.
