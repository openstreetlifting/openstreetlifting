<script lang="ts">
  import type { Snippet } from 'svelte';
  import SearchInput from './search-input.svelte';
  import { ChevronIcon, CloseIcon } from '$lib/components/icons';
  import { EDGE_TO_EDGE } from '$lib/constants/table';
  import { CONTROL, FIELD } from '$lib/constants/typography';

  interface Props {
    search: string;
    placeholder?: string;
    onSearch: () => void;
    activeCount?: number;
    /** Resets every filter and the sort. Omit it and no clear control is offered. */
    onClear?: () => void;
    /** Whether anything is worth clearing. Defaults to the filters and the search. */
    clearable?: boolean;
    children: Snippet;
  }

  let {
    search = $bindable(),
    placeholder = 'Search',
    onSearch,
    activeCount = 0,
    onClear,
    clearable,
    children,
  }: Props = $props();

  let open = $state(false);

  const showsClear = $derived(
    Boolean(onClear) && (clearable ?? (activeCount > 0 || search.trim().length > 0))
  );
</script>

{#snippet clear()}
  <button
    type="button"
    onclick={onClear}
    aria-label="Clear all filters and sorting"
    class="{FIELD} {CONTROL} flex shrink-0 items-center gap-1.5 px-3 py-2 text-zinc-400 hover:border-zinc-700 hover:text-white"
  >
    <CloseIcon class="h-3 w-3" />
    Clear
  </button>
{/snippet}

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

    <!-- The panel is collapsed by default on mobile, so clearing has to be
         reachable without opening it first. -->
    {#if showsClear}
      <div class="sm:hidden">{@render clear()}</div>
    {/if}
  </div>

  <div
    id="filter-panel"
    class="{open ? 'flex' : 'hidden'} w-full flex-wrap items-center gap-3 sm:contents"
  >
    {@render children()}

    <!-- The far right of the bar, so it reads as an action on all of the
         controls rather than on whichever one it landed beside. -->
    {#if showsClear}
      <div class="hidden sm:ml-auto sm:block">{@render clear()}</div>
    {/if}
  </div>
</div>
