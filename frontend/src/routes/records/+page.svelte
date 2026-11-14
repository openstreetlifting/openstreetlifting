<script lang="ts">
  import { LAYOUT } from '#lib/constants/layout.js';
  import { CELL, EDGE_TO_EDGE, FIGURE } from '#lib/constants/table.js';
  import { useI18n } from '#lib/i18n/index.js';
  import { useWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import {
    Button,
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
  import { SvelteURL } from 'svelte/reactivity';
  import type { RecordGender, RecordGroup, RecordMovement } from '#lib/types/records.js';
  import type { PageData } from './$types';

  const FILTER = 'w-full min-w-0 sm:w-56';
  const MOVEMENTS: RecordMovement[] = ['muscleup', 'pullup', 'dips', 'squat'];
  const GENDERS: { value: RecordGender; label: 'Women' | 'Men' }[] = [
    { value: 'M', label: 'Men' },
    { value: 'F', label: 'Women' },
  ];

  const i18n = useI18n();
  const weights = useWeightUnit();

  let { data }: { data: PageData } = $props();

  let country = $derived(data.country);
  let federation = $derived(data.federation);
  let year = $derived(data.year);

  const narrowed = $derived(Boolean(data.country || data.federation || data.year));

  const sections = $derived(
    MOVEMENTS.map((movement) => ({
      movement,
      groups: data.groups.filter((group) => group.movement === movement),
    }))
  );

  function classLabel(group: RecordGroup): string {
    if (!group.weight_class) return i18n.t('All Classes');
    return weights.category(`${group.weight_class.slice(1)}kg`, i18n.locale);
  }

  type Scope = { country: string | null; federation: string | null; year: number | null };

  function apply(gender: RecordGender, scope: Scope) {
    const url = new SvelteURL(page.url.href);
    url.search = '';
    if (gender === 'F') url.searchParams.set('gender', 'F');
    if (scope.country) url.searchParams.set('country', scope.country);
    if (scope.federation) url.searchParams.set('federation', scope.federation);
    if (scope.year) url.searchParams.set('year', String(scope.year));
    return goto(i18n.href(`${url.pathname}${url.search}`), { reset: false });
  }

  const update = () => apply(data.gender, { country, federation, year });
</script>

<Seo
  title="OpenStreetlifting records"
  description="The heaviest muscle-up, pull-up, dips and squat in each weight class, from every competition in the OpenStreetlifting archive."
  canonical={absolute('/records')}
/>

<div class={LAYOUT.page}>
  <Text as="h1" variant="title" class="mb-section text-ink">
    {#if data.country}
      <span class="inline-flex items-center gap-3">
        <Flag countryCode={data.country} class="[--flag-height:0.8em]" />
        {i18n.t('{country} records', { country: countryName(data.country, i18n.locale) })}
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
      onselect={(gender) =>
        apply(gender.value, {
          country: data.country,
          federation: data.federation,
          year: data.year,
        })}
      label={i18n.t('Sex')}
    >
      {#snippet children(gender)}{i18n.t(gender.label)}{/snippet}
    </SegmentedControl>
    <SelectField label={i18n.t('Country')} class={FILTER} bind:value={country} onchange={update}>
      <option value={null}>{i18n.t('All Countries')}</option>
      {#each data.countries as code (code)}
        <option value={code}>{countryName(code, i18n.locale)}</option>
      {/each}
    </SelectField>
    <SelectField
      label={i18n.t('Federation')}
      class={FILTER}
      bind:value={federation}
      onchange={update}
    >
      <option value={null}>{i18n.t('All Federations')}</option>
      {#each data.federations as name (name)}
        <option value={name}>{name}</option>
      {/each}
    </SelectField>
    <SelectField label={i18n.t('Year')} class={FILTER} bind:value={year} onchange={update}>
      <option value={null}>{i18n.t('All Years')}</option>
      {#each data.years as option (option)}
        <option value={option}>{option}</option>
      {/each}
    </SelectField>
    {#if narrowed}
      <Button
        variant="outline"
        size="control"
        type="button"
        onclick={() => apply(data.gender, { country: null, federation: null, year: null })}
        class="gap-1.5 text-meta text-secondary hover:text-ink"
      >
        <CloseIcon class="h-3 w-3" />
        {i18n.t('Clear')}
      </Button>
    {/if}
  </div>

  {#if data.groups.length === 0}
    <p class="border-y border-stroke py-8 text-body text-secondary">
      {i18n.t('No records for this selection yet.')}
    </p>
  {:else}
    <div class="grid gap-section xl:grid-cols-2">
      {#each sections as section (section.movement)}
        {#if section.groups.length}
          <section class="min-w-0" aria-labelledby="records-{section.movement}">
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
                    <th scope="col" class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Athlete')}</th
                    >
                    <th scope="col" class="{TABLE_HEAD_CELL} text-right text-secondary"
                      >{weights.unit}</th
                    >
                    <th scope="col" class="{TABLE_HEAD_CELL} text-secondary"
                      >{i18n.t('Competition')}</th
                    >
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
                            class="flex max-w-[11rem] items-center gap-2 hover:text-secondary sm:max-w-[16rem] xl:max-w-[12rem] 2xl:max-w-[16rem]"
                          >
                            {#if entry.athlete.country}
                              <Flag
                                countryCode={entry.athlete.country}
                                class="shrink-0 [--flag-height:1.25em]"
                              />
                            {/if}
                            <span class="truncate underline"
                              >{formatAthleteName(entry.athlete)}</span
                            >
                          </a>
                        </td>
                        <td
                          class="{TABLE_CELL} {FIGURE} text-right {entry.rank === 1
                            ? CELL.counted
                            : CELL.data}"
                        >
                          {weights.format(entry.weight, i18n.locale)}
                        </td>
                        <td class="{TABLE_CELL} {CELL.data}">
                          <a
                            href={i18n.href(
                              resolve('/competitions/[slug]', { slug: entry.competition.slug })
                            )}
                            class="block max-w-[9rem] truncate sm:max-w-[14rem] xl:max-w-[9rem] 2xl:max-w-[12rem] underline hover:text-ink"
                            >{entry.competition.name}</a
                          >
                        </td>
                        <td class="{TABLE_CELL} {CELL.data} {FIGURE}"
                          >{formatDate(entry.date, i18n.locale)}</td
                        >
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
