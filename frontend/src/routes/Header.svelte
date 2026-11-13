<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import LanguagePicker from '#lib/components/language-picker.svelte';
  import WeightUnitPicker from '#lib/components/weight-unit-picker.svelte';
  import { page } from '$app/state';
  import { afterNavigate } from '$app/navigation';
  import { resolve } from '$app/paths';
  import { GitHubIcon, InstagramIcon, MenuIcon, CloseIcon } from '#lib/components/icons/index.js';
  import { stripLocale } from '#lib/i18n/paths.js';
  import { CONTROL, NAV_LINK } from '#lib/constants/typography.js';

  const i18n = useI18n();

  let menuOpen = $state(false);
  let menuButton: HTMLButtonElement;

  afterNavigate(({ shallow }) => {
    if (shallow) return;

    menuOpen = false;
  });

  function isActive(path: string): boolean {
    const pathname = stripLocale(page.url.pathname);
    return path === '/' ? pathname === '/' : pathname.startsWith(path);
  }

  const menuLinks = $derived([
    { path: '', label: i18n.t('Rankings'), active: isActive('/') },
    { path: 'competitions', label: i18n.t('Competitions'), active: isActive('/competitions') },
    { path: 'records', label: i18n.t('Records'), active: isActive('/records') },
    { path: 'ris', label: 'RIS', active: isActive('/ris') },
    { path: 'contact', label: i18n.t('Contact'), active: isActive('/contact') },
  ] as const);

  const MENU_ITEM = 'border-b border-stroke sm:border-r sm:nth-[3n]:border-r-0';
  const MENU_LINK = `${CONTROL} flex h-control items-center justify-between gap-2 px-gutter transition-colors focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-focus`;
  const SOCIAL =
    'inline-flex size-control items-center justify-center text-secondary transition-colors hover:text-ink focus-visible:outline-2 focus-visible:outline-focus lg:h-9 lg:w-5';
</script>

<svelte:window
  onkeydown={(event) => {
    if (event.key === 'Escape' && menuOpen) {
      menuOpen = false;
      menuButton.focus();
    }
  }}
/>

