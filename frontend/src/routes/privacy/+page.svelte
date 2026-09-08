<script lang="ts">
  import { TEXT } from '$lib/constants/typography';
  import Seo from '$lib/components/seo.svelte';
  import { absolute } from '$lib/seo';
</script>

<Seo
  title="Privacy Policy"
  description="OpenStreetlifting privacy policy and data protection information."
  canonical={absolute('/privacy')}
/>

<div class="mx-auto max-w-reading px-4 py-10 sm:px-6 sm:py-16">
  <div class="mb-16">
    <h1 class="mb-4 {TEXT.title} text-ink">Privacy Policy</h1>
    <p class="text-base text-muted">
      Last updated: {new Date().toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'long',
        day: 'numeric',
      })}
    </p>
  </div>

  <div class="prose max-w-none prose-invert">
    <section class="mb-12">
      <h2 class="mb-4 {TEXT.heading} text-ink">Introduction</h2>
      <p class="mb-4 leading-relaxed text-secondary">
        OpenStreetlifting is an open source project committed to protecting your privacy. This
        policy explains how we collect, use, and protect your personal data in compliance with the
        General Data Protection Regulation (GDPR) and other applicable European data protection
        laws.
      </p>
    </section>

    <section class="mb-12">
      <h2 class="mb-4 {TEXT.heading} text-ink">Data Controller</h2>
      <p class="mb-4 leading-relaxed text-secondary">
        The data controller responsible for your personal data is the OpenStreetlifting project. For
        any privacy-related inquiries, you can contact us at:
      </p>
      <p class="leading-relaxed text-secondary">
        Email: <a
          href="mailto:contact@openstreetlifting.org"
          class="text-secondary transition-colors hover:text-ink">contact@openstreetlifting.org</a
        >
      </p>
    </section>

    <section class="mb-12">
      <h2 class="mb-4 {TEXT.heading} text-ink">Data We Collect</h2>
      <h3 class="mb-3 {TEXT.subheading} text-ink">Analytics Data</h3>
      <p class="mb-4 leading-relaxed text-secondary">
        We use Umami Analytics, a privacy-focused, open source analytics platform that respects your
        privacy. Umami collects only anonymous, aggregated data about website usage, including:
      </p>
      <ul class="mb-4 space-y-2 text-secondary">
        <li class="">Pages visited</li>
        <li class="">Referral sources</li>
        <li class="">Browser and device type</li>
        <li class="">Country (derived from IP address, which is not stored)</li>
      </ul>
      <p class="mb-4 leading-relaxed text-secondary">
        Umami does not use cookies, does not track you across websites, does not collect personal
        information, and does not store IP addresses. All data is anonymized and cannot be used to
        identify individual users.
      </p>

      <h3 class="mb-3 {TEXT.subheading} text-ink">User-Submitted Data</h3>
      <p class="mb-4 leading-relaxed text-secondary">
        If you submit competition results or athlete data to our platform, we collect and store the
        information you provide. The transformed data is licensed under <a
          href="https://creativecommons.org/licenses/by/4.0/"
          target="_blank"
          rel="noopener noreferrer"
          class="text-secondary transition-colors hover:text-ink">CC BY 4.0</a
        >
      </p>
    </section>

    <section class="mb-12">
      <h2 class="mb-4 {TEXT.heading} text-ink">Legal Basis for Processing</h2>
      <p class="mb-4 leading-relaxed text-secondary">
        We process your data based on the following legal grounds:
      </p>
      <ul class="mb-4 space-y-2 text-secondary">
        <li class="">
          <strong class="font-medium text-secondary">Legitimate interests:</strong> Anonymous analytics
          to improve our service
        </li>
        <li class="">
          <strong class="font-medium text-secondary">Consent:</strong> When you voluntarily submit data
          to our platform
        </li>
      </ul>
    </section>

    <section class="mb-12">
      <h2 class="mb-4 {TEXT.heading} text-ink">Data Sharing</h2>
      <p class="mb-4 leading-relaxed text-secondary">
        All competition and athlete data submitted to OpenStreetlifting is publicly available as
        part of our open database. We do not sell, rent, or share personal data with third parties
        for marketing purposes.
      </p>
    </section>

    <section class="mb-12">
      <h2 class="mb-4 {TEXT.heading} text-ink">Your Rights Under GDPR</h2>
      <p class="mb-4 leading-relaxed text-secondary">
        As a user in the European Union, you have the following rights:
      </p>
      <ul class="mb-4 space-y-2 text-secondary">
        <li class="">
          <strong class="font-medium text-secondary">Right to access:</strong> Request a copy of your
          personal data
        </li>
        <li class="">
          <strong class="font-medium text-secondary">Right to rectification:</strong> Request correction
          of inaccurate data
        </li>
        <li class="">
          <strong class="font-medium text-secondary">Right to erasure:</strong> Request deletion of your
          personal data
        </li>
        <li class="">
          <strong class="font-medium text-secondary">Right to restriction:</strong> Request limitation
          of data processing
        </li>
        <li class="">
          <strong class="font-medium text-secondary">Right to data portability:</strong> Receive your
          data in a structured format
        </li>
        <li class="">
          <strong class="font-medium text-secondary">Right to object:</strong> Object to processing of
          your personal data
        </li>
        <li class="">
          <strong class="font-medium text-secondary">Right to withdraw consent:</strong> Withdraw consent
          at any time
        </li>
      </ul>
      <p class="mb-4 leading-relaxed text-secondary">
        To exercise any of these rights, please contact us at <a
          href="mailto:contact@openstreetlifting.org"
          class="text-secondary transition-colors hover:text-ink">contact@openstreetlifting.org</a
        >.
      </p>
    </section>

    <section class="mb-12">
      <h2 class="mb-4 {TEXT.heading} text-ink">Data Retention</h2>
      <p class="mb-4 leading-relaxed text-secondary">
        Anonymous analytics data is retained indefinitely for statistical purposes. User-submitted
        competition and athlete data becomes part of our permanent public archive, as this is the
        core purpose of the OpenStreetlifting project.
      </p>
    </section>

    <section class="mb-12">
      <h2 class="mb-4 {TEXT.heading} text-ink">Cookies</h2>
      <p class="mb-4 leading-relaxed text-secondary">
        This website does not use cookies. Umami Analytics operates without cookies and does not
        require cookie consent banners.
      </p>
    </section>

    <section class="mb-12">
      <h2 class="mb-4 {TEXT.heading} text-ink">Changes to This Policy</h2>
      <p class="mb-4 leading-relaxed text-secondary">
        We may update this privacy policy from time to time. Any changes will be posted on this page
        with an updated revision date.
      </p>
    </section>

    <section class="mb-12">
      <h2 class="mb-4 {TEXT.heading} text-ink">Contact Us</h2>
      <p class="mb-4 leading-relaxed text-secondary">
        If you have any questions about this privacy policy or our data practices, please contact
        us:
      </p>
      <ul class="space-y-2 text-secondary">
        <li class="">
          Email: <a
            href="mailto:contact@openstreetlifting.org"
            class="text-secondary transition-colors hover:text-ink">contact@openstreetlifting.org</a
          >
        </li>
        <li class="">
          GitHub: <a
            href="https://github.com/openstreetlifting"
            target="_blank"
            rel="noopener noreferrer"
            class="text-secondary transition-colors hover:text-ink">github.com/openstreetlifting</a
          >
        </li>
      </ul>
    </section>
  </div>
</div>
