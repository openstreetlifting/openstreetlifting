<script module lang="ts">
  export const TABLE_HEAD_CELL = 'px-1.5 py-1 text-left align-middle font-medium sm:px-3 sm:py-2';
  export const TABLE_CELL = 'px-1.5 py-1 sm:px-3 sm:py-2';
  export const FROZEN_CELL = 'sticky z-10 bg-[var(--row-bg)]';
  export const FROZEN_HEAD_CELL = 'sticky z-10 bg-surface';
  export const FROZEN_EDGE = 'border-r border-stroke sm:border-r-0';
  export const FROZEN_RANK = 'w-10 min-w-10 text-right sm:w-14 sm:min-w-14 left-0';
  export const ATHLETE_COLUMN = 'w-44 min-w-44 pl-1 sm:w-56 sm:min-w-56 sm:pl-3';
  export const ATHLETE_CONTENT = 'max-w-[10.5rem] sm:max-w-[12.5rem]';
</script>

<script lang="ts" generics="Row">
  import type { Snippet } from 'svelte';
  import { EDGE_TO_EDGE } from '$lib/constants/table';
  import { TABLE_PAGE_SIZE } from '$lib/constants/pagination';
  import type { TablePagination } from '$lib/types/pagination';
  import { page as currentPage } from '$app/state';
  import { SvelteURLSearchParams } from 'svelte/reactivity';
  import Pagination from './pagination.svelte';

  let {
    head,
    body,
    rows,
    itemName = 'entry',
    pagination,
    pageParam = 'table_page',
    busy = false,
  }: {
    head: Snippet;
    body: Snippet<[Row[]]>;
    rows: Row[];
    itemName?: 'athlete' | 'competition' | 'country' | 'federation' | 'entry';
    pagination?: TablePagination;
    pageParam?: string;
    busy?: boolean;
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
  const countLabel = $derived(
    totalItems === 1
      ? itemName
      : {
          athlete: 'athletes',
          competition: 'competitions',
          country: 'countries',
          federation: 'federations',
          entry: 'entries',
        }[itemName]
  );

  function pageHref(target: number): string {
    if (pagination?.pageHref) return pagination.pageHref(target);
    const params = new SvelteURLSearchParams(currentPage.url.searchParams);
    if (target > 1) params.set(pageParam, String(target));
    else params.delete(pageParam);
    const query = params.toString();
    // currentPage.url.pathname already includes the application base path.
    return `${currentPage.url.pathname}${query ? `?${query}` : ''}${currentPage.url.hash}`;
  }
</script>

{#snippet paginationBar(position: string)}
  <div class="flex flex-wrap items-center justify-between gap-3" data-table-pagination={position}>
    <span class="text-xs text-muted">
      Page {page} of {totalPages} &middot; {totalItems}
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

<div class="mb-3">{@render paginationBar('top')}</div>

<div class="osl-table overflow-x-auto {EDGE_TO_EDGE}">
  <table class="w-full text-[0.7rem] whitespace-nowrap sm:text-xs" aria-busy={busy}>
    <thead class="sticky top-0 z-20">
      <tr class="border-b border-stroke bg-surface">
        {@render head()}
      </tr>
    </thead>
    <tbody>
      {@render body(visibleRows)}
    </tbody>
  </table>
</div>

<div class="mt-3">{@render paginationBar('bottom')}</div>

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
</style>
