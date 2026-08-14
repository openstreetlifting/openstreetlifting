<script lang="ts">
  import type { PageData } from './$types';
  import { Card, Pagination } from '$lib/components/ui';
  import { SortIcon } from '$lib/components/icons';
  import type { RankingEntry } from '$lib/types/ranking';
  import { resolve } from '$app/paths';
  import { page } from '$app/state';
  import { goto } from '$app/navigation';
  import { SvelteURLSearchParams } from 'svelte/reactivity';
  import { formatDate, getCountryFlag } from '$lib/utils';
  import { RANKING_MOVEMENTS, RANKING_GENDERS } from '$lib/constants/ranking';

  let { data }: { data: PageData } = $props();

  let rankings = $state<RankingEntry[]>([]);
  let currentPage = $state(1);
  let totalPages = $state(1);
  let totalItems = $state(0);
  let isLoading = $state(false);

  $effect(() => {
    rankings = data.initialRankings;
    currentPage = data.pagination.page;
    totalPages = data.pagination.total_pages;
    totalItems = data.pagination.total_items;
  });

  let genderFilter = $state<string | null>(page.url.searchParams.get('gender') || null);
  let categoryFilter = $state<string | null>(page.url.searchParams.get('category') || null);
  let yearFilter = $state<number | null>(Number(page.url.searchParams.get('year')) || null);
  let movementFilter = $state<string>(page.url.searchParams.get('movement') || 'ris');
  let sortDirection = $state<'asc' | 'desc'>(
    page.url.searchParams.get('direction') === 'asc' ? 'asc' : 'desc'
  );

  const movements = RANKING_MOVEMENTS;
  const genders = RANKING_GENDERS;

  function sortBy(value: string) {
    if (movementFilter === value) {
      sortDirection = sortDirection === 'desc' ? 'asc' : 'desc';
    } else {
      movementFilter = value;
      sortDirection = 'desc';
    }
    handleFilterChange();
  }

  function updateURL(targetPage: number) {
    const params = new SvelteURLSearchParams();

    if (movementFilter !== 'ris') {
      params.set('movement', movementFilter);
    }

    if (sortDirection !== 'desc') {
      params.set('direction', sortDirection);
    }

    if (genderFilter) {
      params.set('gender', genderFilter);
    }

    if (categoryFilter) {
      params.set('category', categoryFilter);
    }

    if (yearFilter) {
      params.set('year', String(yearFilter));
    }

    if (targetPage > 1) {
      params.set('page', String(targetPage));
    }

    const queryString = params.toString();
    const path = queryString ? `/?${queryString}` : '/';
    goto(resolve(path), { replaceState: true, keepFocus: true, noScroll: true });
  }

  async function loadRankings(targetPage: number) {
    if (isLoading) return;
    isLoading = true;
    try {
      const params = new SvelteURLSearchParams();
      params.set('page', String(targetPage));
      params.set('movement', movementFilter);
      params.set('direction', sortDirection);
      if (genderFilter) params.set('gender', genderFilter);
      if (categoryFilter) params.set('category', categoryFilter);
      if (yearFilter) params.set('year', String(yearFilter));

      const response = await fetch(`/api/rankings?${params}`);
      const result = await response.json();

      rankings = result.data;
      currentPage = result.pagination.page;
      totalPages = result.pagination.total_pages;
      totalItems = result.pagination.total_items;
    } catch (error) {
      console.error('Error loading rankings:', error);
    } finally {
      isLoading = false;
    }
  }

  async function handleFilterChange() {
    updateURL(1);
    await loadRankings(1);
  }

  async function goToPage(targetPage: number) {
    if (targetPage < 1 || targetPage > totalPages || targetPage === currentPage) return;
    updateURL(targetPage);
    await loadRankings(targetPage);
  }

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

