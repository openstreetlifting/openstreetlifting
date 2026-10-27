<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { slugify } from '#lib/utils/slug.js';
  import type { PageData } from './$types';
  import {
    Breadcrumb,
    Flag,
    Table,
    TABLE_CELL,
    TABLE_HEAD_CELL,
  } from '#lib/components/ui/index.js';
  import Seo from '#lib/components/seo.svelte';
  import { resolve } from '$app/paths';
  import { rankingsHref } from '#lib/state/rankings-return.svelte.js';
  import { absolute, breadcrumbLd } from '#lib/seo/index.js';
  import { countryName } from '#lib/utils/index.js';
  import { CELL, FIGURE, NO_VALUE } from '#lib/constants/table.js';
  import { TEXT } from '#lib/constants/typography.js';

  const i18n = useI18n();

  let { data }: { data: PageData } = $props();

  const federations = $derived(data.federations);
  const competitionCount = $derived(
    federations.reduce((sum, federation) => sum + federation.competitions, 0)
  );

  function activeYears(first: number | null, last: number | null): string {
    if (first === null || last === null) return NO_VALUE;
    return first === last ? String(first) : `${first} - ${last}`;
  }
</script>

<Seo
  title={i18n.t('Streetlifting federations')}
  description={i18n.t(
    '{federations} Streetlifting federations and organisers in the OpenStreetlifting archive, with {competitions} competitions of muscle up, pull up, dips and squat results.',
    {
      federations: i18n.number(federations.length, 0),
      competitions: i18n.number(competitionCount, 0),
    }
  )}
  canonical={absolute('/federations')}
  jsonLd={[
    breadcrumbLd([
      { name: i18n.t('Rankings'), path: '/' },
      { name: i18n.t('Competitions'), path: '/competitions' },
      { name: i18n.t('Federations'), path: '/federations' },
    ]),
  ]}
/>

<div class="mx-auto max-w-page px-4 py-4 sm:px-6 sm:py-12">
  <Breadcrumb
    items={[
      { label: i18n.t('Rankings'), href: rankingsHref() },
      { label: i18n.t('Competitions'), href: '/competitions' },
      { label: i18n.t('Federations') },
    ]}
  />

  <h1 class="mb-6 {TEXT.title} text-ink sm:mb-10">{i18n.t('Federations')}</h1>

  <Table rows={federations} itemName="federation" pageParam="page">
    {#snippet head()}
      <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Federation')}</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Country')}</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Competitions')}</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Entries')}</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Upcoming')}</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Active')}</th>
    {/snippet}

    {#snippet body(rows)}
      {#each rows as federation (federation.slug)}
        <tr class="transition-colors">
          <td class="{TABLE_CELL} {CELL.identity}">
            <a
              href={resolve('/federations/[slug]', { slug: slugify(federation.name) })}
              class="underline hover:text-secondary"
            >
              {federation.name}
            </a>
          </td>
          <td class="{TABLE_CELL} {CELL.data}">
            {#if federation.country}
              <a
                href={resolve('/countries/[code]', { code: federation.country.toLowerCase() })}
                class="flex items-center gap-2 hover:text-ink"
              >
                <Flag countryCode={federation.country} class="shrink-0 [--flag-height:1.25em]" />
                <span class="underline">{countryName(federation.country, i18n.locale)}</span>
              </a>
            {:else}
              {NO_VALUE}
            {/if}
          </td>
          <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">{federation.competitions}</td>
          <td class="{TABLE_CELL} {CELL.data} {FIGURE}">{federation.entries}</td>
          <td class="{TABLE_CELL} {CELL.data} {FIGURE}">{federation.upcoming}</td>
          <td class="{TABLE_CELL} {CELL.data} whitespace-nowrap">
            {activeYears(federation.firstYear, federation.lastYear)}
          </td>
        </tr>
      {/each}
    {/snippet}
  </Table>
</div>
