<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import type { PageData } from './$types';
  import { Card, Breadcrumb, FilterBar, SearchEmpty } from '#lib/components/ui/index.js';
  import CompetitionsTable from '#lib/components/competitions-table.svelte';
  import CompetitionFormatFilter from '#lib/components/competition-format-filter.svelte';
  import { resolve } from '$app/paths';
  import { rankingsHref } from '#lib/state/rankings-return.svelte.js';
  import { slowNavigation } from '#lib/state/slow-navigation.svelte.js';
  import { afterNavigate } from '$app/navigation';
  import { ListingSearch } from '#lib/state/listing-search.svelte.js';
  import { untrack } from 'svelte';
  import { SvelteURLSearchParams } from 'svelte/reactivity';
  import { page as currentPage, navigating } from '$app/state';
  import { countryName, formatDate, formatLocation } from '#lib/utils/index.js';
  import { FIELD, TEXT, CONTROL } from '#lib/constants/typography.js';
  import Seo from '#lib/components/seo.svelte';
  import { breadcrumbLd, listingSeo } from '#lib/seo/index.js';

  const i18n = useI18n();

  let { data }: { data: PageData } = $props();

  const competitions = $derived(data.competitions);
  const pagination = $derived(data.pagination);
  const loading = slowNavigation(() => navigating.to?.url.pathname === currentPage.url.pathname);
  const busy = $derived(loading.current);

  const TABS = [
    { status: 'completed', label: 'Results' },
    { status: 'live', label: 'In Progress' },
    { status: 'upcoming', label: 'Upcoming' },
  ] as const;

  const showsUpcoming = $derived(data.status === 'upcoming');

  const search = new ListingSearch(currentPage.url);
  let federation = $state(untrack(() => data.federation ?? null));
  let country = $state(untrack(() => data.country ?? null));
  let year = $state(untrack(() => data.year ?? null));
  let event = $state(untrack(() => data.event ?? ''));

  // The page outlives a navigation, so a link that drops the query string has to
  // reach the controls as well as the rows.
  afterNavigate(({ type, shallow }) => {
    if (shallow) return;

    search.sync(currentPage.url, type);
    federation = data.federation ?? null;
    country = data.country ?? null;
    year = data.year ?? null;
    event = data.event ?? '';
  });

  const narrowed = $derived(Boolean(search.value || federation || country || year || event));
  const canReset = $derived(narrowed || pagination.page > 1);

  const activeFilters = $derived([federation, country, year, event].filter(Boolean).length);

  // Paging and filtering live in the URL so a page of results can be linked to,
  // and so a filter narrows the whole archive rather than the current page.
  // Defaults stay out of the query string, matching the rankings tables.
  function listingHref(next: { status?: string; page?: number } = {}, query = search.applied) {
    const target = next.status ?? data.status;
    const params = new SvelteURLSearchParams();

    if (target !== 'completed') params.set('status', target);
    if (query.trim()) params.set('q', query.trim());
    if (federation) params.set('federation', federation);
    if (country) params.set('country', country);
    if (year) params.set('year', String(year));
    if (event) params.set('event', event);
    if (next.page && next.page > 1) params.set('page', String(next.page));

    const queryString = params.toString();
    return i18n.href(resolve(queryString ? `/competitions?${queryString}` : '/competitions'));
  }

  function apply(next: { status?: string; page?: number } = {}, replaceState = false) {
    return search.navigate(listingHref(next, search.value), replaceState);
  }

  function clearFilters() {
    search.value = '';
    federation = null;
    country = null;
    year = null;
    event = '';
    return apply();
  }

  const SELECT = `w-full ${FIELD} px-3 py-2 sm:w-auto`;

  const seo = $derived(listingSeo(currentPage.url));

  const description = $derived(
    narrowed || data.status !== 'completed'
      ? i18n.t(
          'Streetlifting competition results by federation, country and year, with muscle up, pull up, dips and squat standings for every meet in the archive.'
        )
      : i18n.t(
          'Results from {count} Streetlifting competitions worldwide, with muscle up, pull up, dips and squat standings, plus the calendar of upcoming meets.',
          { count: i18n.number(pagination.total_items, 0) }
        )
  );
</script>

<Seo
  title={i18n.t('Streetlifting competition results')}
  {description}
  canonical={seo.canonical}
  noindex={seo.noindex}
  jsonLd={[
    breadcrumbLd(
      [
        { name: i18n.t('Rankings'), path: '/' },
        { name: i18n.t('Competitions'), path: '/competitions' },
      ],
      i18n.locale
    ),
  ]}
/>