{#snippet primaryLinks()}
  <li class="min-w-0 flex-1 lg:flex-none">
    <a
      href={i18n.href(resolve(''))}
      class="{NAV_LINK} w-full justify-start border-b-0 border-transparent px-0 lg:justify-center lg:border-b-2"
      class:border-ink={isActive('/')}
      class:text-ink={isActive('/')}
      aria-current={isActive('/') ? 'page' : undefined}>{i18n.t('Rankings')}</a
    >
  </li>
  <li class="min-w-0 flex-1 lg:flex-none">
    <a
      href={i18n.href(resolve('competitions'))}
      class="{NAV_LINK} w-full justify-start border-b-0 border-transparent px-0 lg:justify-center lg:border-b-2"
      class:border-ink={isActive('/competitions')}
      class:text-ink={isActive('/competitions')}
      aria-current={isActive('/competitions') ? 'page' : undefined}>{i18n.t('Competitions')}</a
    >
  </li>
  <li class="min-w-0 flex-1 lg:flex-none">
    <a
      href={i18n.href(resolve('records'))}
      class="{NAV_LINK} w-full justify-start border-b-0 border-transparent px-0 lg:justify-center lg:border-b-2"
      class:border-ink={isActive('/records')}
      class:text-ink={isActive('/records')}
      aria-current={isActive('/records') ? 'page' : undefined}>{i18n.t('Records')}</a
    >
  </li>
{/snippet}

{#snippet secondaryLinks()}
  <li>
    <a
      href={i18n.href(resolve('ris'))}
      class={NAV_LINK}
      class:text-ink={isActive('/ris')}
      aria-current={isActive('/ris') ? 'page' : undefined}>RIS</a
    >
  </li>
  <li>
    <a
      href="https://docs.openstreetlifting.org/"
      target="_blank"
      rel="noopener noreferrer"
      class={NAV_LINK}>{i18n.t('Docs')}</a
    >
  </li>
  <li>
    <a
      href={i18n.href(resolve('contact'))}
      class={NAV_LINK}
      class:text-ink={isActive('/contact')}
      aria-current={isActive('/contact') ? 'page' : undefined}>{i18n.t('Contact')}</a
    >
  </li>
{/snippet}

<header class="border-b border-stroke-strong bg-canvas">
  <nav
    class="mx-auto grid max-w-page grid-cols-[minmax(0,1fr)_auto] items-center gap-x-4 px-gutter lg:flex lg:gap-6"
  >
    <a
      href={i18n.href(resolve(''))}
      title={i18n.t('OpenStreetlifting, back to the full rankings')}
      class="flex min-h-12 min-w-0 items-center lg:min-h-14 justify-self-start py-2 opacity-90 transition-opacity hover:opacity-100 focus-visible:outline-2 focus-visible:outline-focus"
    >
      <img
        src="/logo_width.svg"
        width="3442"
        height="512"
        alt="OpenStreetlifting"
        fetchpriority="high"
        class="h-6 w-auto max-w-full sm:h-7"
      />
    </a>

    <button
      bind:this={menuButton}
      type="button"
      onclick={() => (menuOpen = !menuOpen)}
      aria-expanded={menuOpen}
      aria-controls="primary-nav"
      aria-label={menuOpen ? i18n.t('Close menu') : i18n.t('Open menu')}
      class="flex size-11 items-center justify-center rounded-control text-secondary transition-colors hover:bg-surface hover:text-ink focus-visible:outline-2 focus-visible:outline-focus lg:hidden"
    >
      {#if menuOpen}<CloseIcon />{:else}<MenuIcon />{/if}
    </button>

    <ul class="hidden items-center gap-6 lg:ml-auto lg:flex">
      {@render primaryLinks()}
      {@render secondaryLinks()}
    </ul>

    <div
      id="primary-nav"
      class="{menuOpen
        ? 'flex'
        : 'hidden'} col-span-2 row-start-2 -mx-gutter flex-col border-t border-stroke lg:col-span-1 lg:col-start-3 lg:row-start-1 lg:mx-0 lg:flex lg:flex-row lg:items-center lg:justify-self-end lg:border-0"
    >
      <ul class="grid sm:grid-cols-3 lg:hidden">
        {#each menuLinks as link (link.label)}
          <li class={MENU_ITEM}>
            <a
              href={i18n.href(resolve(link.path))}
              aria-current={link.active ? 'page' : undefined}
              class="{MENU_LINK} {link.active
                ? 'bg-surface-selected text-ink'
                : 'text-secondary hover:bg-surface hover:text-ink'}">{link.label}</a
            >
          </li>
        {/each}
        <li class={MENU_ITEM}>
          <a
            href="https://docs.openstreetlifting.org/"
            target="_blank"
            rel="noopener noreferrer"
            class="{MENU_LINK} text-secondary hover:bg-surface hover:text-ink"
            >{i18n.t('Docs')}<span aria-hidden="true" class="text-muted">↗</span></a
          >
        </li>
      </ul>
      <div
        class="flex items-center justify-between gap-3 bg-surface-subtle px-gutter py-1.5 lg:justify-start lg:bg-transparent lg:p-0"
      >
        <div class="flex items-center gap-2"><LanguagePicker /><WeightUnitPicker /></div>
        <div class="flex items-center gap-1 lg:gap-3">
          <a
            href="https://github.com/openstreetlifting/openstreetlifting"
            target="_blank"
            rel="noopener noreferrer"
            class={SOCIAL}
            aria-label="GitHub"><GitHubIcon class="h-5 w-5" /></a
          >
          <a
            href="https://www.instagram.com/openstreetlifting"
            target="_blank"
            rel="noopener noreferrer"
            class={SOCIAL}
            aria-label="Instagram"><InstagramIcon class="h-5 w-5" /></a
          >
        </div>
      </div>
    </div>
  </nav>
</header>
