# Licensing

OpenStreetlifting keeps code and data under separate licences. The code is free
software under the
[GNU Affero General Public License v3](https://www.gnu.org/licenses/agpl-3.0.html).
The data is dedicated to the public domain under
[CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/).

## Which licence applies where

| Path or surface                                                                 | Licence                                               |
| ------------------------------------------------------------------------------- | ----------------------------------------------------- |
| `backend/data/competitions/`, every `competition.toml` and `entries.csv`        | CC0 1.0                                               |
| `backend/data/athletes/instagram.csv`                                           | CC0 1.0                                               |
| API responses from `api.openstreetlifting.org` and CSV downloads on the website | CC0 1.0                                               |
| `frontend/static/flags/`                                                        | CC BY 4.0, see [Third-party material](#third-party-material) |
| Everything else in the repository, including the rest of `backend/`, `frontend/`, `charts/` and `osl-bruno/` | AGPLv3 |

The API and the CSV downloads are built from `backend/data`, along with the
values computed at import such as totals, placings and RIS scores, so they carry
the same dedication as the files.

The full texts sit at the root of the repository:
[`LICENSE`](https://github.com/openstreetlifting/openstreetlifting/blob/main/LICENSE)
for the code and
[`LICENSE-DATA`](https://github.com/openstreetlifting/openstreetlifting/blob/main/LICENSE-DATA)
for the data.

## Using the data

CC0 puts no conditions on reuse. You can copy, modify, merge and redistribute the
data, commercially or not, without asking.

Some countries, France among them, do not let an author give up every right. CC0
covers this with a fallback: where the waiver does not hold, it grants an
unconditional licence to the same effect. It also waives the database right that
EU law gives to the maker of a database.

## CC0 and personal data

CC0 waives copyright and the database right. It cannot waive data protection
rights, and [says so itself](https://creativecommons.org/publicdomain/zero/1.0/legalcode):
the dedication does not affect the privacy or publicity rights of anyone whose
data the work describes.

The files name living people. Taking a copy makes you responsible for what you
then do with it, on your own account and under your own law, whatever the
licence allows. That responsibility is yours, not this project's, and CC0 does
not transfer it.

It also runs one way in time. An athlete who asks to be taken off the archive is
redacted here, but a snapshot somebody downloaded last year still carries their
name and nothing can reach into it. If you redistribute the data, take it fresh
rather than serving an old copy. See
[Personal Data](./PERSONAL_DATA.md).

## Credit

Credit is not required. It helps lifters find the archive and brings in the
corrections that keep it accurate, so if you publish something built on the data,
please mention:

> Data from the OpenStreetlifting project, [openstreetlifting.org](https://openstreetlifting.org)

## Contributing data

The archive stays public domain only if every contribution is. By opening a pull
request that adds or changes files under `backend/data/`, you dedicate that
contribution to the public domain under CC0 1.0.

## Sources

Results are compiled from federation result sheets, competition software such as
FinalRep and LiftControl, and public posts. Competition results are facts. CC0
covers OpenStreetlifting's compilation of them; it does not relicense a source's
own documents, photos or videos, which the
[`sources`](./DATA_REFERENCE.md#sources) key of each `competition.toml` points to.

## Third-party material

Country flags in `frontend/static/flags/` are
[Twemoji](https://github.com/jdecked/twemoji) graphics, copyright Twitter, Inc and
other contributors, licensed under
[CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).

Dependencies, including the Inter, JetBrains Mono and Instrument Serif fonts, keep
their own licences, stated in their packages.
