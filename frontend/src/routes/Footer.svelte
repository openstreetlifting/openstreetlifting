<script lang="ts">
  import { resolve } from '$app/paths';
  import { GitHubIcon, InstagramIcon, DiscordIcon, EmailIcon } from '$lib/components/icons';
  import { buildInfo } from '$lib/version';
  import { NAV_LINK } from '$lib/constants/typography';

  const LINK = `${NAV_LINK} text-zinc-400`;
  const ICON = 'text-zinc-400 transition-colors hover:text-white';
</script>

<!-- Five links do not need four headed columns: headings for one link each are
     what turned this into a screen of its own on a phone. One row that wraps
     reads the same at every width. -->
<footer class="border-t border-zinc-800/50 bg-zinc-950">
  <div class="mx-auto max-w-[var(--content-max-width)] px-4 py-6 sm:px-6 sm:py-10">
    <div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
      <nav aria-label="Footer" class="flex flex-wrap gap-x-5 gap-y-2">
        <a href={resolve('/')} class={LINK}>Rankings</a>
        <a href={resolve('/competitions')} class={LINK}>Competitions</a>
        <a
          href="https://docs.openstreetlifting.org/"
          target="_blank"
          rel="noopener noreferrer"
          class={LINK}
        >
          Documentation
        </a>
        <a href={resolve('/contact')} class={LINK}>Contact</a>
        <a href={resolve('/privacy')} class={LINK}>Privacy</a>
      </nav>

      <div class="flex gap-4">
        <a
          href="https://github.com/openstreetlifting"
          target="_blank"
          rel="noopener noreferrer"
          class={ICON}
          aria-label="GitHub"
        >
          <GitHubIcon class="h-5 w-5" />
        </a>
        <a
          href="https://www.instagram.com/openstreetlifting"
          target="_blank"
          rel="noopener noreferrer"
          class={ICON}
          aria-label="Instagram"
        >
          <InstagramIcon class="h-5 w-5" />
        </a>
        <a
          href="https://discord.gg/MR9z7AnT"
          target="_blank"
          rel="noopener noreferrer"
          class="text-zinc-400 transition-colors hover:text-indigo-400"
          aria-label="Discord"
        >
          <DiscordIcon class="h-5 w-5" />
        </a>
        <a href="mailto:contact@openstreetlifting.org" class={ICON} aria-label="Email">
          <EmailIcon class="h-5 w-5" />
        </a>
      </div>
    </div>

    <div
      class="mt-5 flex flex-col gap-1.5 border-t border-zinc-800/50 pt-4 sm:mt-6 sm:flex-row sm:items-center sm:justify-between sm:pt-5"
    >
      <p class="text-xs font-light text-zinc-500">
        Code licensed under
        <a
          href="https://www.gnu.org/licenses/agpl-3.0.html"
          target="_blank"
          rel="noopener noreferrer"
          class="text-zinc-400 transition-colors hover:text-white">AGPLv3</a
        >
        &middot; Data licensed under
        <a
          href="https://creativecommons.org/licenses/by/4.0/"
          target="_blank"
          rel="noopener noreferrer"
          class="text-zinc-400 transition-colors hover:text-white">CC BY 4.0</a
        >
      </p>
      <p class="font-mono text-xs font-light text-zinc-600">
        {#if buildInfo.label}
          <span class="text-zinc-500">{buildInfo.label}</span>
        {/if}
        {#if buildInfo.version}
          <a
            href="https://github.com/openstreetlifting/openstreetlifting/releases/tag/v{buildInfo.version}"
            target="_blank"
            rel="noopener noreferrer"
            class="transition-colors hover:text-white">v{buildInfo.version}</a
          >
        {/if}
        {#if buildInfo.shortSha}
          {#if buildInfo.label || buildInfo.version}<span aria-hidden="true">&middot;</span>{/if}
          <a
            href="https://github.com/openstreetlifting/openstreetlifting/commit/{buildInfo.sha}"
            target="_blank"
            rel="noopener noreferrer"
            class="transition-colors hover:text-white">{buildInfo.shortSha}</a
          >
        {:else if !buildInfo.label && !buildInfo.version}
          dev
        {/if}
      </p>
    </div>
  </div>
</footer>
