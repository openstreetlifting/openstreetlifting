<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { formatLongDate } from '#lib/utils/format.js';
  import { TEXT } from '#lib/constants/typography.js';
  import Seo from '#lib/components/seo.svelte';
  import { absolute, breadcrumbLd } from '#lib/seo/index.js';
  import { Breadcrumb } from '#lib/components/ui/index.js';
  import { rankingsHref } from '#lib/state/rankings-return.svelte.js';

  const i18n = useI18n();

  const linkClass = 'text-secondary underline underline-offset-4 transition-colors hover:text-ink';
</script>

<Seo
  title={i18n.t('Privacy policy')}
  description={i18n.t(
    'How OpenStreetlifting uses athlete and visitor data, your GDPR rights, and how to ask for a correction or removal.'
  )}
  canonical={absolute('/privacy')}
  jsonLd={[
    breadcrumbLd([
      { name: i18n.t('Rankings'), path: '/' },
      { name: i18n.t('Privacy'), path: '/privacy' },
    ]),
  ]}
/>

<div class="mx-auto max-w-reading px-4 py-10 sm:px-6 sm:py-16">
  <Breadcrumb
    items={[{ label: i18n.t('Rankings'), href: rankingsHref() }, { label: i18n.t('Privacy') }]}
  />

  <header class="mb-10">
    <h1 class="mb-4 {TEXT.title} text-ink">{i18n.t('Privacy policy')}</h1>
    <p class="mb-6 text-sm text-muted">
      {i18n.t('Last updated:')}
      <time datetime="2026-09-16">{formatLongDate('2026-09-16', i18n.locale)}</time>
    </p>
    <p class="mb-4 leading-relaxed text-secondary">
      {i18n.t(
        'OpenStreetlifting collects public competition results into a searchable archive. If you compete, your results may appear here even if you have never used the site.'
      )}
    </p>
    <p class="leading-relaxed text-secondary">
      {i18n.t(
        'To correct your data, remove your Instagram link, or ask to be taken off the site, email'
      )}
      <a href="mailto:contact@openstreetlifting.org" class={linkClass}
        >contact@openstreetlifting.org</a
      >{i18n.t('. Please keep privacy requests out of public GitHub issues and Discord messages.')}
    </p>
  </header>

  <p class="mb-6 leading-relaxed text-secondary">
    {i18n.t(
      'A language preference cookie remembers your choice for one year. It is only set when you choose a language.'
    )}
  </p>

  <nav aria-label={i18n.t('On this page')} class="mb-12 border-y border-stroke py-5">
    <ul class="flex flex-wrap gap-x-6 gap-y-3 text-sm">
      <li><a href="#data" class={linkClass}>{i18n.t('Personal data')}</a></li>
      <li><a href="#legal-basis" class={linkClass}>{i18n.t('Legal basis')}</a></li>
      <li><a href="#your-rights" class={linkClass}>{i18n.t('GDPR rights')}</a></li>
      <li><a href="#removal" class={linkClass}>{i18n.t('Removal requests')}</a></li>
      <li><a href="#retention" class={linkClass}>{i18n.t('Data retention')}</a></li>
    </ul>
  </nav>

  <div class="space-y-12 leading-relaxed text-secondary">
    <section aria-labelledby="controller">
      <h2 id="controller" class="mb-4 {TEXT.heading} text-ink">{i18n.t('Data controller')}</h2>
      <p>
        {i18n.t('Adrien Pelfresne is the data controller for OpenStreetlifting. Contact:')}
        <a href="mailto:contact@openstreetlifting.org" class={linkClass}
          >contact@openstreetlifting.org</a
        >.
      </p>
    </section>

    <section aria-labelledby="data">
      <h2 id="data" class="mb-4 {TEXT.heading} text-ink">{i18n.t('Personal data')}</h2>
      <p class="mb-4">
        {i18n.t(
          'Athlete records come from results published by federations, organisers, competition platforms, and public posts, or supplied by contributors. Competition pages link to sources where available. Records can include:'
        )}
      </p>
      <ul class="mb-4 list-disc space-y-2 pl-5">
        <li>{i18n.t('Your name, including its original script, sex category, and country.')}</li>
        <li>{i18n.t('Competitions entered, dates, locations, divisions, and weight classes.')}</li>
        <li>
          {i18n.t(
            'Bodyweight, attempts, best lifts, totals, placings, scores, and participation status.'
          )}
        </li>
        <li>{i18n.t('A public Instagram handle, when an account has been matched to you.')}</li>
      </ul>
      <p class="mb-4">
        {i18n.t(
          'The site also calculates rankings and performance comparisons. Some missing bodyweights are estimated from published scores and marked as recovered in the data.'
        )}
      </p>
      <p>
        {i18n.t(
          'Emails contain your address and the information you send. GitHub contributions display your public account and contribution history.'
        )}
      </p>
    </section>

    <section aria-labelledby="legal-basis">
      <h2 id="legal-basis" class="mb-4 {TEXT.heading} text-ink">{i18n.t('Legal basis')}</h2>
      <p class="mb-4">
        {i18n.t(
          'I rely on legitimate interests under Article 6(1)(f) of the GDPR to preserve competition results, connect an athlete’s performances across events, and make the sport’s record accessible. Instagram links help readers find an athlete’s public account.'
        )}
      </p>
      <p class="mb-4">
        {i18n.t(
          'The same basis applies to answering enquiries and managing contributions. Handling privacy requests and maintaining the removal record relies on legal obligations under Article 6(1)(c).'
        )}
      </p>
    </section>

    <section aria-labelledby="sharing">
      <h2 id="sharing" class="mb-4 {TEXT.heading} text-ink">{i18n.t('Data sharing')}</h2>
      <p class="mb-4">
        {i18n.t(
          'Athlete data is public through the website, API, downloads, and GitHub repository. Search engines can index it, and people worldwide can copy it. The CC0 licence does not affect your data protection rights.'
        )}
      </p>
      <p class="mb-4">
        {i18n.t(
          'OVH hosts the service, database, and analytics on a VPS in France. GitHub hosts the public repository. Public copies may be held outside the European Economic Area.'
        )}
      </p>
      <p>{i18n.t('Privacy-request emails are handled privately.')}</p>
    </section>

    <section aria-labelledby="visitors">
      <h2 id="visitors" class="mb-4 {TEXT.heading} text-ink">{i18n.t('Website analytics')}</h2>
      <p class="mb-4">
        {i18n.t(
          'You can browse without an account. Servers receive connection information, including your IP address and browser details, to deliver pages and protect the service, on the basis of legitimate interests under Article 6(1)(f).'
        )}
      </p>
      <p>
        {i18n.t(
          'When enabled, self-hosted Umami measures visits to help me understand how the site is used. It can record pages, referral sources, browser and device details, and approximate location. Its standard tracker uses no cookies.'
        )}
      </p>
    </section>

    <section aria-labelledby="your-rights">
      <h2 id="your-rights" class="mb-4 {TEXT.heading} text-ink">{i18n.t('GDPR rights')}</h2>
      <p class="mb-5">
        {i18n.t('Subject to the conditions of the')}
        <a
          href={i18n.locale === 'fr'
            ? 'https://eur-lex.europa.eu/eli/reg/2016/679/oj/fra'
            : 'https://eur-lex.europa.eu/eli/reg/2016/679/oj/eng'}
          class={linkClass}>{i18n.t('GDPR')}</a
        >{i18n.t(', you have the following rights:')}
      </p>
      <ul class="list-disc space-y-3 pl-5">
        <li>
          {i18n.t(
            'Information about processing and access to your personal data (Articles 12–15).'
          )}
        </li>
        <li>{i18n.t('Rectification of inaccurate or incomplete data (Article 16).')}</li>
        <li>{i18n.t('Erasure of your personal data (Article 17).')}</li>
        <li>{i18n.t('Restriction of processing (Article 18).')}</li>
        <li>
          {i18n.t(
            'Information about recipients notified of rectification, erasure, or restriction (Article 19).'
          )}
        </li>
        <li>
          {i18n.t(
            'Portability of data you provided, where processing is automated and based on consent or contract (Article 20).'
          )}
        </li>
        <li>
          {i18n.t(
            'Objection to processing based on legitimate interests, on grounds relating to your particular situation (Article 21).'
          )}
        </li>
        <li>{i18n.t('Withdrawal of consent where processing relies on it (Article 7(3)).')}</li>
        <li>
          {i18n.t(
            'Protection against solely automated decisions with legal or similarly significant effects (Article 22).'
          )}
        </li>
        <li>
          {i18n.t(
            'Notification of a personal data breach posing a high risk to your rights and freedoms (Article 34).'
          )}
        </li>
      </ul>
      <p class="mt-5">
        {i18n.t(
          'Athlete-result publication relies on legitimate interests. You can object to that processing.'
        )}
      </p>
    </section>

    <section aria-labelledby="removal">
      <h2 id="removal" class="mb-4 {TEXT.heading} text-ink">{i18n.t('Removal requests')}</h2>
      <p class="mb-4">
        {i18n.t('Email')}
        <a href="mailto:contact@openstreetlifting.org" class={linkClass}
          >contact@openstreetlifting.org</a
        >
        {i18n.t(
          'with your athlete-page link or competition details and the change you are requesting. Requests are handled in accordance with Article 12 of the GDPR.'
        )}
      </p>
      <h3 class="mb-3 {TEXT.subheading} text-ink">{i18n.t('Name removal')}</h3>
      <p class="mb-4">
        {i18n.t(
          'The procedure replaces your name with a numbered label, such as “Redacted Athlete #7”, and removes your original-script name and Instagram link. Once published, the change appears on the website, API, and fresh downloads, and the old named profile is removed.'
        )}
      </p>
      <p class="mb-4">
        {i18n.t(
          'Competition details, sex category, country, bodyweight, and results remain. These may still identify you when compared with other sources. Name removal is pseudonymisation; a request for erasure may require further action.'
        )}
      </p>
      <p class="mb-4">
        {i18n.t(
          'A public removal record contains a keyed fingerprint of your name, matching details, and the replacement number to help prevent republication. The key is kept separately. The record remains personal data.'
        )}
      </p>
      <p>
        {i18n.t(
          'The procedure does not automatically remove earlier Git revisions, downloaded copies, source results, or search-engine caches. Requests affecting those copies are assessed under the GDPR, including the notification duties in Articles 17(2) and 19.'
        )}
      </p>
    </section>

    <section aria-labelledby="retention">
      <h2 id="retention" class="mb-4 {TEXT.heading} text-ink">{i18n.t('Data retention')}</h2>
      <p class="mb-4">
        {i18n.t(
          'Competition results have no scheduled expiry: they form the sporting archive. Continued retention remains subject to your GDPR rights.'
        )}
      </p>
      <p>
        {i18n.t(
          'Removal records are kept while needed to prevent your name from being republished through future imports.'
        )}
      </p>
    </section>

    <section aria-labelledby="complaints">
      <h2 id="complaints" class="mb-4 {TEXT.heading} text-ink">{i18n.t('Complaints')}</h2>
      <p>
        {i18n.t('You may lodge a complaint with the')}
        <a href="https://www.cnil.fr/fr/plaintes" class={linkClass}>{i18n.t('CNIL')}</a>
        {i18n.t('or another competent supervisory authority (Article 77).')}
      </p>
    </section>
  </div>
</div>
