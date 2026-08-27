<script lang="ts">
  import type { PageData } from './$types';
  import {
    Card,
    Breadcrumb,
    Pagination,
    SearchInput,
    Table,
    TABLE_CELL,
    TABLE_HEAD_CELL,
  } from '$lib/components/ui';
  import { resolve } from '$app/paths';
  import { goto } from '$app/navigation';
  import { SvelteURLSearchParams } from 'svelte/reactivity';
  import { page as currentPage, navigating } from '$app/state';
  import { formatDate, formatLocation, formatCountdown, countryName } from '$lib/utils';
  import { CELL } from '$lib/constants/table';
  import type { Competition } from '$lib/types/competition';
  import { FIELD, TEXT, CONTROL } from '$lib/constants/typography';

  let { data }: { data: PageData } = $props();

  type View = 'results' | 'upcoming' | 'all';

  const competitions = $derived(data.competitions);
  const pagination = $derived(data.pagination);
  const upcoming = $derived(data.upcoming);
  const busy = $derived(navigating.to?.url.pathname === currentPage.url.pathname);

  const view = $derived<View>(
    data.status === 'all' ? 'all' : data.status === 'upcoming' ? 'upcoming' : 'results'
  );

  const views: { value: View; label: string }[] = [
    { value: 'results', label: 'Results' },
    { value: 'upcoming', label: 'Planned' },
    { value: 'all', label: 'All' },
  ];

  let search = $state(data.q ?? '');
  let federation = $state(data.federation ?? null);
  let country = $state(data.country ?? null);
  let year = $state(data.year ?? null);

  const narrowed = $derived(Boolean(search || federation || country || year || data.status));

  // Paging and filtering live in the URL so a page of results can be linked to,
  // and so a filter narrows the whole archive rather than the current page.
  // Defaults stay out of the query string, matching the rankings tables.
  function apply(next: { view?: View; page?: number } = {}) {
    const target = next.view ?? view;
    const params = new SvelteURLSearchParams();

    if (target !== 'results') params.set('status', target === 'all' ? 'all' : 'upcoming');
    if (search) params.set('q', search);
    if (federation) params.set('federation', federation);
    if (country) params.set('country', country);
    if (year) params.set('year', String(year));
    if (next.page && next.page > 1) params.set('page', String(next.page));

    const query = params.toString();
    return goto(resolve(query ? `/competitions?${query}` : '/competitions'), {
      keepFocus: true,
      noScroll: true,
    });
  }

  function clearFilters() {
    search = '';
    federation = null;
    country = null;
    year = null;
    return apply({ view: 'results' });
  }

  function competitionDates(start: string | null, end: string | null): string {
    const from = formatDate(start);
    return end && end !== start ? `${from} - ${formatDate(end)}` : from;
  }

  const isLifted = (competition: Competition) => competition.status !== 'upcoming';

  const monthLabel = new Intl.DateTimeFormat('en', { month: 'long', year: 'numeric' });

  function dayOfMonth(date: string | null): string {
    return date ? date.slice(8, 10) : '';
  }

  // A countdown only tells the reader something while it is short. On a meet
  // half a year out every row reads "in 5 months" and the column is noise.
  function isSoon(date: string | null): boolean {
    if (!date) return false;
    return new Date(date).getTime() - Date.now() < 31 * 86_400_000;
  }

  function groupByMonth(list: Competition[]) {
    const months: { label: string; competitions: Competition[] }[] = [];
    for (const competition of list) {
      const label = competition.start_date
        ? monthLabel.format(new Date(competition.start_date))
        : 'Date to be confirmed';
      const last = months.at(-1);
      if (last?.label === label) last.competitions.push(competition);
      else months.push({ label, competitions: [competition] });
    }
    return months;
  }

  const SELECT = `w-full ${FIELD} px-3 py-2 sm:w-auto`;
</script>

<svelte:head>
  <title>Competitions - OpenStreetlifting</title>
  <meta name="description" content="List of availables competitions" />
</svelte:head>

