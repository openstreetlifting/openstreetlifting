<script lang="ts">
  import type { PageData } from './$types';
  import { Card, Breadcrumb, Table, TABLE_CELL, TABLE_HEAD_CELL } from '$lib/components/ui';
  import { resolve } from '$app/paths';
  import { formatDate, formatLocation } from '$lib/utils';

  let { data }: { data: PageData } = $props();

  let statusFilter = $state<string>('all');

  let filteredCompetitions = $derived(() => {
    if (statusFilter === 'all') return data.competitions;
    return data.competitions.filter((comp) => comp.status === statusFilter);
  });

  function competitionDates(start: string | null, end: string | null): string {
    const from = formatDate(start);
    return end && end !== start ? `${from} - ${formatDate(end)}` : from;
  }
</script>

<svelte:head>
  <title>Competitions - OpenStreetlifting</title>
  <meta name="description" content="List of availables competitions" />
</svelte:head>

<div class="mx-auto max-w-[var(--content-max-width)] px-4 py-8 sm:px-6 sm:py-12">
  <Breadcrumb items={[{ label: 'Home', href: '/' }, { label: 'Competitions' }]} />

  <div class="mb-8">
    <h1 class="mb-4 text-3xl font-light tracking-tight text-white sm:text-4xl">Competitions</h1>
  </div>

  <div class="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-end">
    <div class="flex flex-wrap gap-2">
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
    {#snippet statusText(status: string)}
      {#if status === 'upcoming'}
        <span class="text-blue-400">Planned</span>
      {:else if status === 'ongoing'}
        <span class="text-purple-400">Ongoing</span>
      {:else if status === 'completed'}
        <span class="text-emerald-400">Completed</span>
      {/if}
    {/snippet}

    <!-- A 5 column table does not fit a phone, so the same rows read as cards there,
         matching how the athlete page shows competition history. -->
    <div class="grid gap-3 md:hidden">
      {#each filteredCompetitions() as competition (competition.slug)}
        <a
          href={resolve(`/competitions/${competition.slug}`)}
          class="block rounded-xl focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
        >
          <Card class="p-4">
            <h2 class="mb-2 text-base font-medium text-white">{competition.name}</h2>
            <div class="flex flex-wrap items-center gap-x-2 gap-y-1 text-xs text-zinc-500">
              <span>{competitionDates(competition.start_date, competition.end_date)}</span>
              {#if formatLocation(competition.country, competition.region, competition.city)}
                <span aria-hidden="true">&middot;</span>
                <span
                  >{formatLocation(competition.country, competition.region, competition.city)}</span
                >
              {/if}
              <span aria-hidden="true">&middot;</span>
              <span title={competition.federation.name}>
                {competition.federation.abbreviation || competition.federation.name}
              </span>
            </div>
            <div class="mt-3">
              {@render statusText(competition.status)}
            </div>
          </Card>
        </a>
      {/each}
    </div>

    <div class="hidden md:block">
      <Table>
        {#snippet head()}
          <th class="{TABLE_HEAD_CELL} text-zinc-400">Competition</th>
          <th class="{TABLE_HEAD_CELL} text-zinc-400">Date</th>
          <th class="{TABLE_HEAD_CELL} text-zinc-400">Location</th>
          <th class="{TABLE_HEAD_CELL} text-zinc-400">Federation</th>
          <th class="{TABLE_HEAD_CELL} text-zinc-400">Status</th>
        {/snippet}

        {#snippet body()}
          {#each filteredCompetitions() as competition (competition.slug)}
            <tr
              class="border-b border-zinc-800/50 transition-colors even:bg-zinc-900/60 hover:bg-zinc-800/50"
            >
              <td class="{TABLE_CELL} text-white">
                <a
                  href={resolve(`/competitions/${competition.slug}`)}
                  class="underline hover:text-zinc-300"
                >
                  {competition.name}
                </a>
              </td>
              <td class="{TABLE_CELL} whitespace-nowrap text-zinc-400">
                {competitionDates(competition.start_date, competition.end_date)}
              </td>
              <td class="{TABLE_CELL} text-zinc-400">
                {formatLocation(competition.country, competition.region, competition.city)}
              </td>
              <td class="{TABLE_CELL} text-zinc-400" title={competition.federation.name}>
                {competition.federation.abbreviation || competition.federation.name}
              </td>
              <td class={TABLE_CELL}>
                {@render statusText(competition.status)}
              </td>
            </tr>
          {/each}
        {/snippet}
      </Table>
    </div>

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
