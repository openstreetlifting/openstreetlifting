<script lang="ts">
  import Header from './Header.svelte';
  import Footer from './Footer.svelte';
  import { umami } from '$lib/analytics';
  import '../app.css';

  let { children } = $props();
</script>

<svelte:head>
  {#if umami.enabled}
    <script defer src={umami.scriptUrl} data-website-id={umami.websiteId}></script>
  {/if}
</svelte:head>

<div class="flex min-h-screen flex-col bg-zinc-950">
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
