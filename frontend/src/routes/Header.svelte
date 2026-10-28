<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import LanguagePicker from '#lib/components/language-picker.svelte';
  import { page } from '$app/state';
  import { afterNavigate } from '$app/navigation';
  import { resolve } from '$app/paths';
  import { GitHubIcon, InstagramIcon, MenuIcon, CloseIcon } from '#lib/components/icons/index.js';
  import { stripLocale } from '#lib/i18n/paths.js';
  import { NAV_LINK } from '#lib/constants/typography.js';

  const i18n = useI18n();

  let menuOpen = $state(false);

  afterNavigate(({ shallow }) => {
    if (shallow) return;

    menuOpen = false;
  });

  function isActive(path: string): boolean {
    const pathname = stripLocale(page.url.pathname);
    return path === '/' ? pathname === '/' : pathname.startsWith(path);
  }
</script>

<svelte:window
  onkeydown={(event) => {
    if (event.key === 'Escape') menuOpen = false;
  }}
/>

<header class="bg-canvas">
  <nav
    class="mx-auto flex max-w-page flex-wrap items-center justify-between px-4 py-3 sm:px-6 sm:py-4"
  >
    <a
      href={i18n.href(resolve(''))}
      title={i18n.t('OpenStreetlifting, back to the full rankings')}
      class="opacity-90 transition-opacity hover:opacity-100"
    >
      <img src="/logo_width.svg" alt="OpenStreetlifting" class="h-7 w-auto sm:h-8" />
    </a>

    <button
      type="button"
      onclick={() => (menuOpen = !menuOpen)}
      aria-expanded={menuOpen}
      aria-controls="primary-nav"
      aria-label={menuOpen ? i18n.t('Close menu') : i18n.t('Open menu')}
      class="rounded-lg p-2 text-secondary transition-colors hover:text-ink focus:ring-2 focus:ring-focus focus:ring-offset-2 focus:ring-offset-canvas focus:outline-none md:hidden"
    >
      {#if menuOpen}
        <CloseIcon />
      {:else}
        <MenuIcon />
      {/if}
    </button>

    <ul
      id="primary-nav"
      class="{menuOpen
        ? 'flex'
        : 'hidden'} w-full flex-col gap-4 pt-4 pb-2 md:flex md:w-auto md:flex-row md:items-center md:gap-6 md:pt-0 md:pb-0"
    >
      <li>
        <a
          href={i18n.href(resolve(''))}
          class={NAV_LINK}
          class:text-ink={isActive('/')}
          class:text-secondary={!isActive('/')}
          aria-current={isActive('/') ? 'page' : undefined}
        >
          {i18n.t('Rankings')}
        </a>
      </li>
      <li>
        <a
          href={i18n.href(resolve('competitions'))}
          class={NAV_LINK}
          class:text-ink={isActive('/competitions')}
          class:text-secondary={!isActive('/competitions')}
          aria-current={isActive('/competitions') ? 'page' : undefined}
        >
          {i18n.t('Competitions')}
        </a>
      </li>
      <li>
        <a
          href={i18n.href(resolve('ris'))}
          class={NAV_LINK}
          class:text-ink={isActive('/ris')}
          class:text-secondary={!isActive('/ris')}
          aria-current={isActive('/ris') ? 'page' : undefined}>RIS</a
        >
      </li>
      <li>
        <a
          href="https://docs.openstreetlifting.org/"
          target="_blank"
          rel="noopener noreferrer"
          class={NAV_LINK}
        >
          {i18n.t('Docs')}
        </a>
      </li>
      <li>
        <a
          href={i18n.href(resolve('contact'))}
          class={NAV_LINK}
          class:text-ink={isActive('/contact')}
          class:text-secondary={!isActive('/contact')}
          aria-current={isActive('/contact') ? 'page' : undefined}
        >
          {i18n.t('Contact')}
        </a>
      </li>
      <li><LanguagePicker /></li>
      <li class="flex items-center gap-3">
        <a
          href="https://github.com/openstreetlifting/openstreetlifting"
          target="_blank"
          rel="noopener noreferrer"
          class="text-secondary transition-colors hover:text-ink"
          aria-label="GitHub"
        >
          <GitHubIcon class="h-5 w-5" />
        </a>
        <a
          href="https://www.instagram.com/openstreetlifting"
          target="_blank"
          rel="noopener noreferrer"
          class="text-secondary transition-colors hover:text-ink"
          aria-label="Instagram"
        >
          <InstagramIcon class="h-5 w-5" />
        </a>
      </li>
    </ul>
  </nav>
  <div class="h-px bg-stroke"></div>
</header>
