<script lang="ts">
  import { provideI18n } from '#lib/i18n/index.js';
  import { provideWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import type { LayoutData } from './$types';
  import Header from './Header.svelte';
  import Footer from './Footer.svelte';
  import { umami } from '#lib/analytics.js';
  import '../app.css';

  let { data, children }: { data: LayoutData; children: import('svelte').Snippet } = $props();
  const dev = import.meta.env.DEV;
  const i18n = provideI18n(() => data.locale);
  provideWeightUnit();

  $effect(() => {
    document.documentElement.lang = data.locale;
  });
</script>

<svelte:head>
  <link rel="manifest" href={i18n.href('/manifest.webmanifest')} />
  {#if umami.enabled}
    <script defer src={umami.scriptUrl} data-website-id={umami.websiteId}></script>
  {/if}
</svelte:head>

<div class="flex min-h-dvh flex-col bg-canvas">
  <a
    href="#main-content"
    class="sr-only focus:not-sr-only focus:fixed focus:top-3 focus:left-3 focus:z-50 focus:bg-action focus:px-4 focus:py-3 focus:text-on-action focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus"
    >{i18n.t('Skip to main content')}</a
  >
  <Header />
  <!-- min-w-0 because a flex item defaults to min-width:auto, which refuses to
       shrink below its content. Without it a table wider than the screen widens
       this instead of scrolling inside its own container, and the whole page
       pans sideways. -->
  <main id="main-content" tabindex="-1" class="min-w-0 flex-1">
    {@render children()}
  </main>
  <Footer />
</div>

{#if dev}
  {#await import('#lib/components/dev/typeface-preview.svelte') then { default: TypefacePreview }}
    <TypefacePreview />
  {/await}
{/if}
