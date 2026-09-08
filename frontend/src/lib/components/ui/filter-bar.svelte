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
    onClear?: () => void;
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
    class="{FIELD} {CONTROL} flex shrink-0 items-center gap-1.5 px-3 py-2 text-secondary hover:border-stroke-strong hover:text-ink"
  >
    <CloseIcon class="h-3 w-3" />
    Clear
  </button>
{/snippet}

<div
  class="{EDGE_TO_EDGE} mb-4 flex flex-wrap items-center gap-3 rounded-none border border-x-0 border-stroke bg-surface p-3 sm:mb-6 sm:rounded-lg sm:border-x"
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
      class="{FIELD} {CONTROL} flex shrink-0 items-center gap-1.5 px-3 py-2 text-secondary sm:hidden"
    >
      Filters
      {#if activeCount > 0}
        <span class="rounded bg-action px-1 text-[0.65rem] font-semibold text-on-action">
          {activeCount}
        </span>
      {/if}
      <ChevronIcon class="h-3.5 w-3.5 transition-transform {open ? 'rotate-180' : ''}" />
    </button>
    {#if showsClear}
      <div class="sm:hidden">{@render clear()}</div>
    {/if}
  </div>

  <div
    id="filter-panel"
    class="{open ? 'flex' : 'hidden'} w-full flex-wrap items-center gap-3 sm:contents"
  >
    {@render children()}
    {#if showsClear}
      <div class="hidden sm:ml-auto sm:block">{@render clear()}</div>
    {/if}
  </div>
</div>
