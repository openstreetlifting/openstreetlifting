<script lang="ts">
  import type { PageData } from './$types';
  import {
    Card,
    Breadcrumb,
    Pagination,
    Table,
    TABLE_CELL,
    TABLE_HEAD_CELL,
  } from '$lib/components/ui';
  import { resolve } from '$app/paths';
  import { goto } from '$app/navigation';
  import { SvelteURLSearchParams } from 'svelte/reactivity';
  import { page as currentPage, navigating } from '$app/state';
  import { formatDate, formatLocation } from '$lib/utils';
  import { COMPETITION_STATUSES, COMPETITION_STATUS_FILTERS } from '$lib/constants/competition';
  import type { Competition, CompetitionStatus } from '$lib/types/competition';

  let { data }: { data: PageData } = $props();

  const isLifted = (competition: Competition) => competition.status !== 'upcoming';

  const competitions = $derived(data.competitions);
  const pagination = $derived(data.pagination);
  const statusFilter = $derived(data.status ?? 'all');
  const busy = $derived(navigating.to?.url.pathname === currentPage.url.pathname);

  const filters = [
    { value: 'all', label: 'All' },
    ...COMPETITION_STATUS_FILTERS.map(({ value, label }) => ({ value, label })),
  ] as const;

  // Paging and filtering live in the URL so a page of results can be linked to,
  // and so the filter narrows the whole archive rather than the current page.
  // Defaults stay out of the query string, matching the rankings tables.
  function show(status: CompetitionStatus | 'all', page = 1) {
    const params = new SvelteURLSearchParams();
    if (status !== 'all') params.set('status', status);
    if (page > 1) params.set('page', String(page));

    const query = params.toString();
    return goto(resolve(query ? `/competitions?${query}` : '/competitions'), {
      keepFocus: true,
      noScroll: true,
    });
  }

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
  <Breadcrumb items={[{ label: 'Rankings', href: '/' }, { label: 'Competitions' }]} />

  <div class="mb-8">
    <h1 class="mb-4 text-3xl font-light tracking-tight text-white sm:text-4xl">Competitions</h1>
  </div>

  <div class="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-end">
    <div class="flex flex-wrap gap-2">
      {#each filters as filter (filter.value)}
        <button
          onclick={() => show(filter.value)}
          class="rounded-lg px-4 py-2 text-sm font-medium transition-colors focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none
					{statusFilter === filter.value
            ? 'bg-white text-zinc-950'
            : 'bg-zinc-800/50 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-300'}"
        >
          {filter.label}
        </button>
      {/each}
    </div>
  </div>

  {#if data.error}
    <Card class="p-8">
      <div class="text-center">
        <p class="text-red-400">{data.error}</p>
      </div>
    </Card>
  {:else if competitions.length === 0}
    <Card class="p-8">
      <div class="text-center">
        <p class="text-zinc-400">
          {statusFilter !== 'all' ? 'No competitions match your filters' : 'No competitions found'}
        </p>
        {#if statusFilter !== 'all'}
          <button
            onclick={() => show('all')}
            class="mt-4 text-sm text-zinc-500 underline hover:text-zinc-300 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
          >
            Clear filters
          </button>
        {/if}
      </div>
    </Card>
  {:else}
    {#snippet statusText(status: CompetitionStatus)}
      {@const match = COMPETITION_STATUSES.find((known) => known.value === status)}
      {#if match}
        <span class={match.text}>{match.label}</span>
      {/if}
    {/snippet}

    <!-- A 5 column table does not fit a phone, so the same rows read as cards there,
         matching how the athlete page shows competition history. -->
    {#snippet competitionCard(competition: Competition)}
      <Card class="p-4">
        <h2 class="mb-2 text-base font-medium text-white">{competition.name}</h2>
        <div class="flex flex-wrap items-center gap-x-2 gap-y-1 text-xs text-zinc-500">
          <span>{competitionDates(competition.start_date, competition.end_date)}</span>
          {#if formatLocation(competition.country, competition.region, competition.city)}
            <span aria-hidden="true">&middot;</span>
            <span>{formatLocation(competition.country, competition.region, competition.city)}</span>
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
    {/snippet}

    <div class="grid gap-3 md:hidden">
      {#each competitions as competition (competition.slug)}
        {#if isLifted(competition)}
          <a
            href={resolve(`/competitions/${competition.slug}`)}
            class="block rounded-xl focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
          >
            {@render competitionCard(competition)}
          </a>
        {:else}
          {@render competitionCard(competition)}
        {/if}
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
          {#each competitions as competition (competition.slug)}
            <tr
              class="border-b border-zinc-800/50 transition-colors even:bg-zinc-900/60 hover:bg-zinc-800/50"
            >
              <td class="{TABLE_CELL} text-white">
                {#if isLifted(competition)}
                  <a
                    href={resolve(`/competitions/${competition.slug}`)}
                    class="underline hover:text-zinc-300"
                  >
                    {competition.name}
                  </a>
                {:else}
                  {competition.name}
                {/if}
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

    <div class="mt-8 flex flex-wrap items-center justify-between gap-3">
      <span class="text-sm text-zinc-500">
        {pagination.total_items} competitions
        {#if pagination.total_pages > 1}
          &middot; page {pagination.page} of {pagination.total_pages}
        {/if}
      </span>
      {#if pagination.total_pages > 1}
        <Pagination
          page={pagination.page}
          totalPages={pagination.total_pages}
          disabled={busy}
          onNavigate={(target) => show(statusFilter, target)}
        />
      {/if}
    </div>
  {/if}
</div>
