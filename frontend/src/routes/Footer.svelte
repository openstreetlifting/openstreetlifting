<script lang="ts">
  import { resolve } from '$app/paths';
  import { GitHubIcon, InstagramIcon, DiscordIcon, EmailIcon } from '$lib/components/icons';
  import { buildInfo } from '$lib/version';
  import { NAV_LINK } from '$lib/constants/typography';

  const ICON = 'text-secondary transition-colors hover:text-ink';
</script>

<footer class="border-t border-stroke bg-canvas">
  <div class="mx-auto max-w-page px-4 py-6 sm:px-6 sm:py-10">
    <div class="flex flex-wrap items-center gap-x-8 gap-y-5">
      <nav aria-label="Footer" class="flex flex-wrap items-center gap-x-5 gap-y-2">
        <a href={resolve('/')} class={NAV_LINK}>Rankings</a>
        <a href={resolve('/competitions')} class={NAV_LINK}>Competitions</a>
        <a
          href="https://docs.openstreetlifting.org/"
          target="_blank"
          rel="noopener noreferrer"
          class={NAV_LINK}
        >
          Documentation
        </a>
        <a href={resolve('/contact')} class={NAV_LINK}>Contact</a>
        <a href={resolve('/privacy')} class={NAV_LINK}>Privacy</a>
      </nav>

      <p class="text-sm text-muted">
        Code licensed under
        <a
          href="https://www.gnu.org/licenses/agpl-3.0.html"
          target="_blank"
          rel="noopener noreferrer"
          class="text-secondary transition-colors hover:text-ink">AGPLv3</a
        >
        &middot; Data licensed under
        <a
          href="https://creativecommons.org/licenses/by/4.0/"
          target="_blank"
          rel="noopener noreferrer"
          class="text-secondary transition-colors hover:text-ink">CC BY 4.0</a
        >
      </p>

      <div class="flex items-center gap-4 xl:ml-auto">
        <span class="font-mono text-sm text-muted">
          {#if buildInfo.label}
            {buildInfo.label}
          {/if}
          {#if buildInfo.version}
            <a
              href="https://github.com/openstreetlifting/openstreetlifting/releases/tag/v{buildInfo.version}"
              target="_blank"
              rel="noopener noreferrer"
              class="transition-colors hover:text-ink">v{buildInfo.version}</a
            >
          {/if}
          {#if buildInfo.shortSha}
            {#if buildInfo.label || buildInfo.version}<span aria-hidden="true">&middot;</span>{/if}
            <a
              href="https://github.com/openstreetlifting/openstreetlifting/commit/{buildInfo.sha}"
              target="_blank"
              rel="noopener noreferrer"
              class="transition-colors hover:text-ink">{buildInfo.shortSha}</a
            >
          {:else if !buildInfo.label && !buildInfo.version}
            dev
          {/if}
        </span>
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
          class="text-secondary transition-colors hover:text-ink"
          aria-label="Discord"
        >
          <DiscordIcon class="h-5 w-5" />
        </a>
        <a href="mailto:contact@openstreetlifting.org" class={ICON} aria-label="Email">
          <EmailIcon class="h-5 w-5" />
        </a>
      </div>
    </div>
  </div>
</footer>
