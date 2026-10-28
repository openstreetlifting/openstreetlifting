<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { slugify } from '#lib/utils/slug.js';
  import type { PageData } from './$types';
  import {
    Card,
    Flag,
    FilterBar,
    Table,
    RisHeader,
    RisScore,
    RankingsEmpty,
    TABLE_CELL,
    TABLE_HEAD_CELL,
    FROZEN_CELL,
    FROZEN_HEAD_CELL,
    FROZEN_EDGE,
    FROZEN_RANK,
    ATHLETE_COLUMN,
    ATHLETE_CONTENT,
  } from '#lib/components/ui/index.js';
  import { InstagramIcon } from '#lib/components/icons/index.js';
  import Seo from '#lib/components/seo.svelte';
  import { resolve } from '$app/paths';
  import { page, navigating } from '$app/state';
  import { afterNavigate } from '$app/navigation';
  import { formatDate, countryName, formatWeight, formatAthleteName } from '#lib/utils/index.js';
  import { CELL, FIGURE, NO_VALUE, SORTED_COLUMN, TEXT_CELL } from '#lib/constants/table.js';
  import { RANKING_LIFTS, RANKING_SORTS, RANKING_GENDERS } from '#lib/constants/ranking.js';
  import { RankingsTable } from '#lib/state/rankings-table.svelte.js';
  import { rememberRankings } from '#lib/state/rankings-return.svelte.js';
  import { slowNavigation } from '#lib/state/slow-navigation.svelte.js';
  import { FIELD } from '#lib/constants/typography.js';
  import { listingSeo, websiteLd } from '#lib/seo/index.js';

  const i18n = useI18n();

  let { data }: { data: PageData } = $props();

  const table = new RankingsTable({ basePath: '/', includeYear: true, initialUrl: page.url });

  const rankings = $derived(data.initialRankings);
  const pagination = $derived(data.pagination);
  const loading = slowNavigation(() => navigating.to?.url.pathname === page.url.pathname);
  const busy = $derived(loading.current);

  const lifts = RANKING_LIFTS;
  const sorts = RANKING_SORTS;
  const genders = RANKING_GENDERS;

  const seo = $derived(listingSeo(page.url));

  const focused = $derived(page.url.searchParams.get('athlete'));

  afterNavigate(({ type, shallow }) => {
    if (shallow) return;

    table.syncFromUrl(page.url, type);
    rememberRankings(page.url.search);
    if (!focused) return;
    requestAnimationFrame(() => {
      document.querySelector('[data-focused]')?.scrollIntoView({ block: 'center' });
    });
  });

  const sorted = (column: string) => (table.movementFilter === column ? SORTED_COLUMN : '');

  const activeFilters = $derived(
    [
      table.countryFilter,
      table.federationFilter,
      table.yearFilter,
      table.genderFilter,
      table.categoryFilter,
    ].filter(Boolean).length
  );
</script>

<Seo
  title={i18n.t('Streetlifting rankings and records')}
  description={i18n.t(
    'Global Streetlifting rankings from every competition in the archive: muscle up, pull up, dips and squat results, RIS scores and athlete records.'
  )}
  canonical={seo.canonical}
  noindex={seo.noindex}
  jsonLd={[websiteLd(i18n.locale)]}
/>

