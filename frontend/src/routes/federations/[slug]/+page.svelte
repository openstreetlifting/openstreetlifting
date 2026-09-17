<script lang="ts">
  import type { PageData } from './$types';
  import { Breadcrumb, Flag, Pagination } from '$lib/components/ui';
  import CompetitionsTable from '$lib/components/competitions-table.svelte';
  import RankingList from '$lib/components/ranking-list.svelte';
  import Seo from '$lib/components/seo.svelte';
  import { resolve } from '$app/paths';
  import { page as currentPage, navigating } from '$app/state';
  import { rankingsHref } from '$lib/state/rankings-return.svelte';
  import { slowNavigation } from '$lib/state/slow-navigation.svelte';
  import { breadcrumbLd, federationLd, listingSeo } from '$lib/seo';
  import { countryName, federationPath } from '$lib/utils';
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
        : `${federation.firstYear} - ${federation.lastYear}`
  );

  const title = $derived(
    firstPage
      ? `${federation.name} streetlifting results and rankings`
      : `${federation.name} streetlifting rankings, page ${pagination.page}`
  );

  const description = $derived(
    [
      `${federation.name} streetlifting results`,
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

  const pageHref = (target: number) => resolve(target > 1 ? `${path}?page=${target}` : path);
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
      { name: 'Federations', path: '/federations' },
      { name: federation.name, path },
    ]),
  ]}
/>

<div class="mx-auto max-w-page px-4 py-4 sm:px-6 sm:py-12">
  <Breadcrumb
    items={[
      { label: 'Rankings', href: rankingsHref() },
      { label: 'Federations', href: '/federations' },
      { label: federation.name },
    ]}
  />

  <div class="mb-6 sm:mb-10">
    <h1 class="{TEXT.title} flex min-w-0 items-center gap-3 text-ink">
      {#if federation.country}
        <Flag countryCode={federation.country} class="shrink-0 [--flag-height:0.8em]" />
      {/if}
      <span class="truncate">{federation.name}</span>
    </h1>

    <p class="mt-2 flex flex-wrap items-center gap-x-2 text-xs text-secondary sm:text-sm">
      {#if federation.abbreviation && federation.abbreviation !== federation.name}
        <span>{federation.abbreviation}</span>
        <span aria-hidden="true">&middot;</span>
      {/if}
      {#if federation.country}
        <span>{countryName(federation.country)}</span>
        <span aria-hidden="true">&middot;</span>
      {/if}
      <span class="whitespace-nowrap">
        {federation.competitions}
        {federation.competitions === 1 ? 'competition' : 'competitions'}
      </span>
      {#if federation.countries > 1}
        <span aria-hidden="true">&middot;</span>
        <span class="whitespace-nowrap">{federation.countries} countries</span>
      {/if}
      {#if years}
        <span aria-hidden="true">&middot;</span>
        <span class="whitespace-nowrap">{years}</span>
      {/if}
      <span aria-hidden="true">&middot;</span>
      <span class="whitespace-nowrap">{pagination.total_items} ranked athletes</span>
    </p>
  </div>

  {#if firstPage && data.results.length > 0}
    <section class="mb-8 sm:mb-12">
      <h2 class="mb-3 {TEXT.heading} text-ink">Competitions</h2>
      <CompetitionsTable competitions={data.results} showFederation={false} />
    </section>
  {/if}

  {#if data.rankings.length > 0}
    <section>
      <h2 class="mb-3 {TEXT.heading} text-ink">Athletes</h2>
      <RankingList entries={data.rankings} showFederation={false} {busy} />

      {#if pagination.total_pages > 1}
        <div class="mt-3 flex flex-wrap items-center justify-between gap-3">
          <span class="text-xs text-muted">
            Page {pagination.page} of {pagination.total_pages} &middot; {pagination.total_items} athletes
          </span>
          <Pagination
            page={pagination.page}
            totalPages={pagination.total_pages}
            disabled={busy}
            {pageHref}
          />
        </div>
      {/if}
    </section>
  {/if}

  {#if firstPage && data.upcoming.length > 0}
    <section class="mt-8 sm:mt-12">
      <h2 class="mb-3 {TEXT.heading} text-ink">Upcoming</h2>
      <CompetitionsTable competitions={data.upcoming} upcoming showFederation={false} />
    </section>
  {/if}
</div>
