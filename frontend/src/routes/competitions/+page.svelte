<script lang="ts">
  import type { PageData } from './$types';
  import { Card, Breadcrumb, Table } from '$lib/components/ui';
  import { resolve } from '$app/paths';
  import { formatDate, formatLocation } from '$lib/utils';

  let { data }: { data: PageData } = $props();

  let statusFilter = $state<string>('all');

  let filteredCompetitions = $derived(() => {
    if (statusFilter === 'all') return data.competitions;
    return data.competitions.filter((comp) => comp.status === statusFilter);
  });
</script>

<svelte:head>
  <title>Competitions - OpenStreetlifting</title>
  <meta name="description" content="List of availables competitions" />
</svelte:head>

<div class="mx-auto max-w-[var(--content-max-width)] px-6 py-12">
  <Breadcrumb items={[{ label: 'Home', href: '/' }, { label: 'Competitions' }]} />

  <div class="mb-8">
    <h1 class="mb-4 text-4xl font-light tracking-tight text-white">Competitions</h1>
  </div>

  <div class="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-end">
    <div class="flex gap-2">
      <button
        onclick={() => (statusFilter = 'all')}
        class="rounded-lg px-4 py-2 text-sm font-medium transition-colors focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none
					{statusFilter === 'all'
          ? 'bg-white text-zinc-950'
          : 'bg-zinc-800/50 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-300'}"
      >
        All
      </button>
      <button
        onclick={() => (statusFilter = 'upcoming')}
        class="rounded-lg px-4 py-2 text-sm font-medium transition-colors focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none
					{statusFilter === 'upcoming'
          ? 'bg-blue-500 text-white'
          : 'bg-zinc-800/50 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-300'}"
      >
        Planned
      </button>
      <button
        onclick={() => (statusFilter = 'ongoing')}
        class="rounded-lg px-4 py-2 text-sm font-medium transition-colors focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none
					{statusFilter === 'ongoing'
          ? 'bg-purple-500 text-white'
          : 'bg-zinc-800/50 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-300'}"
      >
        Ongoing
      </button>
      <button
        onclick={() => (statusFilter = 'completed')}
        class="rounded-lg px-4 py-2 text-sm font-medium transition-colors focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none
					{statusFilter === 'completed'
          ? 'bg-emerald-500 text-white'
          : 'bg-zinc-800/50 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-300'}"
      >
        Completed
      </button>
    </div>
  </div>

  {#if data.error}
    <Card class="p-8">
      <div class="text-center">
        <p class="text-red-400">{data.error}</p>
      </div>
    </Card>
  {:else if filteredCompetitions().length === 0}
    <Card class="p-8">
      <div class="text-center">
        <p class="text-zinc-400">
          {statusFilter !== 'all' ? 'No competitions match your filters' : 'No competitions found'}
        </p>
        {#if statusFilter !== 'all'}
          <button
            onclick={() => (statusFilter = 'all')}
            class="mt-4 text-sm text-zinc-500 underline hover:text-zinc-300 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
          >
            Clear filters
          </button>
        {/if}
      </div>
    </Card>
  {:else}
    <Table>
      {#snippet head()}
        <th class="px-3 py-2 text-left font-medium text-zinc-400">Competition</th>
        <th class="px-3 py-2 text-left font-medium text-zinc-400">Date</th>
        <th class="px-3 py-2 text-left font-medium text-zinc-400">Location</th>
        <th class="px-3 py-2 text-left font-medium text-zinc-400">Federation</th>
        <th class="px-3 py-2 text-left font-medium text-zinc-400">Status</th>
      {/snippet}

      {#snippet body()}
        {#each filteredCompetitions() as competition (competition.slug)}
          <tr
            class="border-b border-zinc-800/50 transition-colors even:bg-zinc-900/60 hover:bg-zinc-800/50"
          >
            <td class="px-3 py-2 text-white">
              <a
                href={resolve(`/competitions/${competition.slug}`)}
                class="underline hover:text-zinc-300"
              >
                {competition.name}
              </a>
            </td>
            <td class="px-3 py-2 whitespace-nowrap text-zinc-400">
              {formatDate(competition.start_date)}
              {#if competition.end_date && competition.end_date !== competition.start_date}
                - {formatDate(competition.end_date)}
              {/if}
            </td>
            <td class="px-3 py-2 text-zinc-400">
              {formatLocation(competition.country, competition.region, competition.city)}
            </td>
            <td class="px-3 py-2 text-zinc-400" title={competition.federation.name}>
              {competition.federation.abbreviation || competition.federation.name}
            </td>
            <td class="px-3 py-2">
              {#if competition.status === 'upcoming'}
                <span
                  class="inline-flex items-center gap-1.5 rounded-md border border-blue-500/20 bg-blue-500/10 px-2.5 py-1 text-xs font-medium text-blue-400"
                >
                  <svg class="h-3 w-3" fill="currentColor" viewBox="0 0 8 8">
                    <circle cx="4" cy="4" r="3" />
                  </svg>
                  Planned
                </span>
              {:else if competition.status === 'ongoing'}
                <span
                  class="inline-flex items-center gap-1.5 rounded-md border border-purple-500/20 bg-purple-500/10 px-2.5 py-1 text-xs font-medium text-purple-400"
                >
                  <svg class="h-3 w-3" fill="currentColor" viewBox="0 0 8 8">
                    <circle cx="4" cy="4" r="3" />
                  </svg>
                  Ongoing
                </span>
              {:else if competition.status === 'completed'}
                <span
                  class="inline-flex items-center gap-1.5 rounded-md border border-emerald-500/20 bg-emerald-500/10 px-2.5 py-1 text-xs font-medium text-emerald-400"
                >
                  <svg class="h-3 w-3" fill="currentColor" viewBox="0 0 8 8">
                    <circle cx="4" cy="4" r="3" />
                  </svg>
                  Completed
                </span>
              {/if}
            </td>
          </tr>
        {/each}
      {/snippet}
    </Table>

    {#if filteredCompetitions().length > 0}
      <div class="mt-8 text-center text-sm text-zinc-500">
        Showing {filteredCompetitions().length}
        {#if statusFilter !== 'all'}
          of {data.competitions.length}
        {/if}
        competitions
      </div>
    {/if}
  {/if}
</div>