<div class="mx-auto max-w-page px-4 py-3 sm:px-6 sm:py-12">
  <h1 class="sr-only">{i18n.t('Streetlifting rankings')}</h1>

  <FilterBar
    bind:search={table.searchFilter}
    placeholder={i18n.t('Search an athlete')}
    onSearch={() => table.handleFilterChange()}
    activeCount={activeFilters}
    onClear={() => table.clearFilters()}
    clearable={table.narrowed}
  >
    <select
      bind:value={table.countryFilter}
      onchange={() => table.handleFilterChange()}
      class="w-full {FIELD} px-3 py-2 sm:w-auto"
    >
      <option value={null}>{i18n.t('All Countries')}</option>
      {#each data.countries as countryOption (countryOption)}
        <option value={countryOption}>{countryName(countryOption, i18n.locale)}</option>
      {/each}
    </select>
    <select
      bind:value={table.federationFilter}
      onchange={() => table.handleFilterChange()}
      class="w-full {FIELD} px-3 py-2 sm:w-auto"
    >
      <option value={null}>{i18n.t('All Federations')}</option>
      {#each data.federations as federationOption (federationOption)}
        <option value={federationOption}>{federationOption}</option>
      {/each}
    </select>
    <select
      bind:value={table.yearFilter}
      onchange={() => table.handleFilterChange()}
      class="w-full {FIELD} px-3 py-2 sm:w-auto"
    >
      <option value={null}>{i18n.t('All Years')}</option>
      {#each data.years as yearOption (yearOption)}
        <option value={yearOption}>{yearOption}</option>
      {/each}
    </select>
    <select
      bind:value={table.genderFilter}
      onchange={() => {
        table.categoryFilter = null;
        table.handleFilterChange();
      }}
      class="w-full {FIELD} px-3 py-2 sm:w-auto"
    >
      {#each genders as gender (gender.label)}
        <option value={gender.value}>{i18n.t(gender.label)}</option>
      {/each}
    </select>
    <select
      bind:value={table.categoryFilter}
      onchange={() => table.handleFilterChange()}
      class="w-full {FIELD} px-3 py-2 sm:w-auto"
    >
      <option value={null}>{i18n.t('All Classes')}</option>
      {#each data.classes as classOption (classOption)}
        <option value={classOption}>{classOption}</option>
      {/each}
    </select>

    <div class="flex w-full items-center gap-2 sm:w-auto">
      <label for="sort-by" class="text-sm text-muted">{i18n.t('Sort by')}</label>
      <select
        id="sort-by"
        bind:value={table.movementFilter}
        onchange={() => table.handleFilterChange()}
        class="flex-1 {FIELD} px-3 py-2 sm:flex-none"
      >
        {#each sorts as sort (sort.value)}
          <option value={sort.value}>{i18n.t(sort.label)}</option>
        {/each}
      </select>
    </div>
  </FilterBar>

  {#if data.error}
    <Card class="max-w-summary p-8">
      <div class="text-center">
        <p class="text-danger">{i18n.t('Failed to load rankings')}</p>
      </div>
    </Card>
  {:else if rankings.length === 0 && !busy}
    <RankingsEmpty
      canReset={table.narrowed || pagination.page > 1}
      resetHref={i18n.href(resolve(''))}
    />
  {:else}
    <Table
      rows={rankings}
      itemName="athlete"
      {busy}
      pagination={{
        ...pagination,
        pageHref: (target) => table.pageHref(target),
        replaceState: true,
      }}
    >
      {#snippet head()}
        <th class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_RANK} {FROZEN_EDGE} text-secondary"
          >{i18n.t('Rank')}</th
        >
        <th class="{TABLE_HEAD_CELL} {ATHLETE_COLUMN} text-secondary">{i18n.t('Athlete')}</th>
        <th class="{TABLE_HEAD_CELL} text-secondary {sorted('total')}">{i18n.t('Total')}</th>
        <th class="{TABLE_HEAD_CELL} text-secondary {sorted('ris')}">
          <RisHeader />
        </th>
        {#each lifts as lift (lift.value)}
          <th class="{TABLE_HEAD_CELL} text-secondary {sorted(lift.value)}">
            {i18n.t(lift.label)}
          </th>
        {/each}
        <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Competition')}</th>
        <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Federation')}</th>
        <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Date')}</th>
        <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Sex')}</th>
        <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Class')}</th>
      {/snippet}

      {#snippet body(rows)}
        {#each rows as entry (entry.rank + entry.athlete.athlete_id)}
          <tr
            class="transition-colors"
            data-focused={entry.athlete.slug === focused ? '' : undefined}
          >
            <td class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_RANK} {FROZEN_EDGE} {CELL.identity}">
              {entry.rank}
            </td>
            <td class="{TABLE_CELL} {ATHLETE_COLUMN} {CELL.identity}">
              <span class="flex items-center gap-1.5 {ATHLETE_CONTENT}">
                <span class="flex min-w-0 items-center gap-2.5">
                  <Flag
                    link
                    countryCode={entry.athlete.country}
                    class="shrink-0 [--flag-height:1.25em]"
                  />
                  <a
                    href={i18n.href(resolve(`athletes/${entry.athlete.slug}`))}
                    class="truncate underline hover:text-secondary"
                  >
                    {formatAthleteName(entry.athlete)}
                  </a>
                </span>
                {#if entry.athlete.instagram_handle}
                  <a
                    href={`https://www.instagram.com/${entry.athlete.instagram_handle}`}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-flex shrink-0 items-center text-ink transition-colors hover:text-secondary"
                    aria-label={i18n.t('{athlete} on Instagram', {
                      athlete: formatAthleteName(entry.athlete),
                    })}
                    title="@{entry.athlete.instagram_handle}"
                  >
                    <InstagramIcon class="h-4 w-4" />
                  </a>
                {/if}
              </span>
            </td>
            <td class="{TABLE_CELL} {CELL.counted} {FIGURE}"
              >{formatWeight(entry.total, i18n.locale)}</td
            >
            <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">
              <RisScore value={entry.ris} source={entry.ris_source} />
            </td>
            <td class="{TABLE_CELL} {CELL.counted} {FIGURE}"
              >{formatWeight(entry.muscleup, i18n.locale)}</td
            >
            <td class="{TABLE_CELL} {CELL.counted} {FIGURE}"
              >{formatWeight(entry.pullup, i18n.locale)}</td
            >
            <td class="{TABLE_CELL} {CELL.counted} {FIGURE}"
              >{formatWeight(entry.dips, i18n.locale)}</td
            >
            <td class="{TABLE_CELL} {CELL.counted} {FIGURE}"
              >{formatWeight(entry.squat, i18n.locale)}</td
            >
            <td class="{TABLE_CELL} {CELL.data}">
              <a
                href={i18n.href(resolve(`competitions/${entry.competition.slug}`))}
                class="{TEXT_CELL.competition} underline hover:text-secondary"
              >
                {entry.competition.name}
              </a>
            </td>
            <td class="{TABLE_CELL} {CELL.data}" title={entry.federation.name}>
              <a
                href={i18n.href(
                  resolve('/federations/[slug]', { slug: slugify(entry.federation.name) })
                )}
                class="{TEXT_CELL.federation} underline hover:text-ink"
              >
                {entry.federation.abbreviation || entry.federation.name}
              </a>
            </td>
            <td class="{TABLE_CELL} {CELL.data} whitespace-nowrap">
              {formatDate(entry.competition.date, i18n.locale)}
            </td>
            <td class="{TABLE_CELL} {CELL.data}">{entry.athlete.gender}</td>
            <td class="{TABLE_CELL} {CELL.data}">{entry.category || NO_VALUE}</td>
          </tr>
        {/each}
      {/snippet}
    </Table>
  {/if}
</div>
