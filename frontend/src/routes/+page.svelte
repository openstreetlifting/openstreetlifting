<script lang="ts">
  import type { PageData } from './$types';
  import {
    Card,
    Pagination,
    Flag,
    FilterBar,
    Table,
    RisHeader,
    RisScore,
    TABLE_CELL,
    TABLE_HEAD_CELL,
    FROZEN_CELL,
    FROZEN_HEAD_CELL,
    FROZEN_EDGE,
    FROZEN_RANK,
    ATHLETE_COLUMN,
    ATHLETE_CONTENT,
  } from '$lib/components/ui';
  import { InstagramIcon } from '$lib/components/icons';
  import Seo from '$lib/components/seo.svelte';
  import { resolve } from '$app/paths';
  import { page, navigating } from '$app/state';
  import { afterNavigate } from '$app/navigation';
  import { formatDate, countryName, formatWeight, formatAthleteName } from '$lib/utils';
  import { CELL, FIGURE, SORTED_COLUMN, TEXT_CELL } from '$lib/constants/table';
  import { RANKING_LIFTS, RANKING_SORTS, RANKING_GENDERS } from '$lib/constants/ranking';
  import { RankingsTable } from '$lib/state/rankings-table.svelte';
  import { rememberRankings } from '$lib/state/rankings-return.svelte';
  import { slowNavigation } from '$lib/state/slow-navigation.svelte';
  import { FIELD } from '$lib/constants/typography';
  import { listingSeo, websiteLd } from '$lib/seo';

  let { data }: { data: PageData } = $props();

  const table = new RankingsTable({ basePath: '/', includeYear: true, initialUrl: page.url });

  const rankings = $derived(data.initialRankings);
  const pagination = $derived(data.pagination);
  const loading = slowNavigation(() => navigating.to?.url.pathname === page.url.pathname);
  const busy = $derived(loading.current);

  const lifts = RANKING_LIFTS;
  const sorts = RANKING_SORTS;
  const genders = RANKING_GENDERS;

  const seo = $derived(listingSeo(page.url));

  const focused = $derived(page.url.searchParams.get('athlete'));

  // The controls follow the URL in the same pass that brings the new rows in.
  // Catching them up afterwards instead repaints the board a second time, a
  // frame behind the rows the visitor is already reading.
  $effect(() => {
    table.syncFromUrl(page.url);
    rememberRankings(page.url.search);
  });

  afterNavigate(() => {
    if (!focused) return;
    requestAnimationFrame(() => {
      document.querySelector('[data-focused]')?.scrollIntoView({ block: 'center' });
    });
  });

  const sorted = (column: string) => (table.movementFilter === column ? SORTED_COLUMN : '');

  const activeFilters = $derived(
    [
      table.countryFilter,
      table.federationFilter,
      table.yearFilter,
      table.genderFilter,
      table.categoryFilter,
    ].filter(Boolean).length
  );
</script>

<Seo
  title={data.title}
  description={data.description}
  canonical={seo.canonical}
  noindex={seo.noindex}
  jsonLd={[websiteLd()]}
/>

