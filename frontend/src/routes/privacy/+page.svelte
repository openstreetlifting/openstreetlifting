<script lang="ts">
  import { TEXT } from '$lib/constants/typography';
  import Seo from '$lib/components/seo.svelte';
  import { absolute } from '$lib/seo';

  const linkClass = 'text-secondary underline underline-offset-4 transition-colors hover:text-ink';
</script>

<Seo
  title="Privacy policy"
  description="How OpenStreetlifting uses athlete and visitor data, your GDPR rights, and how to ask for a correction or removal."
  canonical={absolute('/privacy')}
/>

<div class="mx-auto max-w-reading px-4 py-10 sm:px-6 sm:py-16">
  <header class="mb-10">
    <h1 class="mb-4 {TEXT.title} text-ink">Privacy policy</h1>
    <p class="mb-6 text-sm text-muted">
      Last updated: <time datetime="2026-09-16">16 September 2026</time>
    </p>
    <p class="mb-4 leading-relaxed text-secondary">
      OpenStreetlifting collects public competition results into a searchable archive. If you
      compete, your results may appear here even if you have never used the site.
    </p>
    <p class="leading-relaxed text-secondary">
      To correct your data, remove your Instagram link, or ask to be taken off the site, email
      <a href="mailto:contact@openstreetlifting.org" class={linkClass}
        >contact@openstreetlifting.org</a
      >. Please keep privacy requests out of public GitHub issues and Discord messages.
    </p>
  </header>

  <nav aria-label="On this page" class="mb-12 border-y border-stroke py-5">
    <ul class="flex flex-wrap gap-x-6 gap-y-3 text-sm">
      <li><a href="#data" class={linkClass}>Personal data</a></li>
      <li><a href="#legal-basis" class={linkClass}>Legal basis</a></li>
      <li><a href="#your-rights" class={linkClass}>GDPR rights</a></li>
      <li><a href="#removal" class={linkClass}>Removal requests</a></li>
      <li><a href="#retention" class={linkClass}>Data retention</a></li>
    </ul>
  </nav>

  <div class="space-y-12 leading-relaxed text-secondary">
    <section aria-labelledby="controller">
      <h2 id="controller" class="mb-4 {TEXT.heading} text-ink">Data controller</h2>
      <p>
        Adrien Pelfresne is the data controller for OpenStreetlifting. Contact:
        <a href="mailto:contact@openstreetlifting.org" class={linkClass}
          >contact@openstreetlifting.org</a
        >.
      </p>
    </section>

    <section aria-labelledby="data">
      <h2 id="data" class="mb-4 {TEXT.heading} text-ink">Personal data</h2>
      <p class="mb-4">
        Athlete records come from results published by federations, organisers, competition
        platforms, and public posts, or supplied by contributors. Competition pages link to sources
        where available. Records can include:
      </p>
      <ul class="mb-4 list-disc space-y-2 pl-5">
        <li>Your name, including its original script, sex category, and country.</li>
        <li>Competitions entered, dates, locations, divisions, and weight classes.</li>
        <li>
          Bodyweight, attempts, best lifts, totals, placings, scores, and participation status.
        </li>
        <li>A public Instagram handle, when an account has been matched to you.</li>
      </ul>
      <p class="mb-4">
        The site also calculates rankings and performance comparisons. Some missing bodyweights are
        estimated from published scores and marked as recovered in the data.
      </p>
      <p>
        Emails contain your address and the information you send. GitHub contributions display your
        public account and contribution history.
      </p>
    </section>

    <section aria-labelledby="legal-basis">
      <h2 id="legal-basis" class="mb-4 {TEXT.heading} text-ink">Legal basis</h2>
      <p class="mb-4">
        I rely on legitimate interests under Article 6(1)(f) of the GDPR to preserve competition
        results, connect an athlete’s performances across events, and make the sport’s record
        accessible. Instagram links help readers find an athlete’s public account.
      </p>
      <p class="mb-4">
        The same basis applies to answering enquiries and managing contributions. Handling privacy
        requests and maintaining the removal record relies on legal obligations under Article
        6(1)(c).
      </p>
    </section>

    <section aria-labelledby="sharing">
      <h2 id="sharing" class="mb-4 {TEXT.heading} text-ink">Data sharing</h2>
      <p class="mb-4">
        Athlete data is public through the website, API, downloads, and GitHub repository. Search
        engines can index it, and people worldwide can copy it. The CC0 licence does not affect your
        data protection rights.
      </p>
      <p class="mb-4">
        OVH hosts the service, database, and analytics on a VPS in France. GitHub hosts the public
        repository. Public copies may be held outside the European Economic Area.
      </p>
      <p>Privacy-request emails are handled privately.</p>
    </section>

    <section aria-labelledby="visitors">
      <h2 id="visitors" class="mb-4 {TEXT.heading} text-ink">Website analytics</h2>
      <p class="mb-4">
        You can browse without an account. Servers receive connection information, including your IP
        address and browser details, to deliver pages and protect the service, on the basis of
        legitimate interests under Article 6(1)(f).
      </p>
      <p>
        When enabled, self-hosted Umami measures visits to help me understand how the site is used.
        It can record pages, referral sources, browser and device details, and approximate location.
        Its standard tracker uses no cookies.
      </p>
    </section>

    <section aria-labelledby="your-rights">
      <h2 id="your-rights" class="mb-4 {TEXT.heading} text-ink">GDPR rights</h2>
      <p class="mb-5">
        Subject to the conditions of the
        <a href="https://eur-lex.europa.eu/eli/reg/2016/679/oj/eng" class={linkClass}>GDPR</a>, you
        have the following rights:
      </p>
      <ul class="list-disc space-y-3 pl-5">
        <li>Information about processing and access to your personal data (Articles 12–15).</li>
        <li>Rectification of inaccurate or incomplete data (Article 16).</li>
        <li>Erasure of your personal data (Article 17).</li>
        <li>Restriction of processing (Article 18).</li>
        <li>
          Information about recipients notified of rectification, erasure, or restriction (Article
          19).
        </li>
        <li>
          Portability of data you provided, where processing is automated and based on consent or
          contract (Article 20).
        </li>
        <li>
          Objection to processing based on legitimate interests, on grounds relating to your
          particular situation (Article 21).
        </li>
        <li>Withdrawal of consent where processing relies on it (Article 7(3)).</li>
        <li>
          Protection against solely automated decisions with legal or similarly significant effects
          (Article 22).
        </li>
        <li>
          Notification of a personal data breach posing a high risk to your rights and freedoms
          (Article 34).
        </li>
      </ul>
      <p class="mt-5">
        Athlete-result publication relies on legitimate interests. You can object to that
        processing.
      </p>
    </section>

    <section aria-labelledby="removal">
      <h2 id="removal" class="mb-4 {TEXT.heading} text-ink">Removal requests</h2>
      <p class="mb-4">
        Email <a href="mailto:contact@openstreetlifting.org" class={linkClass}
          >contact@openstreetlifting.org</a
        >
        with your athlete-page link or competition details and the change you are requesting. Requests
        are handled in accordance with Article 12 of the GDPR.
      </p>
      <h3 class="mb-3 {TEXT.subheading} text-ink">Name removal</h3>
      <p class="mb-4">
        The procedure replaces your name with a numbered label, such as “Redacted Athlete #7”, and
        removes your original-script name and Instagram link. Once published, the change appears on
        the website, API, and fresh downloads, and the old named profile is removed.
      </p>
      <p class="mb-4">
        Competition details, sex category, country, bodyweight, and results remain. These may still
        identify you when compared with other sources. Name removal is pseudonymisation; a request
        for erasure may require further action.
      </p>
      <p class="mb-4">
        A public removal record contains a keyed fingerprint of your name, matching details, and the
        replacement number to help prevent republication. The key is kept separately. The record
        remains personal data.
      </p>
      <p>
        The procedure does not automatically remove earlier Git revisions, downloaded copies, source
        results, or search-engine caches. Requests affecting those copies are assessed under the
        GDPR, including the notification duties in Articles 17(2) and 19.
      </p>
    </section>

    <section aria-labelledby="retention">
      <h2 id="retention" class="mb-4 {TEXT.heading} text-ink">Data retention</h2>
      <p class="mb-4">
        Competition results have no scheduled expiry: they form the sporting archive. Continued
        retention remains subject to your GDPR rights.
      </p>
      <p>
        Removal records are kept while needed to prevent your name from being republished through
        future imports.
      </p>
    </section>

    <section aria-labelledby="complaints">
      <h2 id="complaints" class="mb-4 {TEXT.heading} text-ink">Complaints</h2>
      <p>
        You may lodge a complaint with the
        <a href="https://www.cnil.fr/fr/plaintes" class={linkClass}>CNIL</a>
        or another competent supervisory authority (Article 77).
      </p>
    </section>
  </div>
</div>
