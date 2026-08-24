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
  <main class="flex-1">
    {@render children()}
  </main>
  <Footer />
</div>
