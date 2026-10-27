<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import type { PageData } from './$types';
  import { Breadcrumb, Flag } from '#lib/components/ui/index.js';
  import CompetitionsTable from '#lib/components/competitions-table.svelte';
  import RankingList from '#lib/components/ranking-list.svelte';
  import Seo from '#lib/components/seo.svelte';
  import { page as currentPage, navigating } from '$app/state';
  import { rankingsHref } from '#lib/state/rankings-return.svelte.js';
  import { slowNavigation } from '#lib/state/slow-navigation.svelte.js';
  import { breadcrumbLd, federationLd, listingSeo } from '#lib/seo/index.js';
  import { federationPath } from '#lib/utils/index.js';
  import { TEXT } from '#lib/constants/typography.js';

  const i18n = useI18n();

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
      ? i18n.t('{federation} Streetlifting results and rankings', { federation: federation.name })
      : i18n.t('{federation} Streetlifting rankings, page {page}', {
          federation: federation.name,
          page: pagination.page,
        })
  );
  const description = $derived(
    i18n.t(
      '{federation}: {competitions} competitions and {athletes} ranked athletes. Muscle-up, pull-up, dips and squat results.',
      {
        federation: federation.name,
        competitions: i18n.number(federation.competitions, 0),
        athletes: i18n.number(pagination.total_items, 0),
      }
    ) +
      (firstPage
        ? ''
        : ' ' +
          i18n.t('Page {page} of {pages}.', {
            page: pagination.page,
            pages: pagination.total_pages,
          }))
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
      { name: i18n.t('Rankings'), path: '/' },
      { name: i18n.t('Competitions'), path: '/competitions' },
      { name: federation.name, path },
    ]),
  ]}
/>

<div class="mx-auto max-w-page px-4 py-4 sm:px-6 sm:py-12">
  <Breadcrumb
    items={[
      { label: i18n.t('Rankings'), href: rankingsHref() },
      { label: i18n.t('Competitions'), href: '/competitions' },
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
          <dt class="text-muted">{i18n.t('Abbreviation')}</dt>
          <dd class="text-ink">{federation.abbreviation}</dd>
        {/if}
        {#if years}
          <dt class="text-muted">{i18n.t('Years')}</dt>
          <dd class="text-ink">{years}</dd>
        {/if}
        {#if federation.countries > 1}
          <dt class="text-muted">{i18n.t('Host countries')}</dt>
          <dd class="text-ink">{federation.countries}</dd>
        {/if}
      </dl>
    {/if}
  </header>

  {#if data.results.length > 0}
    <section class="mb-8 sm:mb-12">
      <h2 class="mb-3 {TEXT.heading} text-ink">{i18n.t('Competitions')}</h2>
      <CompetitionsTable competitions={data.results} showFederation={false} />
    </section>
  {/if}

  {#if data.rankings.length > 0}
    <section>
      <h2 class="mb-3 {TEXT.heading} text-ink">{i18n.t('Athletes')}</h2>
      <RankingList entries={data.rankings} {pagination} showFederation={false} {busy} />
    </section>
  {/if}

  {#if data.upcoming.length > 0}
    <section class="mt-8 sm:mt-12">
      <h2 class="mb-3 {TEXT.heading} text-ink">{i18n.t('Upcoming')}</h2>
      <CompetitionsTable competitions={data.upcoming} upcoming showFederation={false} />
    </section>
  {/if}
</div>
