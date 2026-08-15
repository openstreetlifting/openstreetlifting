<script lang="ts">
  import type { PageData } from './$types';
  import { Card, Breadcrumb, Pagination, Flag, SearchInput } from '$lib/components/ui';
  import { SortIcon } from '$lib/components/icons';
  import { resolve } from '$app/paths';
  import { page } from '$app/state';
  import { formatDate, countryName } from '$lib/utils';
  import { RANKING_MOVEMENTS, RANKING_GENDERS } from '$lib/constants/ranking';
  import { RankingsTable } from '$lib/state/rankings-table.svelte';

  let { data }: { data: PageData } = $props();
  const competition = $derived(data.competition);

  const table = new RankingsTable({
    basePath: `/competitions/${data.competition.slug}`,
    fixedParams: { competition_id: data.competition.competition_id },
    initialUrl: page.url,
  });

  $effect(() => {
    table.syncFromData(data);
  });

  const movements = RANKING_MOVEMENTS;
  const genders = RANKING_GENDERS;

  // Null means the meet did not contest the movement, which reads as a dash
  // rather than a zero.
  function formatWeight(weight: number | null): string {
    return weight && weight > 0 ? `${weight}` : '-';
  }

  function formatRIS(ris: number | null): string {
    return ris && ris > 0 ? ris.toFixed(2) : '-';
  }
</script>

<svelte:head>
  <title>{competition.name} - OpenStreetlifting</title>
  <meta name="description" content="Results and details for {competition.name}" />
</svelte:head>

