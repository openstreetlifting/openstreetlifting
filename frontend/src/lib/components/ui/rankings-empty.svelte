<script lang="ts">
  import type { ResolvedPathname } from '$app/types';
  import { SearchIcon } from '$lib/components/icons';
  import { EDGE_TO_EDGE } from '$lib/constants/table';
  import { TEXT } from '$lib/constants/typography';
  import Button from './button.svelte';

  let { canReset, resetHref }: { canReset: boolean; resetHref: ResolvedPathname } = $props();
</script>

<div
  class="{EDGE_TO_EDGE} flex min-h-80 items-center justify-center border-y border-stroke bg-surface-subtle px-6 py-10 sm:rounded-lg sm:border sm:py-14"
>
  <div class="flex max-w-sm flex-col items-center text-center">
    <div
      class="mb-5 flex size-12 items-center justify-center rounded-full bg-surface-selected text-secondary"
      aria-hidden="true"
    >
      <SearchIcon class="size-5" />
    </div>

    <div role="status" aria-live="polite" aria-atomic="true">
      <h2 class="{TEXT.title} text-balance text-ink">
        {canReset ? 'Oops, no athletes found.' : 'No athletes to show yet.'}
      </h2>
      <p class="mt-3 text-sm leading-relaxed text-pretty text-secondary">
        {#if canReset}
          The filters might be doing a little too much heavy lifting. Try another name or clear them
          for a fresh start.
        {:else}
          There are no ranked results in this view yet. Check back as more results are added.
        {/if}
      </p>
    </div>

    {#if canReset}
      <Button href={resetHref} class="mt-6 min-h-11 w-full text-sm active:scale-[0.98] sm:w-auto">
        Clear search &amp; filters
      </Button>
    {/if}
  </div>
</div>
