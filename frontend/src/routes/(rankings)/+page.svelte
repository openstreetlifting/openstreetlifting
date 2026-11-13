<script lang="ts">
  import { LAYOUT } from '#lib/constants/layout.js';
  import { useI18n } from '#lib/i18n/index.js';
  import { useWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import { slugify } from '#lib/utils/slug.js';
  import type { PageData } from './$types';
  import {
    ResultState,
    SelectField,
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
    FROZEN_RANK,
    FROZEN_ATHLETE,
    ATHLETE_COLUMN,
    ATHLETE_CONTENT,
    RankHeader,
  } from '#lib/components/ui/index.js';
  import { InstagramIcon } from '#lib/components/icons/index.js';
  import Seo from '#lib/components/seo.svelte';
  import { resolve } from '$app/paths';
  import { page, navigating } from '$app/state';
  import { afterNavigate, refreshAll } from '$app/navigation';
  import { formatDate, countryName, formatAthleteName } from '#lib/utils/index.js';
  import { CELL, FIGURE, NO_VALUE, SORTED_COLUMN, TEXT_CELL } from '#lib/constants/table.js';
  import { RANKING_LIFTS, RANKING_SORTS, RANKING_GENDERS } from '#lib/constants/ranking.js';
  import { RankingsTable } from '#lib/state/rankings-table.svelte.js';
  import { rememberRankings } from '#lib/state/rankings-return.svelte.js';
  import { slowNavigation } from '#lib/state/slow-navigation.svelte.js';
  import { listingSeo, websiteLd, SITE_NAME, SITE_DESCRIPTION } from '#lib/seo/index.js';

  const i18n = useI18n();
  const weights = useWeightUnit();

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
  title={SITE_NAME}
  description={SITE_DESCRIPTION}
  canonical={seo.canonical}
  noindex={seo.noindex}
  jsonLd={[websiteLd()]}
/>

<div class={LAYOUT.rankings}>
  <h1 class="sr-only">{i18n.t('Streetlifting rankings')}</h1>

  <section
    class="min-w-0 rounded-panel border border-stroke-strong"
    aria-label={i18n.t('Streetlifting rankings')}
  >
    <FilterBar
      embedded
      bind:search={table.searchFilter}
      placeholder={i18n.t('Search an athlete')}
      onSearch={() => table.handleFilterChange()}
      activeCount={activeFilters}
      onClear={() => table.clearFilters()}
      clearable={table.narrowed}
    >
      {#snippet primary()}
        <SelectField
          label={i18n.t('Sort by')}
          bind:value={table.movementFilter}
          onchange={() => table.handleFilterChange()}
        >
          {#each sorts as sort (sort.value)}
            <option value={sort.value}
              >{i18n.t('Sort by {sort}', { sort: i18n.t(sort.label) })}</option
            >
          {/each}
        </SelectField>
        <SelectField
          label={i18n.t('Sex')}
          bind:value={table.genderFilter}
          onchange={() => {
            table.categoryFilter = null;
            table.handleFilterChange();
          }}
        >
          {#each genders as gender (gender.label)}
            <option value={gender.value}>{i18n.t(gender.label)}</option>
          {/each}
        </SelectField>
      {/snippet}

      <SelectField
        label={i18n.t('Country')}
        bind:value={table.countryFilter}
        onchange={() => table.handleFilterChange()}
      >
        <option value={null}>{i18n.t('All Countries')}</option>
        {#each data.countries as countryOption (countryOption)}
          <option value={countryOption}>{countryName(countryOption, i18n.locale)}</option>
        {/each}
      </SelectField>
      <SelectField
        label={i18n.t('Federation')}
        bind:value={table.federationFilter}
        onchange={() => table.handleFilterChange()}
      >
        <option value={null}>{i18n.t('All Federations')}</option>
        {#each data.federations as federationOption (federationOption)}
          <option value={federationOption}>{federationOption}</option>
        {/each}
      </SelectField>
      <SelectField
        label={i18n.t('Year')}
        bind:value={table.yearFilter}
        onchange={() => table.handleFilterChange()}
      >
        <option value={null}>{i18n.t('All Years')}</option>
        {#each data.years as yearOption (yearOption)}
          <option value={yearOption}>{yearOption}</option>
        {/each}
      </SelectField>
      <SelectField
        label={i18n.t('Class')}
        bind:value={table.categoryFilter}
        onchange={() => table.handleFilterChange()}
      >
        <option value={null}>{i18n.t('All Classes')}</option>
        {#each data.classes as classOption (classOption)}
          <option value={classOption}>{weights.category(classOption, i18n.locale)}</option>
        {/each}
      </SelectField>
    </FilterBar>

    {#if data.error}
      <ResultState
        embedded
        kind="error"
        title={i18n.t('Couldn’t load rankings')}
        retry={refreshAll}
      />
    {:else if rankings.length === 0 && !busy}
      <RankingsEmpty
        embedded
        canReset={table.narrowed || pagination.page > 1}
        resetHref={i18n.href(resolve(''))}
      />
    {:else}
      <Table
        embedded
        frozenColumns="athlete"
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
          <th class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_RANK} text-secondary"
            ><RankHeader /></th
          >
          <th
            class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_ATHLETE} {ATHLETE_COLUMN} text-secondary"
            >{i18n.t('Athlete')}</th
          >
          <th class="{TABLE_HEAD_CELL} text-secondary {sorted('total')}"
            >{i18n.t('Total')} ({weights.unit})</th
          >
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
              <td class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_RANK} font-mono {CELL.identity}">
                {entry.rank}
              </td>
              <td
                class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_ATHLETE} {ATHLETE_COLUMN} {CELL.identity}"
              >
                <span class="flex items-center gap-1.5 {ATHLETE_CONTENT}">
                  <span class="flex min-w-0 items-center gap-2">
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
                >{weights.format(entry.total, i18n.locale)}</td
              >
              <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">
                <RisScore value={entry.ris} source={entry.ris_source} />
              </td>
              <td class="{TABLE_CELL} {CELL.counted} {FIGURE}"
                >{weights.format(entry.muscleup, i18n.locale)}</td
              >
              <td class="{TABLE_CELL} {CELL.counted} {FIGURE}"
                >{weights.format(entry.pullup, i18n.locale)}</td
              >
              <td class="{TABLE_CELL} {CELL.counted} {FIGURE}"
                >{weights.format(entry.dips, i18n.locale)}</td
              >
              <td class="{TABLE_CELL} {CELL.counted} {FIGURE}"
                >{weights.format(entry.squat, i18n.locale)}</td
              >
              <td class="{TABLE_CELL} {CELL.data}">
                <a
                  href={i18n.href(resolve(`competitions/${entry.competition.slug}`))}
                  class="{TEXT_CELL.resultCompetition} underline hover:text-secondary"
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
              <td class="{TABLE_CELL} {CELL.data}"
                >{weights.category(entry.category, i18n.locale) || NO_VALUE}</td
              >
            </tr>
          {/each}
        {/snippet}
      </Table>
    {/if}
  </section>
</div>