<div class="mx-auto max-w-[var(--content-max-width)] px-6 py-12">
  <Breadcrumb
    items={[
      { label: 'Home', href: '/' },
      { label: 'Competitions', href: '/competitions' },
      { label: competition.name },
    ]}
  />

  <!-- Competition Header -->
  <div class="mb-12">
    <h1 class="mb-4 text-5xl font-light text-white">{competition.name}</h1>

    <div class="flex flex-wrap gap-x-6 gap-y-3 text-base text-zinc-400">
      <div class="flex items-center gap-2">
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
          />
        </svg>
        <span>
          {#if competition.start_date}
            {formatDate(competition.start_date)}
            {#if competition.end_date && competition.start_date !== competition.end_date}
              - {formatDate(competition.end_date)}
            {/if}
          {/if}
        </span>
      </div>

      <div class="flex items-center gap-2">
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
          />
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
          />
        </svg>
        <span>{competition.venue}, {competition.city}, {competition.country}</span>
      </div>

      <div class="flex items-center gap-2">
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
          />
        </svg>
        <span>{competition.federation.name} ({competition.federation.abbreviation})</span>
      </div>
    </div>
  </div>

  <div
    class="mb-6 flex flex-wrap items-center gap-3 rounded-lg border border-zinc-800 bg-zinc-900/30 p-3"
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
      class="rounded-lg border border-zinc-800 bg-zinc-900/50 px-3 py-2 text-sm text-zinc-300 transition-colors focus:border-zinc-700 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
    >
      <option value={null}>All Countries</option>
      {#each data.countries as countryOption (countryOption)}
        <option value={countryOption}>{countryName(countryOption)}</option>
      {/each}
    </select>
    <select
      bind:value={table.genderFilter}
      onchange={() => {
        table.categoryFilter = null;
        table.handleFilterChange();
      }}
      class="rounded-lg border border-zinc-800 bg-zinc-900/50 px-3 py-2 text-sm text-zinc-300 transition-colors focus:border-zinc-700 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
    >
      {#each genders as gender (gender.label)}
        <option value={gender.value}>{gender.label}</option>
      {/each}
    </select>
    <select
      bind:value={table.categoryFilter}
      onchange={() => table.handleFilterChange()}
      class="rounded-lg border border-zinc-800 bg-zinc-900/50 px-3 py-2 text-sm text-zinc-300 transition-colors focus:border-zinc-700 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
    >
      <option value={null}>All Classes</option>
      {#each data.classes as classOption (classOption)}
        <option value={classOption}>{classOption}</option>
      {/each}
    </select>
  </div>

  {#if table.rankings.length === 0 && !table.isLoading}
    <Card class="p-8">
      <div class="text-center">
        <p class="text-zinc-400">No results found for the selected filters</p>
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
        <span class="text-sm text-zinc-500">
          Page {table.currentPage} of {table.totalPages} &middot; {table.totalItems} athletes
        </span>
        <Pagination
          page={table.currentPage}
          totalPages={table.totalPages}
          disabled={table.isLoading}
          onNavigate={(target) => table.goToPage(target)}
        />
      </div>
    {/snippet}

    <div class="mb-3">
      {@render paginationBar()}
    </div>

    <Card class="p-4">
      <div class="overflow-x-auto">
        <table class="w-full text-sm">
          <thead class="sticky top-0 z-10">
            <tr class="border-b border-zinc-800 bg-zinc-900 shadow-lg shadow-zinc-950/50">
              <th class="px-3 py-2 text-left font-medium text-zinc-400">Rank</th>
              <th class="px-3 py-2 text-left font-medium text-zinc-400">Athlete</th>
              <th class="px-3 py-2 text-left font-medium text-zinc-400">Sex</th>
              <th class="px-3 py-2 text-left font-medium text-zinc-400">Class</th>
              {#each movements as movement (movement.value)}
                <th
                  class="cursor-pointer px-3 py-2 text-left font-medium transition-colors select-none hover:text-white {table.movementFilter ===
                  movement.value
                    ? 'text-white'
                    : 'text-zinc-400'}"
                  onclick={() => table.sortBy(movement.value)}
                >
                  {movement.label}
                  <SortIcon
                    direction={table.movementFilter === movement.value
                      ? table.sortDirection
                      : 'none'}
                    class="ml-1"
                  />
                </th>
              {/each}
              <th
                class="cursor-pointer px-3 py-2 text-left font-medium text-zinc-400 transition-colors select-none hover:text-white"
                onclick={() => table.sortBy('ris')}
              >
                RIS
              </th>
            </tr>
          </thead>
          <tbody>
            {#each table.rankings as entry (entry.rank + entry.athlete.athlete_id)}
              <tr
                class="border-b border-zinc-800/50 transition-colors even:bg-zinc-900/60 hover:bg-zinc-800/50"
              >
                <td class="px-3 py-2 text-white">
                  {entry.rank}
                </td>
                <td class="px-3 py-2 text-white">
                  <a
                    href={resolve(`/athletes/${entry.athlete.slug}`)}
                    class="inline-flex items-center gap-2.5 hover:text-zinc-300"
                  >
                    <Flag countryCode={entry.athlete.country} class="-ml-1 shrink-0" />
                    <span class="underline">
                      {entry.athlete.first_name}
                      {entry.athlete.last_name}
                    </span>
                  </a>
                </td>
                <td class="px-3 py-2 text-zinc-400">{entry.athlete.gender}</td>
                <td class="px-3 py-2 text-zinc-400">{entry.category}</td>
                <td class="px-3 py-2 text-zinc-400">{formatWeight(entry.muscleup)}</td>
                <td class="px-3 py-2 text-zinc-400">{formatWeight(entry.pullup)}</td>
                <td class="px-3 py-2 text-zinc-400">{formatWeight(entry.dips)}</td>
                <td class="px-3 py-2 text-zinc-400">{formatWeight(entry.squat)}</td>
                <td class="px-3 py-2 font-medium text-zinc-400">{formatWeight(entry.total)}</td>
                <td class="px-3 py-2 font-medium text-zinc-400">{formatRIS(entry.ris)}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </Card>

    <div class="mt-3">
      {@render paginationBar()}
    </div>
  {/if}
</div>
