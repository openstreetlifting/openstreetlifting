<script lang="ts">
  import type { PageData } from './$types';
  import {
    Card,
    Pagination,
    Flag,
    SearchInput,
    Table,
    RisHeader,
    RisScore,
    TABLE_CELL,
    TABLE_HEAD_CELL,
    FROZEN_CELL,
    FROZEN_HEAD_CELL,
    FROZEN_EDGE,
    FROZEN_RANK,
    FROZEN_ATHLETE,
    FROZEN_ATHLETE_CONTENT,
  } from '$lib/components/ui';
  import { InstagramIcon } from '$lib/components/icons';
  import { resolve } from '$app/paths';
  import { page, navigating } from '$app/state';
  import { formatDate, countryName, formatWeight, formatAthleteName } from '$lib/utils';
  import { CELL, SORTED_COLUMN, EDGE_TO_EDGE } from '$lib/constants/table';
  import { RANKING_MOVEMENTS, RANKING_SORTS, RANKING_GENDERS } from '$lib/constants/ranking';
  import { RankingsTable } from '$lib/state/rankings-table.svelte';
  import { FIELD, TEXT } from '$lib/constants/typography';

  let { data }: { data: PageData } = $props();

  const table = new RankingsTable({ basePath: '/', includeYear: true, initialUrl: page.url });

  const rankings = $derived(data.initialRankings);
  const pagination = $derived(data.pagination);
  const busy = $derived(navigating.to?.url.pathname === page.url.pathname);

  const movements = RANKING_MOVEMENTS;
  const sorts = RANKING_SORTS;
  const genders = RANKING_GENDERS;
</script>

<svelte:head>
  <title>{data.title}</title>
  <meta name="description" content={data.description} />
</svelte:head>

<div class="mx-auto max-w-[var(--content-max-width)] px-4 py-8 sm:px-6 sm:py-12">
  <div class="mb-6">
    <h1 class="mb-4 {TEXT.title} text-white">Rankings</h1>
  </div>

  <div
    class="{EDGE_TO_EDGE} mb-6 flex flex-wrap items-center gap-3 rounded-none border border-x-0 border-zinc-800 bg-zinc-900/30 p-3 sm:rounded-lg sm:border-x"
  >
    <div class="w-full sm:w-64">
      <SearchInput
        bind:value={table.searchFilter}
        placeholder="Search an athlete"
        onSearch={() => table.handleFilterChange()}
      />
    </div>

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

    <div class="flex w-full items-center gap-2 sm:ml-auto sm:w-auto">
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
  </div>

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

    <div class="mb-3">
      {@render paginationBar()}
    </div>

    <Table {busy}>
      {#snippet head()}
        <th class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_RANK} text-zinc-400">Rank</th>
        <th
          class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_ATHLETE} {FROZEN_EDGE} text-zinc-400"
          >Athlete</th
        >
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Competition</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Federation</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Date</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Sex</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Class</th>
        {#each movements as movement (movement.value)}
          <th
            class="{TABLE_HEAD_CELL} text-zinc-400 {table.movementFilter === movement.value
              ? SORTED_COLUMN
              : ''}"
          >
            {movement.label}
          </th>
        {/each}
        <th
          class="{TABLE_HEAD_CELL} text-zinc-400 {table.movementFilter === 'ris'
            ? SORTED_COLUMN
            : ''}"
        >
          <RisHeader />
        </th>
      {/snippet}

      {#snippet body()}
        {#each rankings as entry (entry.rank + entry.athlete.athlete_id)}
          <tr class="border-b border-zinc-800/50 transition-colors">
            <td class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_RANK} {CELL.identity}">
              {entry.rank}
            </td>
            <td class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_ATHLETE} {FROZEN_EDGE} {CELL.identity}">
              <span class="flex items-center gap-1.5 {FROZEN_ATHLETE_CONTENT}">
                <a
                  href={resolve(`/athletes/${entry.athlete.slug}`)}
                  class="flex min-w-0 items-center gap-2.5 hover:text-zinc-300"
                >
                  <Flag
                    countryCode={entry.athlete.country}
                    class="-ml-1 shrink-0 [--flag-height:1.25em]"
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
            <td class="{TABLE_CELL} {CELL.data}">
              <a
                href={resolve(`/competitions/${entry.competition.slug}`)}
                class="block max-w-[9rem] truncate underline hover:text-zinc-300 sm:max-w-[16rem]"
              >
                {entry.competition.name}
              </a>
            </td>
            <td class="{TABLE_CELL} {CELL.data}" title={entry.federation.name}>
              {entry.federation.abbreviation || entry.federation.name}
            </td>
            <td class="{TABLE_CELL} {CELL.data}">{formatDate(entry.competition.date)}</td>
            <td class="{TABLE_CELL} {CELL.data}">{entry.athlete.gender}</td>
            <td class="{TABLE_CELL} {CELL.data}">{entry.category}</td>
            <td class="{TABLE_CELL} {CELL.data}">{formatWeight(entry.muscleup)}</td>
            <td class="{TABLE_CELL} {CELL.data}">{formatWeight(entry.pullup)}</td>
            <td class="{TABLE_CELL} {CELL.data}">{formatWeight(entry.dips)}</td>
            <td class="{TABLE_CELL} {CELL.data}">{formatWeight(entry.squat)}</td>
            <td class="{TABLE_CELL} {CELL.counted}">{formatWeight(entry.total)}</td>
            <td class="{TABLE_CELL} {CELL.counted}">
              <RisScore value={entry.ris} source={entry.ris_source} />
            </td>
          </tr>
        {/each}
      {/snippet}
    </Table>

    <div class="mt-3">
      {@render paginationBar()}
    </div>
  {/if}
</div>