<div class="mx-auto max-w-[var(--content-max-width)] px-6 py-12">
  <div class="mb-6">
    <h1 class="mb-4 text-2xl font-medium text-white">Rankings</h1>
  </div>

  <div
    class="mb-6 flex flex-wrap items-center gap-3 rounded-lg border border-zinc-800 bg-zinc-900/30 p-3"
  >
    <select
      bind:value={genderFilter}
      onchange={() => {
        categoryFilter = null;
        handleFilterChange();
      }}
      class="rounded-lg border border-zinc-800 bg-zinc-900/50 px-3 py-2 text-sm text-zinc-300 transition-colors focus:border-zinc-700 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
    >
      {#each genders as gender (gender.label)}
        <option value={gender.value}>{gender.label}</option>
      {/each}
    </select>

    <select
      bind:value={categoryFilter}
      onchange={handleFilterChange}
      class="rounded-lg border border-zinc-800 bg-zinc-900/50 px-3 py-2 text-sm text-zinc-300 transition-colors focus:border-zinc-700 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
    >
      <option value={null}>All Classes</option>
      {#each data.classes as classOption (classOption)}
        <option value={classOption}>{classOption}</option>
      {/each}
    </select>

    <select
      bind:value={yearFilter}
      onchange={handleFilterChange}
      class="rounded-lg border border-zinc-800 bg-zinc-900/50 px-3 py-2 text-sm text-zinc-300 transition-colors focus:border-zinc-700 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
    >
      <option value={null}>All Years</option>
      {#each data.years as yearOption (yearOption)}
        <option value={yearOption}>{yearOption}</option>
      {/each}
    </select>
  </div>

  {#if data.error}
    <Card class="p-8">
      <div class="text-center">
        <p class="text-red-400">{data.error}</p>
      </div>
    </Card>
  {:else if rankings.length === 0 && !isLoading}
    <Card class="p-8">
      <div class="text-center">
        <p class="text-zinc-400">No rankings found for the selected filters</p>
        <button
          onclick={() => {
            genderFilter = null;
            categoryFilter = null;
            yearFilter = null;
            movementFilter = 'ris';
            sortDirection = 'desc';
            handleFilterChange();
          }}
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
          Page {currentPage} of {totalPages} &middot; {totalItems} athletes
        </span>
        <Pagination page={currentPage} {totalPages} disabled={isLoading} onNavigate={goToPage} />
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
              <th class="px-3 py-2 text-left font-medium text-zinc-400">Country</th>
              <th class="px-3 py-2 text-left font-medium text-zinc-400">Competition</th>
              <th class="px-3 py-2 text-left font-medium text-zinc-400">Federation</th>
              <th class="px-3 py-2 text-left font-medium text-zinc-400">Date</th>
              <th class="px-3 py-2 text-left font-medium text-zinc-400">Sex</th>
              <th class="px-3 py-2 text-left font-medium text-zinc-400">Class</th>
              {#each movements as movement (movement.value)}
                <th
                  class="cursor-pointer px-3 py-2 text-left font-medium transition-colors select-none hover:text-white {movementFilter ===
                  movement.value
                    ? 'text-white'
                    : 'text-zinc-400'}"
                  onclick={() => sortBy(movement.value)}
                >
                  {movement.label}
                  <SortIcon
                    direction={movementFilter === movement.value ? sortDirection : 'none'}
                    class="ml-1"
                  />
                </th>
              {/each}
              <th
                class="cursor-pointer px-3 py-2 text-left font-medium text-zinc-400 transition-colors select-none hover:text-white"
                onclick={() => sortBy('ris')}
              >
                RIS
              </th>
            </tr>
          </thead>
          <tbody>
            {#each rankings as entry (entry.rank + entry.athlete.athlete_id)}
              <tr
                class="border-b border-zinc-800/50 transition-colors even:bg-zinc-900/60 hover:bg-zinc-800/50"
              >
                <td class="px-3 py-2 text-white">
                  {entry.rank}
                </td>
                <td class="px-3 py-2 text-white">
                  <a
                    href={resolve(`/athletes/${entry.athlete.slug}`)}
                    class="underline hover:text-zinc-300"
                  >
                    {entry.athlete.first_name}
                    {entry.athlete.last_name}
                  </a>
                </td>
                <td class="px-3 py-2">
                  <span class="text-lg" title={entry.athlete.country}>
                    {getCountryFlag(entry.athlete.country)}
                  </span>
                </td>
                <td class="px-3 py-2 text-zinc-400">
                  <a
                    href={resolve(`/competitions/${entry.competition.slug}`)}
                    class="underline hover:text-zinc-300"
                  >
                    {entry.competition.name}
                  </a>
                </td>
                <td class="px-3 py-2 text-zinc-400" title={entry.federation.name}>
                  {entry.federation.abbreviation || entry.federation.name}
                </td>
                <td class="px-3 py-2 text-zinc-400">{formatDate(entry.competition.date)}</td>
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