<div class="mx-auto max-w-page px-4 py-4 sm:px-6 sm:py-12">
  <Breadcrumb
    items={[{ label: i18n.t('Rankings'), href: rankingsHref() }, { label: i18n.t('Competitions') }]}
  />

  <h1 class="sr-only">{i18n.t('Streetlifting competitions')}</h1>

  {#if data.runningCompetitions.length > 0}
    <section
      aria-labelledby="running-heading"
      class="mb-6 rounded-lg border border-stroke-strong bg-surface-subtle px-4 py-3"
    >
      <h2 id="running-heading" class="flex items-center gap-2 text-sm font-medium text-ink">
        <span aria-hidden="true" class="size-1.5 shrink-0 rounded-full bg-success"></span>
        {i18n.t('Hey! Those competitions are currently running')}
      </h2>
      <ul class="mt-2 space-y-2 text-sm">
        {#each data.runningCompetitions as competition (competition.slug)}
          <li class="flex flex-wrap items-center gap-x-6 gap-y-1">
            <a
              href={i18n.href(resolve(`competitions/${competition.slug}`))}
              class="inline-flex min-h-8 items-center gap-1.5 text-ink underline decoration-stroke-strong underline-offset-4 hover:text-secondary focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-focus"
              >{competition.name}</a
            >

            <dl class="flex flex-wrap items-center gap-x-6 gap-y-1 text-secondary">
              {#if competition.event_code}
                <div>
                  <dt class="sr-only">{i18n.t('Format')}</dt>
                  <dd
                    title={competition.movements
                      .map(({ movement_name }) => i18n.movement(movement_name))
                      .join(', ')}
                  >
                    {competition.event_code}
                  </dd>
                </div>
              {/if}
              <div>
                <dt class="sr-only">{i18n.t('Federation')}</dt>
                <dd title={competition.federation.name}>
                  {competition.federation.abbreviation || competition.federation.name}
                </dd>
              </div>
              {#if competition.country || competition.region || competition.city}
                <div>
                  <dt class="sr-only">{i18n.t('Location')}</dt>
                  <dd>
                    {formatLocation(competition.country, competition.region, competition.city)}
                  </dd>
                </div>
              {/if}
              {#if competition.start_date}
                <div>
                  <dt class="sr-only">{i18n.t('Date')}</dt>
                  <dd class="flex flex-wrap items-baseline gap-x-1.5 tabular-nums">
                    <time class="whitespace-nowrap" datetime={competition.start_date}
                      >{formatDate(competition.start_date, i18n.locale)}</time
                    >
                    {#if competition.end_date && competition.end_date !== competition.start_date}
                      <span class="inline-flex gap-x-1.5">
                        <span>{i18n.t('to')}</span>
                        <time class="whitespace-nowrap" datetime={competition.end_date}
                          >{formatDate(competition.end_date, i18n.locale)}</time
                        >
                      </span>
                    {/if}
                  </dd>
                </div>
              {/if}
            </dl>
          </li>
        {/each}
        {#if (data.counts.live ?? 0) > data.runningCompetitions.length}
          <li>
            <a
              href={listingHref({ status: 'live' })}
              class="inline-flex min-h-8 items-center text-secondary underline underline-offset-4"
              >{i18n.t('View all in progress')}</a
            >
          </li>
        {/if}
      </ul>
    </section>
  {/if}

  <nav class="mb-4 flex items-center gap-5 border-b border-stroke">
    {#each TABS as tab (tab.status)}
      {@const active = data.status === tab.status}
      <button
        onclick={() => apply({ status: tab.status })}
        aria-current={active ? 'page' : undefined}
        class="-mb-px flex items-baseline gap-1.5 border-b-2 pb-2 {CONTROL} transition-colors focus:ring-2 focus:ring-focus focus:outline-none
 {active ? 'border-ink text-ink' : 'border-transparent text-muted hover:text-secondary'}"
      >
        {i18n.t(tab.label)}
        {#if data.counts[tab.status] !== undefined}
          <span class="{TEXT.micro} {active ? 'text-secondary' : 'text-muted'}">
            {data.counts[tab.status]}
          </span>
        {/if}
      </button>
    {/each}
  </nav>

  <FilterBar
    bind:search={search.value}
    placeholder={i18n.t('Search a competition')}
    onSearch={() => apply({}, true)}
    activeCount={activeFilters}
    onClear={clearFilters}
    clearable={narrowed}
  >
    <select bind:value={federation} onchange={() => apply()} class={SELECT}>
      <option value={null}>{i18n.t('All Federations')}</option>
      {#each data.facets.federations as option (option)}
        <option value={option}>{option}</option>
      {/each}
    </select>

    <CompetitionFormatFilter
      formats={data.facets.formats}
      value={event}
      onChange={(value) => {
        event = value;
        apply();
      }}
    />

    <select bind:value={year} onchange={() => apply()} class={SELECT}>
      <option value={null}>{i18n.t('All Years')}</option>
      {#each data.facets.years as option (option)}
        <option value={option}>{option}</option>
      {/each}
    </select>

    <select bind:value={country} onchange={() => apply()} class={SELECT}>
      <option value={null}>{i18n.t('All Countries')}</option>
      {#each data.facets.countries as option (option)}
        <option value={option}>{countryName(option, i18n.locale)}</option>
      {/each}
    </select>
  </FilterBar>

  {#if data.error}
    <Card class="max-w-summary p-8">
      <div class="text-center">
        <p class="text-danger">{i18n.t('Failed to load competitions')}</p>
      </div>
    </Card>
  {:else if competitions.length === 0 && !busy}
    <SearchEmpty
      title={canReset
        ? i18n.t('Oops, no competitions found.')
        : i18n.t('No competitions to show yet.')}
      description={canReset
        ? i18n.t('Lighten the filters or try another name.')
        : showsUpcoming
          ? i18n.t('No competitions are planned yet.')
          : data.status === 'live'
            ? i18n.t('No competitions are in progress right now.')
            : i18n.t('No competition results in this view yet.')}
      resetHref={canReset
        ? i18n.href(
            resolve(
              data.status === 'completed' ? '/competitions' : `/competitions?status=${data.status}`
            )
          )
        : undefined}
    />
  {:else}
    <CompetitionsTable
      {competitions}
      upcoming={showsUpcoming}
      {busy}
      pagination={{
        ...pagination,
        pageHref: (target) => listingHref({ page: target }),
      }}
    />
  {/if}
</div>
