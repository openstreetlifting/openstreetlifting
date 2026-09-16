# Personal Data

OpenStreetlifting brings competition results together on searchable athlete
pages. An athlete can appear here without having visited the site or submitted
data. The [privacy notice](https://openstreetlifting.org/privacy) identifies the
controller, explains the purposes of processing, and sets out your GDPR rights.

## What the archive holds

Results come from federations, organisers, competition platforms, public posts,
and contributors. They can include names in Latin and original scripts, sex
category, country, competition details, bodyweight, attempts, scores, placings,
and participation status. The archive also calculates rankings and performance
comparisons. Some bodyweights are recovered from published scores rather than
reported at a weigh-in; their origin is recorded in the data.

An Instagram handle may be added when a public account has been matched to an
athlete. You can ask for the link to be corrected or removed separately from
competition results.

The proposed basis for publishing results is legitimate interests under GDPR
Article 6(1)(f): preserving and comparing the sport's record. That basis requires
necessity and a balance with athletes' rights. Public sources, a CC0 licence, and
the word “archive” do not settle that balance. Linking results across years and
to social accounts increases the effect on a person's privacy.

Bodyweight and performance information require care. Whether information reveals
health depends on its context, combinations, and use; being a sports result does
not automatically exclude it from the rules for health data. See the
[CNIL's explanation of health data](https://www.cnil.fr/fr/quest-ce-ce-quune-donnee-de-sante).

## Request a correction or removal

Email [contact@openstreetlifting.org](mailto:contact@openstreetlifting.org) with
your athlete-page link or enough competition details to identify the record.
Say whether you want a correction, removal of a social link, removal of your
name, restriction, or erasure. You do not need to give a reason to use
the voluntary name-removal procedure.

Use email to keep the request private. A public issue, pull request, or Discord
message can expose the information you want removed. A request received through
another channel still needs to be handled; email is the preferred private route.

Requests are normally free and receive a response within one month. A complex
request may require up to two additional months, with an explanation within the
first month. Identity evidence is requested only when reasonably needed. The
[privacy notice](https://openstreetlifting.org/privacy#your-rights) explains
access, correction, objection, erasure, restriction, portability, consent,
automated decisions, and complaint rights, including their conditions.

## What the removal tool does

The importer replaces matching names in current result files with a numbered
label such as `Redacted Athlete #7`. It clears the original-script name and
removes matching Instagram entries. Importing the changed files and pruning the
old database records updates the website, API, and new downloads and removes
the old named profile.

The tool preserves competition details, sex category, country, bodyweight,
attempts, scores, and placings. Those details can still identify someone when
compared with source results or earlier copies. This is pseudonymisation, not
guaranteed anonymity or complete erasure. A successful command does not by itself
resolve an objection or erasure request.

A public removal record holds a keyed fingerprint of the name, matching fields,
and the replacement number. The secret key is kept outside the repository. This
record remains personal data and helps detect later imports of matching names;
changed spellings or identity fields may need additional checks. The check needs
the secret key to work.

For commands and file formats, see the
[athlete-data README](https://github.com/openstreetlifting/openstreetlifting/blob/main/backend/data/athletes/README.md).

## Earlier copies and continued retention

The archive currently has no scheduled expiry for results. Continued retention
of identifiable records still needs justification and remains subject to your
rights. The removal record serves a separate purpose: preventing republication
while future imports remain possible.

The tool does not rewrite Git history, remove source result sheets, recall
copies, or submit search-engine removal requests. Copies under the project's
control, including history, require their own assessment. Where erasure is
required, GDPR Article 17(2) also requires reasonable steps to inform other
controllers using the published data. Article 19 governs notification of
corrections, erasure, and restrictions to recipients, with its stated exceptions.
See [the GDPR](https://eur-lex.europa.eu/eli/reg/2016/679/oj/eng).

CC0 does not waive data protection rights. See
[Licensing](./LICENSING.md#cc0-and-personal-data). Requests to the original
publisher or a search engine can help with copies they control; they do not
replace this project's obligations.

You can complain to the [CNIL](https://www.cnil.fr/fr/plaintes) or another
competent EU data protection authority. The privacy notice also explains your
right to seek a judicial remedy.
