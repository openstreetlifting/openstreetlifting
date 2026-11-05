<script module lang="ts">
  export const TABLE_HEAD_CELL =
    'px-1.5 py-2 text-left align-middle font-mono text-[0.625rem] font-medium sm:px-3 sm:text-[0.6875rem]';
  export const TABLE_CELL = 'px-1.5 py-1 sm:px-3 sm:py-2';
  export const FROZEN_CELL = 'sticky z-10 bg-[var(--row-bg)]';
  export const FROZEN_HEAD_CELL = 'sticky z-10 bg-table-head';
  export const FROZEN_EDGE = 'border-r border-stroke sm:border-r-0';
  export const FROZEN_RANK =
    'w-10 min-w-10 max-w-10 text-right sm:w-14 sm:min-w-14 sm:max-w-14 left-0';
  export const ATHLETE_COLUMN = 'w-44 min-w-44 pl-1 sm:w-64 sm:min-w-64 sm:pl-3';
  export const ATHLETE_CONTENT = 'max-w-[10.25rem] sm:max-w-[14.5rem]';
  export const FROZEN_ATHLETE = 'left-10 border-r border-stroke-strong sm:left-14';
</script>

<script lang="ts" generics="Row">
  import { useI18n } from '#lib/i18n/index.js';
  import type { Snippet } from 'svelte';
  import { EDGE_TO_EDGE } from '#lib/constants/table.js';
  import { TABLE_PAGE_SIZE } from '#lib/constants/pagination.js';
  import type { TablePagination } from '#lib/types/pagination.js';
  import { page as currentPage } from '$app/state';
  import { SvelteURLSearchParams } from 'svelte/reactivity';
  import Pagination from './pagination.svelte';

  const i18n = useI18n();

  let {
    head,
    body,
    rows,
    itemName = 'entry',
    pagination,
    pageParam = 'table_page',
    busy = false,
    embedded = false,
  }: {
    head: Snippet;
    body: Snippet<[Row[]]>;
    rows: Row[];
    itemName?: 'athlete' | 'competition' | 'country' | 'federation' | 'entry';
    pagination?: TablePagination;
    pageParam?: string;
    busy?: boolean;
    /** A parent results frame supplies the outer border and page gutters. */
    embedded?: boolean;
  } = $props();

  const totalItems = $derived(pagination?.total_items ?? rows.length);
  const totalPages = $derived(
    Math.max(1, pagination?.total_pages ?? Math.ceil(rows.length / TABLE_PAGE_SIZE))
  );
  const requestedPage = $derived(Number(currentPage.url.searchParams.get(pageParam)));
  const page = $derived(
    pagination?.page ??
      Math.min(totalPages, Math.max(1, Number.isInteger(requestedPage) ? requestedPage : 1))
  );
  const visibleRows = $derived(
    pagination ? rows : rows.slice((page - 1) * TABLE_PAGE_SIZE, page * TABLE_PAGE_SIZE)
  );
  const countLabel = $derived(i18n.count(itemName, totalItems));

  function pageHref(target: number): string {
    if (pagination?.pageHref) return pagination.pageHref(target);
    const params = new SvelteURLSearchParams(currentPage.url.search);
    if (target > 1) params.set(pageParam, String(target));
    else params.delete(pageParam);
    const query = params.toString();
    // currentPage.url.pathname already includes the application base path.
    return `${currentPage.url.pathname}${query ? `?${query}` : ''}${currentPage.url.hash}`;
  }
</script>

{#snippet paginationBar(position: string)}
  <div
    class="flex min-h-14 items-center justify-between gap-2 bg-surface-subtle px-2 py-1 sm:px-4 sm:py-2"
    data-table-pagination={position}
  >
    <span class="min-w-0 font-mono text-[0.625rem] text-muted sm:text-xs">
      {i18n.number(totalItems, 0)}
      {countLabel}
    </span>
    <Pagination
      {page}
      {totalPages}
      {pageHref}
      disabled={busy}
      replaceState={pagination?.replaceState}
    />
  </div>
{/snippet}

<div
  class={embedded ? 'min-w-0' : `min-w-0 rounded-panel border border-stroke-strong ${EDGE_TO_EDGE}`}
>
  <div class="border-b border-stroke">{@render paginationBar('top')}</div>

  <div class="osl-table overflow-x-auto">
    <table class="w-full text-xs leading-4 whitespace-nowrap" aria-busy={busy}>
      <thead class="sticky top-0 z-20">
        <tr class="border-b border-stroke-strong bg-table-head">
          {@render head()}
        </tr>
      </thead>
      <tbody>
        {@render body(visibleRows)}
      </tbody>
    </table>
  </div>

  <div class="border-t border-stroke">{@render paginationBar('bottom')}</div>
</div>

<style>
  .osl-table :global(tbody) {
    transition: opacity 120ms ease-out;
  }

  .osl-table :global(table[aria-busy='true'] tbody) {
    opacity: 0.55;
  }

  .osl-table :global(tbody tr) {
    --row-bg: var(--table-row);
    background-color: var(--row-bg);
    border-bottom: 1px solid var(--color-stroke);
  }

  .osl-table :global(tbody tr:nth-child(even)) {
    --row-bg: var(--table-row-alt);
  }

  .osl-table :global(tbody tr:hover) {
    --row-bg: var(--table-row-hover);
  }

  .osl-table :global(tbody tr[data-focused]) {
    --row-bg: var(--table-row-focus);
  }

  .osl-table :global(tbody tr:last-child) {
    border-bottom: 0;
  }

  @media (prefers-reduced-motion: reduce) {
    .osl-table :global(tbody) {
      transition: none;
    }
  }
</style>
