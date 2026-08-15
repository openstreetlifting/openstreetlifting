<script lang="ts">
  import { resolve } from '$app/paths';
  import { GitHubIcon, InstagramIcon, DiscordIcon, EmailIcon } from '$lib/components/icons';
  import { buildInfo } from '$lib/version';
</script>

<footer class="border-t border-zinc-800/50 bg-zinc-950">
  <div class="mx-auto max-w-[var(--content-max-width)] px-6 py-16">
    <div class="grid gap-12 md:grid-cols-2 lg:grid-cols-4">
      <div>
        <h3 class="mb-4 text-sm font-medium text-white">Index</h3>
        <ul class="space-y-3">
          <li>
            <a
              href={resolve('/')}
              class="text-sm font-light text-zinc-400 transition-colors hover:text-white"
            >
              Rankings
            </a>
          </li>
          <li>
            <a
              href={resolve('/competitions')}
              class="text-sm font-light text-zinc-400 transition-colors hover:text-white"
            >
              Competitions
            </a>
          </li>
          <li>
            <a
              href={resolve('/athletes')}
              class="text-sm font-light text-zinc-400 transition-colors hover:text-white"
            >
              Athletes
            </a>
          </li>
        </ul>
      </div>

      <div>
        <h3 class="mb-4 text-sm font-medium text-white">Resources</h3>
        <ul class="space-y-3">
          <li>
            <a
              href="https://docs.openstreetlifting.org/"
              target="_blank"
              rel="noopener noreferrer"
              class="text-sm font-light text-zinc-400 transition-colors hover:text-white"
            >
              Documentation
            </a>
          </li>
          <li>
            <a
              href={resolve('/contact')}
              class="text-sm font-light text-zinc-400 transition-colors hover:text-white"
            >
              Contact
            </a>
          </li>
        </ul>
      </div>

      <div>
        <h3 class="mb-4 text-sm font-medium text-white">Legal</h3>
        <ul class="space-y-3">
          <li>
            <a
              href={resolve('/privacy')}
              class="text-sm font-light text-zinc-400 transition-colors hover:text-white"
            >
              Privacy Policy
            </a>
          </li>
        </ul>
      </div>
      <div>
        <h3 class="mb-4 text-sm font-medium text-white">Connect</h3>
        <div class="flex gap-3">
          <a
            href="https://github.com/openstreetlifting"
            target="_blank"
            rel="noopener noreferrer"
            class="text-zinc-400 transition-colors hover:text-white"
            aria-label="GitHub"
          >
            <GitHubIcon />
          </a>
          <a
            href="https://www.instagram.com/openstreetlifting"
            target="_blank"
            rel="noopener noreferrer"
            class="text-zinc-400 transition-colors hover:text-pink-500"
            aria-label="Instagram"
          >
            <InstagramIcon />
          </a>
          <a
            href="https://discord.gg/MR9z7AnT"
            target="_blank"
            rel="noopener noreferrer"
            class="text-zinc-400 transition-colors hover:text-indigo-400"
            aria-label="Discord"
          >
            <DiscordIcon />
          </a>
          <a
            href="mailto:adrien.pelfresne@gmail.com"
            class="text-zinc-400 transition-colors hover:text-white"
            aria-label="Email"
          >
            <EmailIcon />
          </a>
        </div>
      </div>
    </div>
    <div
      class="mt-12 flex flex-col gap-3 border-t border-zinc-800/50 pt-8 sm:flex-row sm:items-center sm:justify-between"
    >
      <p class="text-sm font-light text-zinc-500">
        Code licensed under
        <a
          href="https://www.gnu.org/licenses/agpl-3.0.html"
          target="_blank"
          rel="noopener noreferrer"
          class="text-zinc-400 transition-colors hover:text-white">AGPLv3</a
        >
        · Data licensed under
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
          {#if buildInfo.label || buildInfo.version}<span aria-hidden="true">·</span>{/if}
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
