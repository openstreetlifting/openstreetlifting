<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { slugify } from '#lib/utils/slug.js';
  import type { PageData } from './$types';
  import { Breadcrumb, Flag } from '#lib/components/ui/index.js';
  import CompetitionsTable from '#lib/components/competitions-table.svelte';
  import RankingList from '#lib/components/ranking-list.svelte';
  import Seo from '#lib/components/seo.svelte';
  import { resolve } from '$app/paths';
  import { page as currentPage, navigating } from '$app/state';
  import { rankingsHref } from '#lib/state/rankings-return.svelte.js';
  import { slowNavigation } from '#lib/state/slow-navigation.svelte.js';
  import { breadcrumbLd, listingSeo } from '#lib/seo/index.js';
  import { countryName, countryPath } from '#lib/utils/index.js';
  import { TEXT } from '#lib/constants/typography.js';

  const i18n = useI18n();

  let { data }: { data: PageData } = $props();

  const name = $derived(countryName(data.code, i18n.locale));
  const pagination = $derived(data.pagination);
  const firstPage = $derived(pagination.page <= 1);
  const loading = slowNavigation(() => navigating.to?.url.pathname === currentPage.url.pathname);
  const busy = $derived(loading.current);

  const seo = $derived(listingSeo(currentPage.url));
  const path = $derived(countryPath(data.code));

  const title = $derived(
    firstPage
      ? i18n.t('Streetlifting in {country}: results and rankings', { country: name })
      : i18n.t('Streetlifting in {country}: rankings, page {page}', {
          country: name,
          page: pagination.page,
        })
  );
  const description = $derived(
    i18n.t(
      'Streetlifting in {country}: {count} ranked {athletes}. Muscle-up, pull-up, dips and squat results.',
      {
        country: name,
        count: i18n.number(pagination.total_items, 0),
        athletes: i18n.count('athlete', pagination.total_items),
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
    breadcrumbLd(
      [
        { name: i18n.t('Rankings'), path: '/' },
        { name, path },
      ],
      i18n.locale
    ),
  ]}
/>

<div class="mx-auto max-w-page px-4 py-4 sm:px-6 sm:py-12">
  <Breadcrumb items={[{ label: i18n.t('Rankings'), href: rankingsHref() }, { label: name }]} />

  <header class="mb-6 border-b border-stroke pb-5">
    <h1 class="{TEXT.title} flex min-w-0 items-start gap-3 text-ink">
      <Flag countryCode={data.code} class="mt-[0.2em] shrink-0 [--flag-height:0.8em]" />
      <span class="min-w-0 break-words">{name}</span>
    </h1>
    {#if data.federations.length > 0}
      <dl
        class="mt-4 grid grid-cols-[6rem_minmax(0,1fr)] gap-x-3 gap-y-1.5 text-sm sm:grid-cols-[7rem_minmax(0,1fr)]"
      >
        <dt class="text-muted">
          {data.federations.length === 1 ? i18n.t('Federation') : i18n.t('Federations')}
        </dt>
        <dd>
          <ul class="flex flex-wrap gap-x-4 gap-y-1.5">
            {#each data.federations as federation (federation)}
              <li class="min-w-0">
                <a
                  href={i18n.href(resolve('/federations/[slug]', { slug: slugify(federation) }))}
                  class="break-words text-ink underline decoration-stroke-strong underline-offset-2 hover:text-secondary"
                  >{federation}</a
                >
              </li>
            {/each}
          </ul>
        </dd>
      </dl>
    {/if}
  </header>

  {#if data.results.length > 0}
    <section class="mb-8 sm:mb-12">
      <h2 class="mb-3 {TEXT.heading} text-ink">{i18n.t('Competitions')}</h2>
      <CompetitionsTable competitions={data.results} />
    </section>
  {/if}

  {#if data.rankings.length > 0}
    <section>
      <h2 class="mb-3 {TEXT.heading} text-ink">{i18n.t('Athletes')}</h2>
      <RankingList entries={data.rankings} {pagination} {busy} />
    </section>
  {/if}

  {#if data.upcoming.length > 0}
    <section class="mt-8 sm:mt-12">
      <h2 class="mb-3 {TEXT.heading} text-ink">{i18n.t('Upcoming')}</h2>
      <CompetitionsTable competitions={data.upcoming} upcoming />
    </section>
  {/if}
</div>