<div class="mx-auto max-w-[var(--content-max-width)] px-4 py-8 sm:px-6 sm:py-12">
  <Breadcrumb items={[{ label: 'Rankings', href: '/' }, { label: 'Competitions' }]} />

  <div class="mb-8">
    <h1 class="mb-4 {TEXT.title} text-white">Competitions</h1>
  </div>

  {#if upcoming.length > 0}
    <section class="mb-8">
      <div class="mb-3 flex items-baseline justify-between">
        <h2 class="{TEXT.micro} font-medium tracking-wider text-zinc-500 uppercase">
          Next competitions
        </h2>
        <button
          onclick={() => apply({ view: 'upcoming' })}
          class="text-xs text-zinc-500 underline hover:text-zinc-300 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
        >
          See all planned
        </button>
      </div>

      <div class="rounded-lg border border-zinc-800/60 bg-zinc-900/30 px-4 py-3">
        {#each groupByMonth(upcoming) as month (month.label)}
          <h3 class="mt-3 mb-1.5 {TEXT.micro} tracking-wider text-zinc-600 uppercase first:mt-0">
            {month.label}
          </h3>
          {#each month.competitions as competition (competition.slug)}
            <a
              href={resolve(`/competitions/${competition.slug}`)}
              class="flex items-baseline gap-3 rounded py-1 text-xs hover:bg-zinc-800/40 focus:ring-2 focus:ring-zinc-500 focus:outline-none"
            >
              <span class="w-6 shrink-0 text-right tabular-nums text-zinc-500">
                {dayOfMonth(competition.start_date)}
              </span>
              <span class="truncate text-zinc-300">{competition.name}</span>
              <span class="shrink-0 text-zinc-600"
                >{competition.federation.abbreviation || competition.federation.name}</span
              >
              {#if formatLocation(competition.country, competition.city)}
                <span class="truncate text-zinc-600">
                  {formatLocation(competition.country, competition.city)}
                </span>
              {/if}
              {#if isSoon(competition.start_date)}
                <span class="ml-auto shrink-0 text-zinc-400">
                  {formatCountdown(competition.start_date)}
                </span>
              {/if}
            </a>
          {/each}
        {/each}
      </div>
    </section>
  {/if}

  <div
    class="mb-6 flex flex-wrap items-center gap-3 rounded-lg border border-zinc-800 bg-zinc-900/30 p-3"
  >
    <div class="w-full sm:w-64">
      <SearchInput
        bind:value={search}
        placeholder="Search a competition"
        onSearch={() => apply()}
      />
    </div>

    <select bind:value={federation} onchange={() => apply()} class={SELECT}>
      <option value={null}>All Federations</option>
      {#each data.facets.federations as option (option)}
        <option value={option}>{option}</option>
      {/each}
    </select>

    <select bind:value={year} onchange={() => apply()} class={SELECT}>
      <option value={null}>All Years</option>
      {#each data.facets.years as option (option)}
        <option value={option}>{option}</option>
      {/each}
    </select>

    <select bind:value={country} onchange={() => apply()} class={SELECT}>
      <option value={null}>All Countries</option>
      {#each data.facets.countries as option (option)}
        <option value={option}>{countryName(option)}</option>
      {/each}
    </select>

    <div class="ml-auto flex flex-wrap gap-2">
      {#each views as option (option.value)}
        <button
          onclick={() => apply({ view: option.value })}
          class="rounded-lg px-4 py-2 {CONTROL} transition-colors focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none
            {view === option.value
            ? 'bg-white text-zinc-950'
            : 'bg-zinc-800/50 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-300'}"
        >
          {option.label}
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
          {narrowed ? 'No competitions match your filters' : 'No competitions found'}
        </p>
        <div class="mt-4 flex flex-wrap justify-center gap-4 text-xs">
          <!-- The default view is results only, so a filter that matches nothing
               but planned competitions would otherwise be a dead end. -->
          {#if view === 'results'}
            <button
              onclick={() => apply({ view: 'all' })}
              class="text-zinc-500 underline hover:text-zinc-300 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
            >
              Include planned competitions
            </button>
          {/if}
          {#if narrowed}
            <button
              onclick={clearFilters}
              class="text-zinc-500 underline hover:text-zinc-300 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
            >
              Clear filters
            </button>
          {/if}
        </div>
      </div>
    </Card>
  {:else}
    <!-- A competition with lifters has results to read, one without is still to come.
         The row says which by what it offers, so no status column is needed. -->
    {#snippet lifters(competition: Competition)}
      {#if isLifted(competition)}
        <span class="tabular-nums">{competition.lifter_count ?? 0}</span>
      {:else}
        <span class="text-zinc-500">{formatCountdown(competition.start_date)}</span>
      {/if}
    {/snippet}

    <!-- A 5 column table does not fit a phone, so the same rows read as cards there,
         matching how the athlete page shows competition history. -->
    {#snippet competitionCard(competition: Competition)}
      <Card class="p-4">
        <h2 class="mb-2 {TEXT.subheading} text-white">{competition.name}</h2>
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
        <div class="mt-3 text-xs text-zinc-400">
          {#if isLifted(competition)}
            {competition.lifter_count ?? 0} lifters
          {:else}
            {formatCountdown(competition.start_date)}
          {/if}
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
          <th class="{TABLE_HEAD_CELL} text-zinc-400">Lifters</th>
        {/snippet}

        {#snippet body()}
          {#each competitions as competition (competition.slug)}
            <tr
              class="border-b border-zinc-800/50 transition-colors even:bg-zinc-900/60 hover:bg-zinc-800/50"
            >
              <td class="{TABLE_CELL} {CELL.identity}">
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
              <td class="{TABLE_CELL} {CELL.data}">
                {formatLocation(competition.country, competition.region, competition.city)}
              </td>
              <td class="{TABLE_CELL} {CELL.data}" title={competition.federation.name}>
                {competition.federation.abbreviation || competition.federation.name}
              </td>
              <td class="{TABLE_CELL} {CELL.data}">
                {@render lifters(competition)}
              </td>
            </tr>
          {/each}
        {/snippet}
      </Table>
    </div>

    <div class="mt-8 flex flex-wrap items-center justify-between gap-3">
      <span class="text-xs text-zinc-500">
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
          onNavigate={(target) => apply({ page: target })}
        />
      {/if}
    </div>
  {/if}
</div>