<div class="mx-auto max-w-[var(--content-max-width)] px-4 py-3 sm:px-6 sm:py-12">
  <h1 class="sr-only">Streetlifting rankings</h1>

  <FilterBar
    bind:search={table.searchFilter}
    placeholder="Search an athlete"
    onSearch={() => table.handleFilterChange()}
    activeCount={activeFilters}
    onClear={() => table.clearFilters()}
    clearable={table.narrowed}
  >
    <select
      bind:value={table.countryFilter}
      onchange={() => table.handleFilterChange()}
      class="w-full {FIELD} px-3 py-2 sm:w-auto"
    >
      <option value={null}>All Countries</option>
      {#each data.countries as countryOption (countryOption)}
        <option value={countryOption}>{countryName(countryOption)}</option>
      {/each}
    </select>
    <select
      bind:value={table.federationFilter}
      onchange={() => table.handleFilterChange()}
      class="w-full {FIELD} px-3 py-2 sm:w-auto"
    >
      <option value={null}>All Federations</option>
      {#each data.federations as federationOption (federationOption)}
        <option value={federationOption}>{federationOption}</option>
      {/each}
    </select>
    <select
      bind:value={table.yearFilter}
      onchange={() => table.handleFilterChange()}
      class="w-full {FIELD} px-3 py-2 sm:w-auto"
    >
      <option value={null}>All Years</option>
      {#each data.years as yearOption (yearOption)}
        <option value={yearOption}>{yearOption}</option>
      {/each}
    </select>
    <select
      bind:value={table.genderFilter}
      onchange={() => {
        table.categoryFilter = null;
        table.handleFilterChange();
      }}
      class="w-full {FIELD} px-3 py-2 sm:w-auto"
    >
      {#each genders as gender (gender.label)}
        <option value={gender.value}>{gender.label}</option>
      {/each}
    </select>
    <select
      bind:value={table.categoryFilter}
      onchange={() => table.handleFilterChange()}
      class="w-full {FIELD} px-3 py-2 sm:w-auto"
    >
      <option value={null}>All Classes</option>
      {#each data.classes as classOption (classOption)}
        <option value={classOption}>{classOption}</option>
      {/each}
    </select>

    <div class="flex w-full items-center gap-2 sm:w-auto">
      <label for="sort-by" class="text-sm text-zinc-500">Sort by</label>
      <select
        id="sort-by"
        bind:value={table.movementFilter}
        onchange={() => table.handleFilterChange()}
        class="flex-1 {FIELD} px-3 py-2 sm:flex-none"
      >
        {#each sorts as sort (sort.value)}
          <option value={sort.value}>{sort.label}</option>
        {/each}
      </select>
    </div>
  </FilterBar>

  {#if data.error}
    <Card class="p-8">
      <div class="text-center">
        <p class="text-red-400">{data.error}</p>
      </div>
    </Card>
  {:else if rankings.length === 0 && !busy}
    <Card class="p-8">
      <div class="text-center">
        <p class="text-zinc-400">No rankings found for the selected filters</p>
        <button
          onclick={() => table.clearFilters()}
          class="mt-4 text-sm text-zinc-500 underline hover:text-zinc-300 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
        >
          Clear filters
        </button>
      </div>
    </Card>
  {:else}
    {#snippet paginationBar()}
      <div class="flex flex-wrap items-center justify-between gap-3">
        <span class="text-xs text-zinc-500">
          Page {pagination.page} of {pagination.total_pages} &middot; {pagination.total_items} athletes
        </span>
        <Pagination
          page={pagination.page}
          totalPages={pagination.total_pages}
          disabled={busy}
          onNavigate={(target) => table.goToPage(target)}
        />
      </div>
    {/snippet}

    <div class="hidden sm:mb-3 sm:block">
      {@render paginationBar()}
    </div>

    <Table {busy}>
      {#snippet head()}
        <th class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_RANK} {FROZEN_EDGE} text-zinc-400"
          >Rank</th
        >
        <th class="{TABLE_HEAD_CELL} {ATHLETE_COLUMN} text-zinc-400">Athlete</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400 {sorted('total')}">Total</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400 {sorted('ris')}">
          <RisHeader />
        </th>
        {#each lifts as lift (lift.value)}
          <th class="{TABLE_HEAD_CELL} text-zinc-400 {sorted(lift.value)}">
            {lift.label}
          </th>
        {/each}
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Competition</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Federation</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Date</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Sex</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Class</th>
      {/snippet}

      {#snippet body()}
        {#each rankings as entry (entry.rank + entry.athlete.athlete_id)}
          <tr
            class="border-b border-zinc-800/50 transition-colors"
            data-focused={entry.athlete.slug === focused ? '' : undefined}
          >
            <td class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_RANK} {FROZEN_EDGE} {CELL.identity}">
              {entry.rank}
            </td>
            <td class="{TABLE_CELL} {ATHLETE_COLUMN} {CELL.identity}">
              <span class="flex items-center gap-1.5 {ATHLETE_CONTENT}">
                <a
                  href={resolve(`/athletes/${entry.athlete.slug}`)}
                  class="flex min-w-0 items-center gap-2.5 hover:text-zinc-300"
                >
                  <Flag
                    countryCode={entry.athlete.country}
                    class="shrink-0 [--flag-height:1.25em]"
                  />
                  <span class="truncate underline">
                    {formatAthleteName(entry.athlete)}
                  </span>
                </a>
                {#if entry.athlete.instagram_handle}
                  <a
                    href={`https://www.instagram.com/${entry.athlete.instagram_handle}`}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-flex shrink-0 items-center text-white transition-colors hover:text-zinc-300"
                    aria-label="{formatAthleteName(entry.athlete)} on Instagram"
                    title="@{entry.athlete.instagram_handle}"
                  >
                    <InstagramIcon class="h-4 w-4" />
                  </a>
                {/if}
              </span>
            </td>
            <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">{formatWeight(entry.total)}</td>
            <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">
              <RisScore value={entry.ris} source={entry.ris_source} />
            </td>
            <td class="{TABLE_CELL} {CELL.data} {FIGURE}">{formatWeight(entry.muscleup)}</td>
            <td class="{TABLE_CELL} {CELL.data} {FIGURE}">{formatWeight(entry.pullup)}</td>
            <td class="{TABLE_CELL} {CELL.data} {FIGURE}">{formatWeight(entry.dips)}</td>
            <td class="{TABLE_CELL} {CELL.data} {FIGURE}">{formatWeight(entry.squat)}</td>
            <td class="{TABLE_CELL} {CELL.data}">
              <a
                href={resolve(`/competitions/${entry.competition.slug}`)}
                class="{TEXT_CELL.competition} underline hover:text-zinc-300"
              >
                {entry.competition.name}
              </a>
            </td>
            <td class="{TABLE_CELL} {CELL.data}" title={entry.federation.name}>
              <span class={TEXT_CELL.federation}>
                {entry.federation.abbreviation || entry.federation.name}
              </span>
            </td>
            <td class="{TABLE_CELL} {CELL.data} whitespace-nowrap">
              {formatDate(entry.competition.date)}
            </td>
            <td class="{TABLE_CELL} {CELL.data}">{entry.athlete.gender}</td>
            <td class="{TABLE_CELL} {CELL.data}">{entry.category}</td>
          </tr>
        {/each}
      {/snippet}
    </Table>

    <div class="mt-3">
      {@render paginationBar()}
    </div>
  {/if}
</div>
