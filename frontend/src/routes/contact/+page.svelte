<script lang="ts">
  import { EmailIcon, InstagramIcon, GitHubIcon, DiscordIcon } from '$lib/components/icons';
  import { TEXT } from '$lib/constants/typography';
  import Seo from '$lib/components/seo.svelte';
  import { absolute } from '$lib/seo';

  const rowClass =
    'flex items-center gap-3 py-4 text-sm text-secondary transition-colors hover:text-ink';
</script>

<Seo
  title="Contact"
  description="Get in touch with OpenStreetlifting to report a wrong result, send competition results, or contribute to the archive."
  canonical={absolute('/contact')}
/>

<div class="mx-auto max-w-reading px-4 py-10 sm:px-6 sm:py-16">
  <!-- The h1 carries the page, so the plate is decoration and stays out of the
       accessibility tree rather than repeating the name to a screen reader. -->
  <img
    src="/logo_plate_full.svg"
    alt=""
    aria-hidden="true"
    class="mx-auto mb-8 h-44 w-44 sm:mb-10 sm:h-60 sm:w-60"
  />
  <h1 class="mb-4 {TEXT.title} text-ink">Get in touch</h1>
  <p class="mb-10 text-base text-muted">
    If you have a question, want to help me, or simply want to connect, reach out through any of
    these channels. I'm a solo developer on this project, I may take a little bit of time to
    respond.
  </p>

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
        <span class="ml-auto truncate">Join the server</span>
      </a>
    </li>
    <li>
      <a href="mailto:contact@openstreetlifting.org" class={rowClass}>
        <EmailIcon class="h-5 w-5 shrink-0" />
        <span class="text-ink">Email</span>
        <span class="ml-auto truncate">contact@openstreetlifting.org</span>
      </a>
    </li>
  </ul>
</div>
