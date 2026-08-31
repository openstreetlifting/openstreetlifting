<script lang="ts">
  import type { PageData } from './$types';
  import {
    Card,
    Breadcrumb,
    Pagination,
    FilterBar,
    Table,
    TABLE_CELL,
    TABLE_HEAD_CELL,
  } from '$lib/components/ui';
  import { resolve } from '$app/paths';
  import { goto } from '$app/navigation';
  import { SvelteURLSearchParams } from 'svelte/reactivity';
  import { page as currentPage, navigating } from '$app/state';
  import { formatDate, formatLocation, formatCountdown, countryName } from '$lib/utils';
  import { CELL, TEXT_CELL } from '$lib/constants/table';
  import { FIELD, TEXT, CONTROL } from '$lib/constants/typography';
  import Seo from '$lib/components/seo.svelte';
  import { breadcrumbLd, listingSeo } from '$lib/seo';

  let { data }: { data: PageData } = $props();

  const competitions = $derived(data.competitions);
  const pagination = $derived(data.pagination);
  const busy = $derived(navigating.to?.url.pathname === currentPage.url.pathname);

  // Past and future are two lists, not two filters on one: they sort in opposite
  // directions and answer different questions, so they are tabs rather than a
  // segmented control sitting among the filters that narrow them.
  const TABS = [
    { status: 'completed', label: 'Results' },
    { status: 'upcoming', label: 'Upcoming' },
  ] as const;

  const showsUpcoming = $derived(data.status === 'upcoming');

  let search = $state(data.q ?? '');
  let federation = $state(data.federation ?? null);
  let country = $state(data.country ?? null);
  let year = $state(data.year ?? null);

  const narrowed = $derived(Boolean(search || federation || country || year));

  const activeFilters = $derived([federation, country, year].filter(Boolean).length);

  // Paging and filtering live in the URL so a page of results can be linked to,
  // and so a filter narrows the whole archive rather than the current page.
  // Defaults stay out of the query string, matching the rankings tables.
  function apply(next: { status?: string; page?: number } = {}) {
    const target = next.status ?? data.status;
    const params = new SvelteURLSearchParams();

    if (target === 'upcoming') params.set('status', 'upcoming');
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
    return apply();
  }

  function competitionDates(start: string | null, end: string | null): string {
    const from = formatDate(start);
    return end && end !== start ? `${from} - ${formatDate(end)}` : from;
  }

  const SELECT = `w-full ${FIELD} px-3 py-2 sm:w-auto`;

  const seo = $derived(listingSeo(currentPage.url));

  const description = $derived(
    narrowed || showsUpcoming
      ? 'Streetlifting competition results by federation, country and year, with muscle up, pull up, dips and squat standings for every meet in the archive.'
      : `Results from ${pagination.total_items} streetlifting competitions worldwide, with muscle up, pull up, dips and squat standings, plus the calendar of upcoming meets.`
  );
</script>

<Seo
  title="Streetlifting competition results"
  {description}
  canonical={seo.canonical}
  noindex={seo.noindex}
  jsonLd={[
    breadcrumbLd([
      { name: 'Rankings', path: '/' },
      { name: 'Competitions', path: '/competitions' },
    ]),
  ]}
/>

<div class="mx-auto max-w-[var(--content-max-width)] px-4 py-4 sm:px-6 sm:py-12">
  <Breadcrumb items={[{ label: 'Rankings', href: '/' }, { label: 'Competitions' }]} />

  <!-- The breadcrumb above says which page this is, so the heading would only
       repeat it. It stays in the document for screen readers and crawlers. -->
  <h1 class="sr-only">Streetlifting competitions</h1>

  <nav class="mb-4 flex items-center gap-5 border-b border-zinc-800/60">
    {#each TABS as tab (tab.status)}
      {@const active = data.status === tab.status}
      <button
        onclick={() => apply({ status: tab.status })}
        aria-current={active ? 'page' : undefined}
        class="-mb-px flex items-baseline gap-1.5 border-b-2 pb-2 {CONTROL} transition-colors focus:ring-2 focus:ring-zinc-500 focus:outline-none
          {active
          ? 'border-white text-white'
          : 'border-transparent text-zinc-500 hover:text-zinc-300'}"
      >
        {tab.label}
        {#if data.counts[tab.status] !== undefined}
          <span class="{TEXT.micro} tabular-nums {active ? 'text-zinc-400' : 'text-zinc-600'}">
            {data.counts[tab.status]}
          </span>
        {/if}
      </button>
    {/each}
  </nav>

  <FilterBar
    bind:search
    placeholder="Search a competition"
    onSearch={() => apply()}
    activeCount={activeFilters}
  >
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
  </FilterBar>

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
          {#if narrowed}
            No competitions match your filters
          {:else if showsUpcoming}
            No competitions are planned yet
          {:else}
            No competitions found
          {/if}
        </p>
        <div class="mt-4 flex flex-wrap justify-center gap-4 text-xs">
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
    <!-- The same table at every width, like the rankings board: it pans sideways
         on a phone rather than becoming a different thing to read. -->
    <Table>
      {#snippet head()}
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Competition</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">{showsUpcoming ? 'When' : 'Lifters'}</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Date</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Location</th>
        <th class="{TABLE_HEAD_CELL} text-zinc-400">Federation</th>
      {/snippet}

      {#snippet body()}
        {#each competitions as competition (competition.slug)}
          <tr class="border-b border-zinc-800/50 transition-colors">
            <td class="{TABLE_CELL} {CELL.identity}">
              <a
                href={resolve(`/competitions/${competition.slug}`)}
                class="{TEXT_CELL.competition} underline hover:text-zinc-300"
              >
                {competition.name}
              </a>
            </td>
            <td class="{TABLE_CELL} {CELL.data} whitespace-nowrap tabular-nums">
              {showsUpcoming
                ? formatCountdown(competition.start_date)
                : (competition.lifter_count ?? 0)}
            </td>
            <td class="{TABLE_CELL} whitespace-nowrap text-zinc-400">
              {competitionDates(competition.start_date, competition.end_date)}
            </td>
            <td class="{TABLE_CELL} {CELL.data}">
              <span class={TEXT_CELL.location}>
                {formatLocation(competition.country, competition.region, competition.city)}
              </span>
            </td>
            <td class="{TABLE_CELL} {CELL.data}" title={competition.federation.name}>
              <span class={TEXT_CELL.federation}>
                {competition.federation.abbreviation || competition.federation.name}
              </span>
            </td>
          </tr>
        {/each}
      {/snippet}
    </Table>

    <div class="mt-4 flex flex-wrap items-center justify-between gap-3 sm:mt-8">
      <span class="text-xs text-zinc-500">
        {pagination.total_items} streetlifting competitions
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
