<script lang="ts">
  import type { PageData } from './$types';
  import { Breadcrumb, Flag } from '$lib/components/ui';
  import CompetitionsTable from '$lib/components/competitions-table.svelte';
  import RankingList from '$lib/components/ranking-list.svelte';
  import Seo from '$lib/components/seo.svelte';
  import { page as currentPage, navigating } from '$app/state';
  import { rankingsHref } from '$lib/state/rankings-return.svelte';
  import { slowNavigation } from '$lib/state/slow-navigation.svelte';
  import { breadcrumbLd, federationLd, listingSeo } from '$lib/seo';
  import { federationPath } from '$lib/utils';
  import { TEXT } from '$lib/constants/typography';

  let { data }: { data: PageData } = $props();

  const federation = $derived(data.federation);
  const pagination = $derived(data.pagination);
  const firstPage = $derived(pagination.page <= 1);
  const loading = slowNavigation(() => navigating.to?.url.pathname === currentPage.url.pathname);
  const busy = $derived(loading.current);

  const seo = $derived(listingSeo(currentPage.url));
  const path = $derived(federationPath(federation.name));

  const years = $derived(
    federation.firstYear === null
      ? null
      : federation.firstYear === federation.lastYear
        ? String(federation.firstYear)
        : `${federation.firstYear} – ${federation.lastYear}`
  );

  const title = $derived(
    firstPage
      ? `${federation.name} Streetlifting results and rankings`
      : `${federation.name} Streetlifting rankings, page ${pagination.page}`
  );

  const description = $derived(
    [
      `${federation.name} Streetlifting results`,
      federation.competitions
        ? `: ${federation.competitions} ${federation.competitions === 1 ? 'competition' : 'competitions'}`
        : '',
      federation.countries > 1 ? ` in ${federation.countries} countries` : '',
      years ? ` (${years})` : '',
      pagination.total_items ? ` and ${pagination.total_items} ranked athletes` : '',
      ', with muscle up, pull up, dips and squat standings.',
      firstPage ? '' : ` Page ${pagination.page} of ${pagination.total_pages}.`,
    ].join('')
  );
</script>

<Seo
  {title}
  {description}
  canonical={seo.canonical}
  noindex={seo.noindex}
  jsonLd={[
    federationLd(federation, description),
    breadcrumbLd([
      { name: 'Rankings', path: '/' },
      { name: 'Competitions', path: '/competitions' },
      { name: federation.name, path },
    ]),
  ]}
/>

<div class="mx-auto max-w-page px-4 py-4 sm:px-6 sm:py-12">
  <Breadcrumb
    items={[
      { label: 'Rankings', href: rankingsHref() },
      { label: 'Competitions', href: '/competitions' },
      { label: federation.name },
    ]}
  />

  <header class="mb-6 border-b border-stroke pb-5">
    <h1 class="{TEXT.title} flex min-w-0 items-start gap-3 text-ink">
      {#if federation.country}
        <Flag
          countryCode={federation.country}
          link
          class="mt-[0.2em] shrink-0 [--flag-height:0.8em]"
        />
      {/if}
      <span class="min-w-0 break-words">{federation.name}</span>
    </h1>
    {#if years || federation.countries > 1 || (federation.abbreviation && federation.abbreviation !== federation.name)}
      <dl
        class="mt-4 grid grid-cols-[6rem_minmax(0,1fr)] gap-x-3 gap-y-1.5 text-sm sm:grid-cols-[7rem_minmax(0,1fr)]"
      >
        {#if federation.abbreviation && federation.abbreviation !== federation.name}
          <dt class="text-muted">Abbreviation</dt>
          <dd class="text-ink">{federation.abbreviation}</dd>
        {/if}
        {#if years}
          <dt class="text-muted">Years</dt>
          <dd class="text-ink">{years}</dd>
        {/if}
        {#if federation.countries > 1}
          <dt class="text-muted">Host countries</dt>
          <dd class="text-ink">{federation.countries}</dd>
        {/if}
      </dl>
    {/if}
  </header>

  {#if data.results.length > 0}
    <section class="mb-8 sm:mb-12">
      <h2 class="mb-3 {TEXT.heading} text-ink">Competitions</h2>
      <CompetitionsTable competitions={data.results} showFederation={false} />
    </section>
  {/if}

  {#if data.rankings.length > 0}
    <section>
      <h2 class="mb-3 {TEXT.heading} text-ink">Athletes</h2>
      <RankingList entries={data.rankings} {pagination} showFederation={false} {busy} />
    </section>
  {/if}

  {#if data.upcoming.length > 0}
    <section class="mt-8 sm:mt-12">
      <h2 class="mb-3 {TEXT.heading} text-ink">Upcoming</h2>
      <CompetitionsTable competitions={data.upcoming} upcoming showFederation={false} />
    </section>
  {/if}
</div>
