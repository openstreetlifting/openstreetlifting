# Contributing Data

If you know Git, you can add or
correct the archive's files and open a pull request on
[GitHub](https://github.com/openstreetlifting/openstreetlifting). Otherwise,
send me the results through the [contact page](https://openstreetlifting.org/contact).

## Before you start

### What counts as a source

A federation PDF, a scoresheet photo, a competition livestream or a public
Instagram post can all be useful. The source should identify the competition and
let us check the results. Prefer official results from the organizer when
available.

### Check the competition is not already there

Search the [competition archive](https://openstreetlifting.org/competitions) by
name, federation or year. Check the date and location too, as the name may differ.
If you find the competition, you can help by correcting mistakes or adding
missing results.

## Adding a competition

Each competition has a folder under
`backend/data/competitions/{federation}/{year}/{slug}/`. A _slug_ is a short name
used to identify the competition, such as `worlds-2023`.

The folder contains two files:

- `competition.toml`: the competition's name, dates, location and source links.
- `entries.csv`: the athletes and their results.

For example, the [FinalRep Worlds 2023 folder on GitHub](https://github.com/openstreetlifting/openstreetlifting/tree/main/backend/data/competitions/finalrep/2023/worlds-2023)
contains both files. Open them to see how a competition is stored.

Use an existing competition as an example, replacing its details and results
with your own. Check the project files for an existing entry before creating a
new folder. The [Data Reference](./DATA_REFERENCE.md) explains how to fill in each
file.

### Validate it locally

If you have downloaded the repository and have Rust and Cargo installed, open a
terminal in the `backend` folder and run:

```sh
SQLX_OFFLINE=true cargo run -p osl_importer --bin import -- prepare data/competitions/<federation>/<year>/<slug>
SQLX_OFFLINE=true cargo run -p osl_importer --bin import -- competitions --dry-run
```

Replace the preparation path with your competition. Preparation fills totals from
complete lift results and formats its files; the dry run checks the archive without
writing to the database. The summary should show zero
failures. Review any warnings and compare your entries with the source as well.
If you cannot run this check, mention it in your pull request so we can help.

### Open a pull request

Open a pull request in the
[project repository](https://github.com/openstreetlifting/openstreetlifting).

Give your pull request a clear title, such as `Add FinalRep Worlds 2023 results`.
Include the source link, a short description of your changes and anything you
are unsure about. Mention whether you ran the validation check.

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
- You have explained any validation errors or warnings you need help with.

## Getting help

If you get stuck, email
[contact@openstreetlifting.org](mailto:contact@openstreetlifting.org) or open a
[GitHub issue](https://github.com/openstreetlifting/openstreetlifting/issues)
with your question and source. You do not need to prepare the files before asking
for help. Sharing results is already a contribution.
