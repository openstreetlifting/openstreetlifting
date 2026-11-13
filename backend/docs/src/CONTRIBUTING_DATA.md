# Contributing Data

If you know Git, you can add or
correct the archive's files and open a pull request on
[GitHub](https://github.com/openstreetlifting/openstreetlifting). Otherwise,
send me the results through the [contact page](https://openstreetlifting.org/contact).

## Before you start

### What i consider a source

A PDF, a scoresheet photo, a competition livestream or a Instagram post can all be useful. The source should identify the competition and
let me check the results. Please only use official federation material.

## Adding a competition

Each competition has a folder under
`backend/data/competitions/{federation}/{year}/{slug}/`. A _slug_ is a short name
used to identify the competition, such as `worlds-2023`.

The folder contains two files:

- `competition.toml`: the competition's name, dates, location and source links.
- `entries.csv`: the athletes and their results.

See, for example the [FinalRep Worlds 2023 folder on GitHub](https://github.com/openstreetlifting/openstreetlifting/tree/main/backend/data/competitions/finalrep/2023/worlds-2023)

### Validate it locally

Refer to Github readme instructions to start the project locally and import the competition

### Open a pull request

Open a pull request in the
[project repository](https://github.com/openstreetlifting/openstreetlifting).

## Correcting results that are already published

You can report a mistake through the
[contact page](https://openstreetlifting.org/contact). Include a link to the
competition or athlete, explain what should change and share a supporting source.

To make the correction yourself, edit the competition's existing files and open a
pull request as described above.

## Review checklist

Before submitting files, check that:

- You are adding a new competition or updating the existing entry.
- The results match your sources, and `competition.toml` includes their references.
- You have pointed out missing details, athletes who share a name, or unusual
  weight classes.
- Separate divisions remain separate if a weight class was contested more than once.
- Mixed contests use `CategorySex=MX`, each athlete has `Sex=M` or `Sex=F`, and
  `competition.scoring` matches the source's ranking method.
- RIS contests with fewer than four lifts declare their actual event, have no
  weight classes anywhere in the competition, and use the source's `ReportedRis`.
  Follow [the shorter-event RIS rules](./DATA_REFERENCE.md#ris-for-fewer-than-four-lifts).
- You have explained any validation errors or warnings you need help with.

## Getting help

If you get stuck, email
[contact@openstreetlifting.org](mailto:contact@openstreetlifting.org) or open a
[GitHub issue](https://github.com/openstreetlifting/openstreetlifting/issues)
with your question and source.
