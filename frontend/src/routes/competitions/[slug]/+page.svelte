<script lang="ts">
  import type { PageData } from './$types';
  import {
    Card,
    Breadcrumb,
    Pagination,
    Flag,
    SearchInput,
    Table,
    TABLE_CELL,
    TABLE_HEAD_CELL,
  } from '$lib/components/ui';
  import { resolve } from '$app/paths';
  import { page, navigating } from '$app/state';
  import { formatDate, formatLocation, countryName } from '$lib/utils';
  import { GitHubIcon } from '$lib/components/icons';
  import { RANKING_MOVEMENTS, RANKING_SORTS, RANKING_GENDERS } from '$lib/constants/ranking';
  import { RankingsTable } from '$lib/state/rankings-table.svelte';

  let { data }: { data: PageData } = $props();
  const competition = $derived(data.competition);

  // Canonical files live at imports/{slug}/{slug}.json, which the importer enforces.
  const editUrl = $derived(
    `https://github.com/openstreetlifting/openstreetlifting/edit/main/backend/imports/${competition.slug}/${competition.slug}.json`
  );

  // Most federations are known by an acronym alone, so there is nothing to put
  // in brackets after it.
  const federationLabel = $derived(
    competition.federation.abbreviation &&
      competition.federation.abbreviation !== competition.federation.name
      ? `${competition.federation.name} (${competition.federation.abbreviation})`
      : competition.federation.name
  );

  const table = new RankingsTable({
    basePath: `/competitions/${data.competition.slug}`,
    initialUrl: page.url,
  });

  const rankings = $derived(data.initialRankings);
  const pagination = $derived(data.pagination);
  const busy = $derived(navigating.to?.url.pathname === page.url.pathname);

  const movements = RANKING_MOVEMENTS;
  const sorts = RANKING_SORTS;
  const genders = RANKING_GENDERS;

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

<div class="mx-auto max-w-[var(--content-max-width)] px-4 py-8 sm:px-6 sm:py-12">
  <Breadcrumb
    items={[
      { label: 'Home', href: '/' },
      { label: 'Competitions', href: '/competitions' },
      { label: competition.name },
    ]}
  />

  <!-- Competition Header -->
  <div class="mb-12">
    <h1 class="mb-4 text-3xl font-light tracking-tight text-white sm:text-4xl">
      {competition.name}
    </h1>

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
        <span>{formatLocation(competition.country, competition.region, competition.city)}</span>
      </div>

      <div class="flex items-center gap-2">
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
          <circle cx="12" cy="9" r="6" />
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M15.5 13.9 17 22l-5-3-5 3 1.5-8.1"
          />
        </svg>
        <span>{federationLabel}</span>
      </div>
    </div>

    <!-- eslint-disable svelte/no-navigation-without-resolve -- absolute GitHub URL, not an app route -->
    <a
      href={editUrl}
      target="_blank"
      rel="noopener noreferrer"
      class="mt-6 inline-flex items-center gap-2 rounded-lg border border-zinc-800 bg-zinc-900/50 px-3 py-2 text-sm text-zinc-300 transition-colors hover:border-zinc-700 hover:text-white focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
    >
      <GitHubIcon class="h-4 w-4" />
      Edit on GitHub
    </a>
    <!-- eslint-enable svelte/no-navigation-without-resolve -->
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
      class="w-full rounded-lg border border-zinc-800 bg-zinc-900/50 px-3 py-2 text-sm text-zinc-300 transition-colors focus:border-zinc-700 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none sm:w-auto"
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
      class="w-full rounded-lg border border-zinc-800 bg-zinc-900/50 px-3 py-2 text-sm text-zinc-300 transition-colors focus:border-zinc-700 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none sm:w-auto"
    >
      {#each genders as gender (gender.label)}
        <option value={gender.value}>{gender.label}</option>
      {/each}
    </select>
    <select
      bind:value={table.categoryFilter}
      onchange={() => table.handleFilterChange()}
      class="w-full rounded-lg border border-zinc-800 bg-zinc-900/50 px-3 py-2 text-sm text-zinc-300 transition-colors focus:border-zinc-700 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none sm:w-auto"
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
        class="flex-1 rounded-lg border border-zinc-800 bg-zinc-900/50 px-3 py-2 text-sm text-zinc-300 transition-colors focus:border-zinc-700 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none sm:flex-none"
      >
        {#each sorts as sort (sort.value)}
          <option value={sort.value}>{sort.label}</option>
        {/each}
      </select>
    </div>
  </div>

  {#if rankings.length === 0 && !busy}
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
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Rank</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Athlete</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Sex</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Class</th>
        {#each movements as movement (movement.value)}
          <th
            class="{TABLE_HEAD_CELL} {table.movementFilter === movement.value
              ? 'text-white'
              : 'text-zinc-400'}"
          >
            {movement.label}
          </th>
        {/each}
        <th
          class="{TABLE_HEAD_CELL} {table.movementFilter === 'ris'
            ? 'text-white'
            : 'text-zinc-400'}"
        >
          RIS
        </th>
      {/snippet}

      {#snippet body()}
        {#each rankings as entry (entry.rank + entry.athlete.athlete_id)}
          <tr
            class="border-b border-zinc-800/50 transition-colors even:bg-zinc-900/60 hover:bg-zinc-800/50"
          >
            <td class="{TABLE_CELL} text-white">
              {entry.rank}
            </td>
            <td class="{TABLE_CELL} text-white">
              <a
                href={resolve(`/athletes/${entry.athlete.slug}`)}
                class="inline-flex max-w-[8rem] items-center gap-2.5 hover:text-zinc-300 sm:max-w-none"
              >
                <Flag countryCode={entry.athlete.country} class="-ml-1 shrink-0" />
                <span class="truncate underline">
                  {entry.athlete.first_name}
                  {entry.athlete.last_name}
                </span>
              </a>
            </td>
            <td class="{TABLE_CELL} text-zinc-400">{entry.athlete.gender}</td>
            <td class="{TABLE_CELL} text-zinc-400">{entry.category}</td>
            <td class="{TABLE_CELL} text-zinc-400">{formatWeight(entry.muscleup)}</td>
            <td class="{TABLE_CELL} text-zinc-400">{formatWeight(entry.pullup)}</td>
            <td class="{TABLE_CELL} text-zinc-400">{formatWeight(entry.dips)}</td>
            <td class="{TABLE_CELL} text-zinc-400">{formatWeight(entry.squat)}</td>
            <td class="{TABLE_CELL} font-medium text-zinc-400">{formatWeight(entry.total)}</td>
            <td class="{TABLE_CELL} font-medium text-zinc-400">{formatRIS(entry.ris)}</td>
          </tr>
        {/each}
      {/snippet}
    </Table>

    <div class="mt-3">
      {@render paginationBar()}
    </div>
  {/if}
</div>
