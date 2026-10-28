<script lang="ts">
  import { provideI18n } from '#lib/i18n/index.js';
  import { provideWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import type { LayoutData } from './$types';
  import Header from './Header.svelte';
  import Footer from './Footer.svelte';
  import { umami } from '#lib/analytics.js';
  import '../app.css';

  let { data, children }: { data: LayoutData; children: import('svelte').Snippet } = $props();
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

<div class="flex min-h-screen flex-col bg-canvas">
  <Header />
  <!-- min-w-0 because a flex item defaults to min-width:auto, which refuses to
       shrink below its content. Without it a table wider than the screen widens
       this instead of scrolling inside its own container, and the whole page
       pans sideways. -->
  <main class="min-w-0 flex-1">
    {@render children()}
  </main>
  <Footer />
</div>
