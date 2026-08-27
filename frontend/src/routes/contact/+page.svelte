<script lang="ts">
  import { EmailIcon, InstagramIcon, GitHubIcon, DiscordIcon } from '$lib/components/icons';
  import { TEXT } from '$lib/constants/typography';

  const rowClass =
    'flex items-center gap-3 py-4 text-sm text-zinc-400 transition-colors hover:text-white';
</script>

<svelte:head>
  <title>Contact - OpenStreetlifting</title>
  <meta name="description" content="Get in touch with OpenStreetlifting" />
</svelte:head>

<div class="mx-auto max-w-4xl px-4 py-10 sm:px-6 sm:py-16">
  <h1 class="mb-4 {TEXT.title} text-white">Get in touch</h1>
  <p class="mb-10 text-base font-light text-zinc-500">
    If you have a question, want to help me, or simply want to connect, reach out through any of
    these channels. I'm a solo developer on this project, I may take a little bit of time to
    respond.
  </p>

  <ul class="divide-y divide-zinc-800/50 border-y border-zinc-800/50">
    <li>
      <a
        href="https://github.com/openstreetlifting/openstreetlifting"
        target="_blank"
        rel="noopener noreferrer"
        class={rowClass}
      >
        <GitHubIcon class="h-5 w-5 shrink-0" />
        <span class="text-white">GitHub</span>
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
        <span class="text-white">Instagram</span>
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
        <span class="text-white">Discord</span>
        <span class="ml-auto truncate">Join the server</span>
      </a>
    </li>
    <li>
      <a href="mailto:contact@openstreetlifting.org" class={rowClass}>
        <EmailIcon class="h-5 w-5 shrink-0" />
        <span class="text-white">Email</span>
        <span class="ml-auto truncate">contact@openstreetlifting.org</span>
      </a>
    </li>
  </ul>
</div>
