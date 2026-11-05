<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import type { Snippet } from 'svelte';
  import SearchInput from './search-input.svelte';
  import { ChevronIcon, CloseIcon } from '#lib/components/icons/index.js';
  import { EDGE_TO_EDGE } from '#lib/constants/table.js';
  import Button from './button.svelte';

  const i18n = useI18n();

  interface Props {
    search: string;
    placeholder?: string;
    onSearch: () => void;
    activeCount?: number;
    onClear?: () => void;
    clearable?: boolean;
    children: Snippet;
    primary?: Snippet;
    embedded?: boolean;
  }

  let {
    search = $bindable(),
    placeholder = 'Search',
    onSearch,
    activeCount = 0,
    onClear,
    clearable,
    children,
    primary,
    embedded = false,
  }: Props = $props();

  let open = $state(false);
  const panelId = $props.id();

  const showsClear = $derived(
    Boolean(onClear) && (clearable ?? (activeCount > 0 || search.trim().length > 0))
  );
</script>

{#snippet clear()}
  <Button
    variant="outline"
    size="control"
    type="button"
    onclick={onClear}
    aria-label={i18n.t('Clear all filters and sorting')}
    class="shrink-0 gap-1.5 text-meta text-secondary hover:text-ink"
  >
    <CloseIcon class="h-3 w-3" />
    {i18n.t('Clear')}
  </Button>
{/snippet}

<div
  class="{embedded
    ? 'border-b border-stroke'
    : `${EDGE_TO_EDGE} mb-4 rounded-panel border border-stroke-strong sm:mb-6`} grid grid-cols-2 items-end gap-2 bg-surface p-3 sm:gap-3 {primary
    ? 'sm:grid-cols-4 xl:grid-cols-8'
    : 'sm:grid-cols-2 xl:grid-cols-6'}"
>
  <div class="col-span-2 flex min-w-0 items-center gap-2">
    <div class="min-w-0 flex-1">
      <SearchInput bind:value={search} {placeholder} {onSearch} />
    </div>

    <Button
      variant="outline"
      size="control"
      type="button"
      onclick={() => (open = !open)}
      aria-expanded={open}
      aria-controls={panelId}
      class="shrink-0 gap-1.5 text-meta text-secondary sm:hidden"
    >
      {i18n.t('Filters')}
      {#if activeCount > 0}
        <span class="bg-action px-1 text-micro font-semibold text-on-action">
          {activeCount}
        </span>
      {/if}
      <ChevronIcon class="h-3.5 w-3.5 {open ? 'rotate-180' : ''}" />
    </Button>
    {#if showsClear}<div class="hidden shrink-0 sm:block">{@render clear()}</div>{/if}
  </div>

  {#if primary}
    <div class="col-span-2 grid min-w-0 grid-cols-2 gap-2 sm:contents">
      {@render primary()}
    </div>
  {/if}

  <div
    id={panelId}
    class="{open
      ? 'grid'
      : 'hidden'} col-span-2 min-w-0 grid-cols-2 items-end gap-2 border-t border-stroke pt-3 sm:contents"
  >
    {@render children()}
    {#if showsClear}
      <div class="col-span-2 sm:hidden">{@render clear()}</div>
    {/if}
  </div>
</div>
