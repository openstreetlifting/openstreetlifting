<script lang="ts">
  import { page } from '$app/state';
  import { resolve } from '$app/paths';
  import { GitHubIcon, InstagramIcon } from '$lib/components/icons';

  const linkClass = 'text-sm font-medium transition-colors hover:text-white';

  function isActive(path: string): boolean {
    return path === '/' ? page.url.pathname === '/' : page.url.pathname.startsWith(path);
  }
</script>

<header class="bg-zinc-950">
  <nav class="mx-auto flex max-w-[var(--content-max-width)] items-center justify-between px-6 py-4">
    <a href={resolve('/')} class="opacity-90 transition-opacity hover:opacity-100">
      <img src="/logowidth.png" alt="OpenStreetlifting" class="h-8 w-auto" />
    </a>

    <!-- Records and FAQ are announced but muted until the routes exist. -->
    {#snippet comingSoon(label: string)}
      <li>
        <span
          class="cursor-not-allowed text-sm font-medium text-zinc-600"
          aria-disabled="true"
          title="Coming soon"
        >
          {label}
        </span>
      </li>
    {/snippet}

    <ul class="flex items-center gap-6">
      <li>
        <a
          href={resolve('/')}
          class={linkClass}
          class:text-white={isActive('/')}
          class:text-zinc-400={!isActive('/')}
          aria-current={isActive('/') ? 'page' : undefined}
        >
          Rankings
        </a>
      </li>
      <li>
        <a
          href={resolve('/competitions')}
          class={linkClass}
          class:text-white={isActive('/competitions')}
          class:text-zinc-400={!isActive('/competitions')}
          aria-current={isActive('/competitions') ? 'page' : undefined}
        >
          Competitions
        </a>
      </li>
      {@render comingSoon('Records')}
      {@render comingSoon('FAQ')}
      <li>
        <a
          href="https://docs.openstreetlifting.org/"
          target="_blank"
          rel="noopener noreferrer"
          class="{linkClass} text-zinc-400"
        >
          Docs
        </a>
      </li>
      <li>
        <a
          href={resolve('/contact')}
          class={linkClass}
          class:text-white={isActive('/contact')}
          class:text-zinc-400={!isActive('/contact')}
          aria-current={isActive('/contact') ? 'page' : undefined}
        >
          Contact
        </a>
      </li>
      <li>
        <a
          href="https://github.com/openstreetlifting/openstreetlifting"
          target="_blank"
          rel="noopener noreferrer"
          class="block text-zinc-400 transition-colors hover:text-white"
          aria-label="GitHub"
        >
          <GitHubIcon />
        </a>
      </li>
      <li>
        <a
          href="https://www.instagram.com/openstreetlifting"
          target="_blank"
          rel="noopener noreferrer"
          class="block text-zinc-400 transition-colors hover:text-pink-500"
          aria-label="Instagram"
        >
          <InstagramIcon />
        </a>
      </li>
    </ul>
  </nav>
  <div class="h-px bg-zinc-800/50"></div>
</header>
