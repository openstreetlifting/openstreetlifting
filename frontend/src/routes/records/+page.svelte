<script lang="ts">
  import { liftTone } from '#lib/constants/lift.js';
  import { LAYOUT } from '#lib/constants/layout.js';
  import { CELL, EDGE_TO_EDGE, FIGURE } from '#lib/constants/table.js';
  import { useI18n } from '#lib/i18n/index.js';
  import { useWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import {
    Button,
    Callout,
    Flag,
    SegmentedControl,
    SelectField,
    TABLE_CELL,
    TABLE_HEAD_CELL,
    Text,
  } from '#lib/components/ui/index.js';
  import { CloseIcon } from '#lib/components/icons/index.js';
  import Seo from '#lib/components/seo.svelte';
  import { absolute } from '#lib/seo/index.js';
  import { countryName, formatAthleteName, formatDate } from '#lib/utils/index.js';
  import { resolve } from '$app/paths';
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import { untrack } from 'svelte';
  import { UrlFilters } from '#lib/state/url-filters.svelte.js';
  import { RECORD_FILTERS } from '#lib/constants/filters.js';
  import { SvelteURL } from 'svelte/reactivity';
  import type { RecordGender, RecordGroup, RecordMovement } from '#lib/types/records.js';
  import type { PageData } from './$types';

  const FILTER = 'w-full min-w-0 sm:w-56';
  const MOVEMENTS: RecordMovement[] = ['total', 'muscleup', 'pullup', 'dips', 'squat'];
  const GENDERS: { value: RecordGender; label: 'Women' | 'Men' }[] = [
    { value: 'M', label: 'Men' },
    { value: 'F', label: 'Women' },
  ];

  const i18n = useI18n();
  const weights = useWeightUnit();

  let { data }: { data: PageData } = $props();

  const filters = new UrlFilters(
    RECORD_FILTERS,
    untrack(() => data.filters)
  );
  $effect.pre(() => filters.sync(data.filters));

  const sections = $derived(
    MOVEMENTS.map((movement) => ({
      movement,
      groups: data.groups.filter((group) => group.movement === movement),
    }))
  );

  const selectedSeries = $derived(
    data.facets.series.find((option) => option.series === data.filters.series)
  );
  const staleSeries = $derived(selectedSeries ? null : filters.values.series);
  const organiser = $derived(
    data.filters.series
      ? selectedSeries
        ? `${selectedSeries.federation} · ${selectedSeries.name}`
        : data.filters.series
      : data.filters.federation
  );

  function classLabel(group: RecordGroup): string {
    if (!group.weight_class) return i18n.t('All Classes');
    return weights.category(`${group.weight_class.slice(1)}kg`, i18n.locale);
  }

  function apply(gender: RecordGender = data.gender) {
    const url = new SvelteURL(page.url.href);
    url.search = '';
    if (gender === 'F') url.searchParams.set('gender', 'F');
    filters.writeTo(url.searchParams);
    return goto(i18n.href(`${url.pathname}${url.search}`), { reset: false });
  }
</script>

<Seo
  title="OpenStreetlifting records"
  description="The heaviest total, muscle-up, pull-up, dips and squat in each weight class, from every competition in the OpenStreetlifting archive."
  canonical={absolute('/records')}
/>

<div class={LAYOUT.page}>
  <Text as="h1" variant="title" class="mb-section text-ink">
    {#if data.filters.country}
      <span class="inline-flex items-center gap-3">
        <Flag countryCode={data.filters.country} class="[--flag-height:0.8em]" />
        {i18n.t('{country} records', { country: countryName(data.filters.country, i18n.locale) })}
      </span>
    {:else}
      {i18n.t('OpenStreetlifting records')}
    {/if}
  </Text>

  <div
    class="{EDGE_TO_EDGE} mb-section flex flex-wrap items-center gap-2 rounded-panel border border-stroke-strong bg-surface px-3 py-1.5 sm:gap-3"
  >
    <SegmentedControl
      items={GENDERS}
      key={(gender) => gender.value}
      selected={(gender) => gender.value === data.gender}
      onselect={(gender) => apply(gender.value)}
      label={i18n.t('Sex')}
    >
      {#snippet children(gender)}{i18n.t(gender.label)}{/snippet}
    </SegmentedControl>
    <SelectField
      label={i18n.t('Country')}
      class={FILTER}
      bind:value={filters.values.country}
      onchange={() => apply()}
    >
      <option value={null}>{i18n.t('All Countries')}</option>
      {#each data.facets.countries as code (code)}
        <option value={code}>{countryName(code, i18n.locale)}</option>
      {/each}
    </SelectField>
    <SelectField
      label={i18n.t('Federation')}
      class={FILTER}
      bind:value={filters.values.federation}
      onchange={() => apply()}
    >
      <option value={null}>{i18n.t('All Federations')}</option>
      {#each data.facets.federations as name (name)}
        <option value={name}>{name}</option>
      {/each}
    </SelectField>
    <SelectField
      label={i18n.t('Series')}
      class={FILTER}
      bind:value={filters.values.series}
      onchange={() => apply()}
    >
      <option value={null}>{i18n.t('All Series')}</option>
      {#each data.facets.series as option (option.series)}
        <option value={option.series}>{option.federation} · {option.name}</option>
      {/each}
      {#if staleSeries}<option value={staleSeries}>{staleSeries}</option>{/if}
    </SelectField>
    <SelectField
      label={i18n.t('Year')}
      class={FILTER}
      bind:value={filters.values.year}
      onchange={() => apply()}
    >
      <option value={null}>{i18n.t('All Years')}</option>
      {#each data.facets.years as option (option)}
        <option value={option}>{option}</option>
      {/each}
    </SelectField>
    {#if filters.activeCount > 0}
      <Button
        variant="outline"
        size="control"
        type="button"
        onclick={() => {
          filters.clear();
          apply();
        }}
        class="gap-1.5 text-meta text-secondary hover:text-ink"
      >
        <CloseIcon class="h-3 w-3" />
        {i18n.t('Clear')}
      </Button>
    {/if}
  </div>

  {#if data.facets.custom_weight_classes && organiser}
    <Callout title={i18n.t('Note')} class="mb-section">
      {i18n.t(
        'Records are grouped by the weight classes of {organiser}, which differ from the standard classes.',
        { organiser }
      )}
    </Callout>
  {/if}

  {#if data.groups.length === 0}
    <p class="border-y border-stroke py-8 text-body text-secondary">
      {i18n.t('No records for this selection yet.')}
    </p>
  {:else}
    <div class="grid gap-section xl:grid-cols-2">
      {#each sections as section (section.movement)}
        {#if section.groups.length}
          {@const fullWidth = section.movement === 'total'}
          <section
            class="min-w-0 {fullWidth ? 'xl:col-span-2' : ''}"
            aria-labelledby="records-{section.movement}"
          >
            <Text as="h2" id="records-{section.movement}" variant="heading" class="mb-3 text-ink">
              {i18n.movement(section.movement)}
            </Text>
            <div class="osl-table overflow-x-auto rounded-panel border border-stroke-strong">
              <table class="w-full text-meta leading-4 whitespace-nowrap">
                <caption class="sr-only">
                  {i18n.t('{movement} records', { movement: i18n.movement(section.movement) })}
                </caption>
                <thead>
                  <tr class="border-b border-stroke-strong bg-table-head">
                    <th scope="col" class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Class')}</th>
                    <th scope="col" class="{TABLE_HEAD_CELL} w-8 text-secondary">#</th>
                    <th scope="col" class="{TABLE_HEAD_CELL} text-secondary">
                      {i18n.t('Athlete')}
                    </th>
                    <th scope="col" class="{TABLE_HEAD_CELL} text-right text-secondary">
                      {weights.unit}
                    </th>
                    <th scope="col" class="{TABLE_HEAD_CELL} text-secondary">
                      {i18n.t('Competition')}
                    </th>
                    <th scope="col" class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Date')}</th>
                  </tr>
                </thead>
                {#each section.groups as group (group.weight_class ?? 'open')}
                  <tbody
                    class={group.weight_class
                      ? 'border-b border-stroke last:border-b-0'
                      : 'border-b-2 border-stroke-strong bg-surface'}
                  >
                    {#each group.entries as entry (entry.rank)}
                      <tr>
                        {#if entry.rank === 1}
                          <th
                            scope="rowgroup"
                            rowspan={group.entries.length}
                            class="{TABLE_CELL} w-px align-top text-left font-mono font-medium whitespace-normal text-ink"
                          >
                            {classLabel(group)}
                          </th>
                        {/if}
                        <td class="{TABLE_CELL} {FIGURE} text-muted">{entry.rank}</td>
                        <td class="{TABLE_CELL} {CELL.identity}">
                          <a
                            href={i18n.href(
                              resolve('/athletes/[slug]', { slug: entry.athlete.slug })
                            )}
                            class="flex max-w-[11rem] items-center gap-2 hover:text-secondary sm:max-w-[16rem] {fullWidth
                              ? 'xl:max-w-none'
                              : 'xl:max-w-[12rem] 2xl:max-w-[16rem]'}"
                          >
                            {#if entry.athlete.country}
                              <Flag
                                countryCode={entry.athlete.country}
                                class="shrink-0 [--flag-height:1.25em]"
                              />
                            {/if}
                            <span class="truncate underline">
                              {formatAthleteName(entry.athlete)}
                            </span>
                          </a>
                        </td>
                        <td
                          class="{TABLE_CELL} {FIGURE} text-right {entry.rank === 1
                            ? CELL.counted
                            : CELL.data}"
                        >
                          <span style:color={liftTone(section.movement)}>
                            {weights.format(entry.weight, i18n.locale)}
                          </span>
                        </td>
                        <td class="{TABLE_CELL} {CELL.data}">
                          <a
                            href={i18n.href(
                              resolve('/competitions/[slug]', { slug: entry.competition.slug })
                            )}
                            class="block max-w-[9rem] truncate underline hover:text-ink sm:max-w-[14rem] {fullWidth
                              ? 'xl:max-w-none'
                              : 'xl:max-w-[9rem] 2xl:max-w-[12rem]'}"
                          >
                            {entry.competition.name}
                          </a>
                        </td>
                        <td class="{TABLE_CELL} {CELL.data} {FIGURE}">
                          {formatDate(entry.date, i18n.locale)}
                        </td>
                      </tr>
                    {/each}
                  </tbody>
                {/each}
              </table>
            </div>
          </section>
        {/if}
      {/each}
    </div>
  {/if}
</div>
