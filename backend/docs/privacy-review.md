# Privacy notice review — 16 September 2026

This is a research and implementation review for OPE-312, not a finding that the
service complies with the GDPR. The frontend notice is a draft for review.
This file is outside the published book. Statutory rights in the notice describe
obligations; they do not imply that the redaction command implements every right.

## Facts and open questions

The maintainer confirmed that the service runs in a Kubernetes cluster on an
OVH VPS in France. The repository separately publishes data through GitHub.
The draft identifies Adrien Pelfresne as controller, based on the project and
conversation context; confirm that no company or association holds that role.

Before publication, confirm the contact-mail provider, backups, subprocessors,
access from outside the EEA, and relevant contractual transfer safeguards.
Specify actual retention periods or concrete deletion criteria for analytics,
connection logs, backups, general correspondence, and rights-request evidence.
These facts cannot be established from this repository. The draft does not
invent periods or claim that all processing stays in France.

## Legal basis: a defensible candidate, with work remaining

For the results archive, Article 6(1)(f) is a candidate basis, not an automatic
permission. Record a legitimate-interest assessment for each purpose:

| Purpose | Assessment needed |
| --- | --- |
| Named results and cross-event profiles | Explain the sporting-history benefit, why identification is necessary, and why less intrusive publication would not suffice. |
| Exact and recovered bodyweights, rankings, comparisons | Assess necessity, estimation errors, health inferences, and long-term visibility. |
| Instagram links | Assess separately: finding an account is a different benefit from preserving a result. A clear match does not establish consent or necessity. |
| Public API, downloads, Git history, search indexing | Assess the wider audience, persistent copies, linkability, and reasonable expectations. |
| Children or vulnerable athletes | Give their interests additional weight and define how concerns will be detected and handled. |

