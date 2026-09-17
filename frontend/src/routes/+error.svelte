<script lang="ts">
  import { resolve } from '$app/paths';
  import { page } from '$app/state';
  import Button from '$lib/components/ui/button.svelte';
  import { TEXT } from '$lib/constants/typography';

  const copy = $derived.by(() => {
    if (page.status === 404) {
      return {
        label: 'Page not found',
        title: 'This page has left the platform.',
        description: '',
      };
    }
    return {
      label: 'Something went wrong',
      title: 'We couldn’t load this page.',
      description: 'Please try again later.',
    };
  });
</script>

<svelte:head>
  <title>{page.status} · {copy.label} | OpenStreetlifting</title>
  <meta name="robots" content="noindex, follow" />
</svelte:head>

<div
  class="mx-auto flex min-h-[65vh] max-w-reading flex-col items-center justify-center px-6 py-12 text-center sm:py-16"
>
  <img
    src="/logo_plate_full.svg"
    alt=""
    aria-hidden="true"
    width="160"
    height="160"
    class="mb-8 h-32 w-32 sm:h-40 sm:w-40"
  />
  <p class="mb-3 text-sm font-medium text-secondary">{page.status} · {copy.label}</p>
  <h1 class="{TEXT.title} text-balance text-ink">{copy.title}</h1>
  {#if copy.description}
    <p class="mt-3 max-w-sm text-sm leading-relaxed text-pretty text-secondary">
      {copy.description}
    </p>
  {/if}
  <div class="mt-8 flex w-full max-w-sm flex-col gap-3 sm:w-auto sm:flex-row">
    <Button href={resolve('/')} class="min-h-11 text-sm">Back to rankings</Button>
    <Button href={resolve('/contact')} variant="outline" class="min-h-11 text-sm text-ink">
      Get in touch
    </Button>
  </div>
</div>
