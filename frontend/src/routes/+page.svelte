<script lang="ts">
  import type { PageData } from './$types';
  import {
    Card,
    Pagination,
    Flag,
    SearchInput,
    Table,
    TABLE_CELL,
    TABLE_HEAD_CELL,
  } from '$lib/components/ui';
  import { InstagramIcon } from '$lib/components/icons';
  import { resolve } from '$app/paths';
  import { page, navigating } from '$app/state';
  import { formatDate, countryName } from '$lib/utils';
  import { RANKING_MOVEMENTS, RANKING_SORTS, RANKING_GENDERS } from '$lib/constants/ranking';
  import { RankingsTable } from '$lib/state/rankings-table.svelte';

  let { data }: { data: PageData } = $props();

  const table = new RankingsTable({ basePath: '/', includeYear: true, initialUrl: page.url });

  const rankings = $derived(data.initialRankings);
  const pagination = $derived(data.pagination);
  const busy = $derived(navigating.to?.url.pathname === page.url.pathname);

  const movements = RANKING_MOVEMENTS;
  const sorts = RANKING_SORTS;
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
  <title>{data.title}</title>
  <meta name="description" content={data.description} />
</svelte:head>

<div class="mx-auto max-w-[var(--content-max-width)] px-4 py-8 sm:px-6 sm:py-12">
  <div class="mb-6">
    <h1 class="mb-4 text-3xl font-light tracking-tight text-white sm:text-4xl">Rankings</h1>
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
      bind:value={table.yearFilter}
      onchange={() => table.handleFilterChange()}
      class="w-full rounded-lg border border-zinc-800 bg-zinc-900/50 px-3 py-2 text-sm text-zinc-300 transition-colors focus:border-zinc-700 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none sm:w-auto"
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
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Competition</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Federation</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Date</th>
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
              <span class="inline-flex items-center gap-1.5">
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
                {#if entry.athlete.instagram_handle}
                  <a
                    href={`https://www.instagram.com/${entry.athlete.instagram_handle}`}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-flex shrink-0 items-center text-white transition-colors hover:text-zinc-300"
                    aria-label="{entry.athlete.first_name} {entry.athlete.last_name} on Instagram"
                    title="@{entry.athlete.instagram_handle}"
                  >
                    <InstagramIcon class="h-4 w-4" />
                  </a>
                {/if}
              </span>
            </td>
            <td class="{TABLE_CELL} text-zinc-400">
              <a
                href={resolve(`/competitions/${entry.competition.slug}`)}
                class="block max-w-[10rem] truncate underline hover:text-zinc-300 sm:max-w-none"
              >
                {entry.competition.name}
              </a>
            </td>
            <td class="{TABLE_CELL} text-zinc-400" title={entry.federation.name}>
              {entry.federation.abbreviation || entry.federation.name}
            </td>
            <td class="{TABLE_CELL} text-zinc-400">{formatDate(entry.competition.date)}</td>
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
