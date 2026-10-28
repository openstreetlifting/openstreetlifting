<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { resolve } from '$app/paths';
  import {
    EmailIcon,
    InstagramIcon,
    GitHubIcon,
    DiscordIcon,
  } from '#lib/components/icons/index.js';
  import { TEXT } from '#lib/constants/typography.js';
  import Seo from '#lib/components/seo.svelte';
  import { absolute, breadcrumbLd } from '#lib/seo/index.js';
  import { Breadcrumb } from '#lib/components/ui/index.js';
  import Message from '#lib/components/ui/message.svelte';
  import { rankingsHref } from '#lib/state/rankings-return.svelte.js';

  const i18n = useI18n();

  const rowClass =
    'flex items-center gap-3 py-4 text-sm text-secondary transition-colors hover:text-ink';
</script>

<Seo
  title={i18n.t('Contact')}
  description={i18n.t(
    'Get in touch with OpenStreetlifting to report a wrong result, send competition results, or contribute to the archive.'
  )}
  canonical={absolute('/contact')}
  jsonLd={[
    breadcrumbLd(
      [
        { name: i18n.t('Rankings'), path: '/' },
        { name: i18n.t('Contact'), path: '/contact' },
      ],
      i18n.locale
    ),
  ]}
/>

<div class="mx-auto max-w-reading px-4 py-10 sm:px-6 sm:py-16">
  <Breadcrumb
    items={[{ label: i18n.t('Rankings'), href: rankingsHref() }, { label: i18n.t('Contact') }]}
  />

  <!-- The h1 carries the page, so the plate is decoration and stays out of the
       accessibility tree rather than repeating the name to a screen reader. -->
  <img
    src="/logo_plate_full.svg"
    alt=""
    aria-hidden="true"
    class="mx-auto mb-8 h-44 w-44 sm:mb-10 sm:h-60 sm:w-60"
  />
  <h1 class="mb-4 {TEXT.title} text-ink">{i18n.t('Get in touch')}</h1>
  <div class="mb-10 space-y-4 text-base leading-relaxed text-secondary">
    <p>
      {i18n.t(
        'Ask a question, flag a mistake, or share competition results. I run OpenStreetlifting on my own, so replies may take a little time.'
      )}
    </p>
    <p>
      {#snippet email()}<a
          href="mailto:contact@openstreetlifting.org"
          class="underline underline-offset-4 hover:text-ink">contact@openstreetlifting.org</a
        >{/snippet}
      {#snippet privacyNotice()}<a
          href={i18n.href(resolve('privacy'))}
          class="underline underline-offset-4 hover:text-ink">{i18n.t('privacy notice')}</a
        >{/snippet}
      <Message
        message={'To ask about your personal data or request name removal, email {email}. Keep these requests private. You can read more in the {privacyNotice}.'}
        values={{ email, privacyNotice }}
      />
    </p>
  </div>

  <ul class="divide-y divide-stroke border-y border-stroke">
    <li>
      <a
        href="https://github.com/openstreetlifting/openstreetlifting"
        target="_blank"
        rel="noopener noreferrer"
        class={rowClass}
      >
        <GitHubIcon class="h-5 w-5 shrink-0" />
        <span class="text-ink">GitHub</span>
        <span class="ml-auto truncate">openstreetlifting</span>
      </a>
    </li>
    <li>
      <a
        href="https://www.instagram.com/openstreetlifting"
        target="_blank"
        rel="noopener noreferrer"
        class={rowClass}
      >
        <InstagramIcon class="h-5 w-5 shrink-0" />
        <span class="text-ink">Instagram</span>
        <span class="ml-auto truncate">@openstreetlifting</span>
      </a>
    </li>
    <li>
      <a
        href="https://discord.gg/MR9z7AnT"
        target="_blank"
        rel="noopener noreferrer"
        class={rowClass}
      >
        <DiscordIcon class="h-5 w-5 shrink-0" />
        <span class="text-ink">Discord</span>
        <span class="ml-auto truncate">{i18n.t('Join the server')}</span>
      </a>
    </li>
    <li>
      <a href="mailto:contact@openstreetlifting.org" class={rowClass}>
        <EmailIcon class="h-5 w-5 shrink-0" />
        <span class="text-ink">{i18n.t('Email')}</span>
        <span class="ml-auto truncate">contact@openstreetlifting.org</span>
      </a>
    </li>
  </ul>
</div>
