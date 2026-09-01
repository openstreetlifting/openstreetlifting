<script lang="ts">
  import { page } from '$app/state';
  import { afterNavigate } from '$app/navigation';
  import { resolve } from '$app/paths';
  import { GitHubIcon, InstagramIcon, MenuIcon, CloseIcon } from '$lib/components/icons';
  import { NAV_LINK } from '$lib/constants/typography';

  const linkClass = NAV_LINK;

  let menuOpen = $state(false);

  afterNavigate(() => {
    menuOpen = false;
  });

  function isActive(path: string): boolean {
    return path === '/' ? page.url.pathname === '/' : page.url.pathname.startsWith(path);
  }
</script>

<svelte:window
  onkeydown={(event) => {
    if (event.key === 'Escape') menuOpen = false;
  }}
/>

<header class="bg-zinc-950">
  <nav
    class="mx-auto flex max-w-[var(--content-max-width)] flex-wrap items-center justify-between px-4 py-3 sm:px-6 sm:py-4"
  >
    <a
      href={resolve('/')}
      title="OpenStreetlifting, back to the full rankings"
      class="opacity-90 transition-opacity hover:opacity-100"
    >
      <img src="/logo_width.svg" alt="OpenStreetlifting" class="h-7 w-auto sm:h-8" />
    </a>

    <button
      type="button"
      onclick={() => (menuOpen = !menuOpen)}
      aria-expanded={menuOpen}
      aria-controls="primary-nav"
      aria-label={menuOpen ? 'Close menu' : 'Open menu'}
      class="rounded-lg p-2 text-zinc-400 transition-colors hover:text-white focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none md:hidden"
    >
      {#if menuOpen}
        <CloseIcon />
      {:else}
        <MenuIcon />
      {/if}
    </button>

    <!-- Records and FAQ are announced but muted until the routes exist. -->
    {#snippet comingSoon(label: string)}
      <li>
        <span
          class="{NAV_LINK} cursor-not-allowed text-zinc-600 hover:text-zinc-600"
          aria-disabled="true"
          title="Coming soon"
        >
          {label}
        </span>
      </li>
    {/snippet}

    <ul
      id="primary-nav"
      class="{menuOpen
        ? 'flex'
        : 'hidden'} w-full flex-col gap-4 pt-4 pb-2 md:flex md:w-auto md:flex-row md:items-center md:gap-6 md:pt-0 md:pb-0"
    >
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
      <li class="flex items-center gap-3">
        <a
          href="https://github.com/openstreetlifting/openstreetlifting"
          target="_blank"
          rel="noopener noreferrer"
          class="text-zinc-400 transition-colors hover:text-white"
          aria-label="GitHub"
        >
          <GitHubIcon class="h-5 w-5" />
        </a>
        <a
          href="https://www.instagram.com/openstreetlifting"
          target="_blank"
          rel="noopener noreferrer"
          class="text-zinc-400 transition-colors hover:text-white"
          aria-label="Instagram"
        >
          <InstagramIcon class="h-5 w-5" />
        </a>
      </li>
    </ul>
  </nav>
  <div class="h-px bg-zinc-800/50"></div>
</header>
