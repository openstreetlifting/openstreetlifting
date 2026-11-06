<script module lang="ts">
  export const TABLE_HEAD_CELL =
    'px-1.5 py-1 text-left align-middle font-mono text-meta font-medium sm:px-3 sm:py-1.5';
  export const TABLE_CELL = 'px-1.5 py-1 sm:px-3 sm:py-1.5';
  export const FROZEN_CELL = 'sticky z-10 bg-[var(--row-bg)]';
  export const FROZEN_HEAD_CELL = 'sticky z-10 bg-table-head';
  export const FROZEN_RANK = 'w-8 min-w-8 max-w-8 sm:w-10 sm:min-w-10 sm:max-w-10 left-0';
  export const ATHLETE_COLUMN = 'w-36 min-w-36 pl-1 sm:w-64 sm:min-w-64 sm:pl-3';
  export const ATHLETE_COLUMN_FILL = 'w-36 min-w-36 pl-1 sm:w-full sm:min-w-64 sm:pl-3';
  export const ATHLETE_CONTENT = 'max-w-[8.25rem] sm:max-w-[14.5rem]';
  export const FROZEN_ATHLETE = 'left-8 sm:left-10';
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
    frozenColumns,
    label,
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
    frozenColumns?: 'rank' | 'athlete';
    label?: string;
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
  const tableLabel = $derived(
    label ??
      i18n.t(
        (
          {
            athlete: 'Athletes',
            competition: 'Competitions',
            country: 'Countries',
            federation: 'Federations',
            entry: 'Entries',
          } as const
        )[itemName]
      )
  );

  function revealFocus(event: FocusEvent) {
    const target = event.target;
    if (!frozenColumns || !(target instanceof HTMLElement)) return;
    if (target.closest('td, th')?.classList.contains('sticky')) return;

    // Native focus scrolling can leave links behind the frozen columns.
    requestAnimationFrame(() => {
      if (document.activeElement === target) {
        target.scrollIntoView({ block: 'nearest', inline: 'nearest' });
      }
    });
  }

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
    class="flex items-center justify-between gap-2 bg-surface-subtle px-3 py-1.5"
    data-table-pagination={position}
  >
    <span
      class="min-w-0 font-mono text-micro text-muted sm:text-meta"
      role={position === 'top' ? 'status' : undefined}
      aria-live={position === 'top' ? 'polite' : undefined}
      aria-atomic={position === 'top' ? true : undefined}
      aria-busy={position === 'top' ? busy : undefined}
    >
      {i18n.number(totalItems, 0)}
      {countLabel}
      {#if position === 'top'}
        <span class="sr-only">
          {i18n.t('Page {page} of {pages}.', {
            page: i18n.number(page, 0),
            pages: i18n.number(totalPages, 0),
          })}
        </span>
      {/if}
    </span>
    <Pagination
      label={position === 'top'
        ? i18n.t('{table} pagination (top)', { table: tableLabel })
        : i18n.t('{table} pagination (bottom)', { table: tableLabel })}
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

  <div
    onfocusin={revealFocus}
    data-frozen={frozenColumns}
    class="osl-table overflow-x-auto {frozenColumns === 'athlete'
      ? 'scroll-ps-48 sm:scroll-ps-80'
      : frozenColumns === 'rank'
        ? 'scroll-ps-14 sm:scroll-ps-16'
        : ''}"
  >
    <table class="w-full text-meta leading-4 whitespace-nowrap" aria-busy={busy}>
      <caption class="sr-only">{tableLabel}</caption>
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

  .osl-table[data-frozen='rank'] :global(tr > :first-child),
  .osl-table[data-frozen='athlete'] :global(tr > :nth-child(2)) {
    border-right: 1px solid var(--color-stroke-strong);
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
