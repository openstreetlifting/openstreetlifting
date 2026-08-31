<script lang="ts">
  import type { Snippet } from 'svelte';
  import SearchInput from './search-input.svelte';
  import { ChevronIcon } from '$lib/components/icons';
  import { EDGE_TO_EDGE } from '$lib/constants/table';
  import { CONTROL, FIELD } from '$lib/constants/typography';

  interface Props {
    search: string;
    placeholder?: string;
    onSearch: () => void;
    /** How many filters are set, so a closed panel still says it is narrowing the table. */
    activeCount?: number;
    children: Snippet;
  }

  let {
    search = $bindable(),
    placeholder = 'Search',
    onSearch,
    activeCount = 0,
    children,
  }: Props = $props();

  let open = $state(false);
</script>

<!-- Six controls above the table push the first rows off a phone screen, so
     below sm only the search stays out and the rest collapses behind a toggle.
     The panel is display:contents from sm, which drops the wrapper back out of
     the way and leaves the desktop bar laid out exactly as it was. -->
<div
  class="{EDGE_TO_EDGE} mb-4 flex flex-wrap items-center gap-3 rounded-none border border-x-0 border-zinc-800 bg-zinc-900/30 p-3 sm:mb-6 sm:rounded-lg sm:border-x"
>
  <div class="flex w-full items-center gap-2 sm:w-64">
    <div class="min-w-0 flex-1">
      <SearchInput bind:value={search} {placeholder} {onSearch} />
    </div>

    <button
      type="button"
      onclick={() => (open = !open)}
      aria-expanded={open}
      aria-controls="filter-panel"
      class="{FIELD} {CONTROL} flex shrink-0 items-center gap-1.5 px-3 py-2 text-zinc-300 sm:hidden"
    >
      Filters
      {#if activeCount > 0}
        <span class="rounded bg-white px-1 text-[0.65rem] font-semibold text-zinc-950">
          {activeCount}
        </span>
      {/if}
      <ChevronIcon class="h-3.5 w-3.5 transition-transform {open ? 'rotate-180' : ''}" />
    </button>
  </div>

  <div
    id="filter-panel"
    class="{open ? 'flex' : 'hidden'} w-full flex-wrap items-center gap-3 sm:contents"
  >
    {@render children()}
  </div>
</div>
