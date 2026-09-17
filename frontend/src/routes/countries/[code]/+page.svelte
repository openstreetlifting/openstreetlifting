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
  import { breadcrumbLd, listingSeo } from '$lib/seo';
  import { countryName, countryPath, federationPath } from '$lib/utils';
  import { TEXT } from '$lib/constants/typography';

  let { data }: { data: PageData } = $props();

  const name = $derived(countryName(data.code));
  const pagination = $derived(data.pagination);
  const firstPage = $derived(pagination.page <= 1);
  const loading = slowNavigation(() => navigating.to?.url.pathname === currentPage.url.pathname);
  const busy = $derived(loading.current);

  const seo = $derived(listingSeo(currentPage.url));
  const path = $derived(countryPath(data.code));

  const title = $derived(
    firstPage
      ? `Streetlifting in ${name}: results and rankings`
      : `Streetlifting in ${name}: rankings, page ${pagination.page}`
  );

  const description = $derived(
    [
      `Streetlifting in ${name}: `,
      `${pagination.total_items} ranked ${pagination.total_items === 1 ? 'athlete' : 'athletes'}`,
      data.results.length
        ? ` and ${data.results.length} ${data.results.length === 1 ? 'competition' : 'competitions'} held there`
        : '',
      data.federations.length ? `, including ${data.federations.join(', ')}` : '',
      ', with muscle up, pull up, dips and squat results.',
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
    breadcrumbLd([
      { name: 'Rankings', path: '/' },
      { name: 'Countries', path: '/countries' },
      { name, path },
    ]),
  ]}
/>

<div class="mx-auto max-w-page px-4 py-4 sm:px-6 sm:py-12">
  <Breadcrumb
    items={[
      { label: 'Rankings', href: rankingsHref() },
      { label: 'Countries', href: '/countries' },
      { label: name },
    ]}
  />

  <div class="mb-6 sm:mb-10">
    <h1 class="{TEXT.title} flex min-w-0 items-center gap-3 text-ink">
      <Flag countryCode={data.code} class="shrink-0 [--flag-height:0.8em]" />
      <span class="truncate">{name}</span>
    </h1>

    <p class="mt-2 flex flex-wrap items-center gap-x-2 text-xs text-secondary sm:text-sm">
      <span class="whitespace-nowrap">{pagination.total_items} ranked athletes</span>
      <span aria-hidden="true">&middot;</span>
      <span class="whitespace-nowrap">
        {data.results.length}
        {data.results.length === 1 ? 'competition' : 'competitions'}
      </span>
      {#each data.federations as federation (federation)}
        <span aria-hidden="true">&middot;</span>
        <a href={resolve(federationPath(federation))} class="underline hover:text-ink">
          {federation}
        </a>
      {/each}
    </p>
  </div>

  {#if firstPage && data.results.length > 0}
    <section class="mb-8 sm:mb-12">
      <h2 class="mb-3 {TEXT.heading} text-ink">Competitions</h2>
      <CompetitionsTable competitions={data.results} />
    </section>
  {/if}

  {#if data.rankings.length > 0}
    <section>
      <h2 class="mb-3 {TEXT.heading} text-ink">Athletes</h2>
      <RankingList entries={data.rankings} {busy} />

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
      <CompetitionsTable competitions={data.upcoming} upcoming />
    </section>
  {/if}
</div>