Keep the purpose, necessity, alternatives, balancing, and safeguards in the
record. A request mechanism is a safeguard; it cannot cure unnecessary processing.
See [CNIL: legitimate interests](https://www.cnil.fr/fr/les-bases-legales/interet-legitime).

A contributor cannot consent on behalf of every athlete in a results sheet.
A public source is not a standalone legal basis. CNIL's reuse guidance applies
to information already online, including social platforms. See
[CNIL: reuse of published personal data](https://www.cnil.fr/fr/ouverture-et-reutilisation-de-donnees-personnelles-sur-internet-la-cnil-publie-ses-recommandations).

CNIL's amateur-sport guidance says that online publication requires information
and a simple objection route, and that a successful objection normally leads to
removal of published results. It concerns sports organisations; applying its
reasoning to this independent archive is an inference, not an express CNIL
approval of this project. It cautions against treating a numbered result as the
universal response. See
[CNIL: amateur sport, question 3](https://www.cnil.fr/fr/sport-amateur-hors-contrat/questions-reponses).

Handling rights requests has an Article 6(1)(c) basis. Retain only the evidence
and suppression data needed for those obligations; document the duration and
access rules. The archive is not shown to have a statutory public-interest
mission. Do not infer an Article 6(1)(e) basis or an Article 89 exemption from
its name. See [GDPR, Articles 5, 6 and 89](https://eur-lex.europa.eu/eli/reg/2016/679/oj/eng).

## Transparency and sensitive information

Indirect collection triggers Article 14. Identify how athletes will receive the
notice within the applicable deadline, including by the first disclosure where
required. Publishing a page alone needs a supported exception to individual
notice; document any Article 14(5)(b) assessment and safeguards. Lack of contact
details is relevant, not a blanket waiver. See
[CNIL reuse recommendations, information duties](https://cnil.fr/sites/cnil/files/2024-06/recommandations_reutilisateurs_donnees_publiees_sur_internet.pdf).

The old book asserted that none of the archive's data was special-category data.
That was too broad. Weight combined with other information can reveal health;
review bodyweight estimates and free-text status reasons in context. If Article
9 applies, an Article 6 basis alone is insufficient. A federation's publication
does not itself establish that the athlete manifestly made health data public.
See [CNIL: health data](https://www.cnil.fr/fr/quest-ce-ce-quune-donnee-de-sante)
and [GDPR, Article 9](https://eur-lex.europa.eu/eli/reg/2016/679/oj/eng).

## The tool's limits

`osl_importer/src/redact.rs` changes names, clears native names, removes social
handles, and writes a keyed suppression record. It retains the result and
matching metadata. An observer can compare results or Git revisions to identify
the person. The fingerprint is not encryption of the whole record and does not
make its public metadata anonymous. See
[CNIL: pseudonymisation and anonymisation](https://www.cnil.fr/fr/technologies/lanonymisation-de-donnees-personnelles).

The current implementation needs a separate manual process for erasure and
restriction, repository-history review, caches and backups, recipient notices,
and search-engine requests. It has no automatic erasure workflow for those
surfaces. A request to erase personal data must not silently become a request
to replace a name. The notice explains the distinction.

The deployment chart supplies `OSL_PRIVACY_KEY`; default import arguments are
`bulk-import --prune --yes`. Imports require a verified key when suppression
records exist. Fork CI can explicitly skip suppression checks only in
validation mode, which cannot write to the database. Verify the deployed
secret configuration. Name and identity variations remain a limitation.

## Rights covered by the notice

The draft includes information and access, correction, objection, erasure,
restriction, conditional portability, withdrawal of consent where applicable,
and safeguards for significant automated decisions. It also covers recipient
notification, high-risk breach information, and complaints. It avoids limiting
the notice to EU citizens. Judicial remedies, compensation, and representation
remain available under Articles 78–82; the notice does not recite those provisions.

Portability is conditional on automated processing based on consent or contract;
it is distinct from access. A computed ranking is not automatically an Article
22 decision: assess its actual effects, including foreseeable consequential use.
See [CNIL: portability](https://www.cnil.fr/fr/comprendre-mes-droits/le-droit-la-portabilite-obtenir-et-reutiliser-une-copie-de-vos-donnees)
and [automated-decision guidelines](https://www.cnil.fr/sites/default/files/atoms/files/wp251_profilage-fr.pdf).

The notice refers to Article 12 for request handling. This gives one month to respond, with an explained extension of
up to two months for complexity or volume, proportionate identity checks, and
normally free handling. Email is preferred for privacy, not a reason to ignore
an otherwise valid request. See
[GDPR rights and deadlines](https://www.cnil.fr/fr/reglement-europeen-protection-donnees/chapitre3),
[CNIL: identity checks](https://www.cnil.fr/fr/repondre-une-demande-de-droit-dacces),
[breach information](https://www.cnil.fr/fr/violations-de-donnees-personnelles-les-regles-suivre),
and [remedies](https://www.cnil.fr/fr/reglement-europeen-protection-donnees/chapitre8).

## Analytics and retention still need configuration evidence

`frontend/src/routes/+layout.svelte` loads Umami whenever both public environment
variables are set. No consent gate appears there. Self-hosting and avoiding
cookies do not alone establish an exemption under French tracker rules.
Verify the deployed version, collected fields, identifiers, reuse, retention,
and the conditions for exempt audience measurement. Establish the applicable
GDPR basis separately; if exemption conditions fail, change collection or obtain
valid consent before tracking. The notice cannot settle this through wording.
See [CNIL: audience measurement](https://www.cnil.fr/fr/cookies-solutions-pour-les-outils-de-mesure-daudience)
and [Umami documentation](https://docs.umami.is/docs/about).

The archive currently has no scheduled expiry. Document necessity and a review
schedule instead of treating history as permission to retain every identifiable
field forever. Fix concrete deletion rules for operational data before the
notice is published. See
[transparency guidelines, retention information](https://www.cnil.fr/sites/cnil/files/atoms/files/wp260_guidelines-transparence-fr.pdf).
