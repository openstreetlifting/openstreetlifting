# Personal Data

This archive is a record of people. It names them, says what they weighed and
what they lifted, and puts that on a page a search engine can reach. None of
them were asked first, because a results archive that only holds the people who
opted in is not a record of the sport.

This chapter says what is held, why, and what you can do about it. The
[privacy notice](https://openstreetlifting.org/privacy) is the formal version
and names the controller; this is the part that explains itself.

## What the archive holds about you

| What | Where it comes from |
| --- | --- |
| Your name | The result sheet the federation published |
| Your name in its own alphabet | The same, when the source gives it |
| Sex and country | The same |
| Bodyweight | The weigh-in |
| Every attempt, the placing, the RIS | The same |
| An Instagram handle | Added by hand, only when the account is obviously yours |

There is no birth date, no age, no email address, no club, no photograph. The
files are `backend/data/competitions/{federation}/{year}/{slug}/entries.csv`,
and the handle, if there is one, is one line in
`backend/data/athletes/instagram.csv`.

## Why it is published

The results are already public. Federations publish them, competition software
publishes them, and organisers post them. What the archive adds is that they
stay published, in one place, in a shape you can compare across meets and years.
Streetlifting has no governing body keeping that record, so without it the
results scatter across Instagram posts and dead PDFs.

That is the legitimate interest the processing rests on, and it is worth being
honest about the other half of the balance. A single federation page names you
once. This archive puts every meet you have entered on one page, cross-links it
to your Instagram, and hands it to Google. That is more than any of the sources
did on their own, and it is the reason the chapter you are reading exists rather
than a line in a footer. Where the two do not balance for you, they do not, and
the answer is [redaction](#asking-to-be-taken-off-the-site) rather than an
argument.

## What it is not

None of this is special category data. Sex is not. Bodyweight is a weigh-in
figure that decides which class you lift in, not a measure of your health, and
it is recorded because the score cannot be computed without it.

## How long it stays

Indefinitely. A record that drops the results from ten years ago is not a
record, and the value of the archive is that a result stays findable after the
federation's website has gone. Retention is the whole point rather than an
oversight, which is why the way out is redaction rather than an expiry date.

## Your options

**Fix something wrong.** A misspelled name, a wrong lift, a missing competition.
Open an issue or a pull request against the file, or email. See
[Contributing Data](./CONTRIBUTING_DATA.md).

**Take the handle off.** The Instagram link is the one piece of this that is not
a competition result, and it is the part that turns a result into a way to find
you. Ask and the line goes, and nothing else changes.

**Be taken off the site.** Your name is removed from the archive and your
results stay, under a stand-in.

## Asking to be taken off the site

Email [contact@openstreetlifting.org](mailto:contact@openstreetlifting.org).

You do not have to give a reason, and you will not be argued with.

**Do not open an issue or a pull request**, and do not ask in the Discord. All
three are public, and a public request to be forgotten is a second publication
of the thing you are asking to remove. Email is the only route for this.

## What redaction does

Your name is replaced everywhere it appears with a stand-in, `Redacted Athlete
#7`. The name in its own alphabet goes. The Instagram handle goes. Your athlete
page, the rankings, the API and the CSV downloads carry the stand-in from the
next deploy onward, and your old page stops existing.

The result itself does not move. Sex, country, bodyweight, every attempt and the
placing stay exactly as they were, so the meet still reconciles and the rankings
do not shift. This is the trade the archive makes: the sport keeps its record,
and the record stops being about a named person.

Your name is also recorded against the redaction so that a competition you enter
next year does not quietly put it back. It is stored as a hash under a key that
is not in the repository, so the list of people who asked cannot be read off it.

## What redaction cannot undo

Being straight about the limits matters more than sounding thorough.

The archive is a public git repository. Commits made before the redaction still
contain your name, and history is not rewritten as a matter of course, because
doing so breaks every clone and fork and does not reach the copies that already
exist. If your situation makes the history itself the problem, say so in your
email and it will be looked at.

The data is released under CC0, so anyone may have taken a copy. A redaction
cannot reach a copy that has already been downloaded, and whoever holds it
becomes responsible for it on their own account. See
[Licensing](./LICENSING.md#cc0-and-personal-data).

Search engines are asked to drop the old page, and usually do, but they are not
under this project's control. If your own name in a search result is what
brought you here, the search engine has its own removal process and it is worth
using alongside this one.

## If you are not satisfied

You can complain to the CNIL, the French data protection authority, at
[cnil.fr](https://www.cnil.fr/). Doing so does not affect anything asked for
here.
