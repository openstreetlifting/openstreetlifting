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

  const countries = $derived(data.countries);
  const athleteCount = $derived(countries.reduce((sum, country) => sum + country.athletes, 0));
</script>

<Seo
  title={i18n.t('Streetlifting by country')}
  description={i18n.t(
    'Streetlifting results and rankings for {countries} countries, covering {athletes} athletes in the OpenStreetlifting archive.',
    { countries: i18n.number(countries.length, 0), athletes: i18n.number(athleteCount, 0) }
  )}
  canonical={absolute('/countries')}
  jsonLd={[
    breadcrumbLd([
      { name: i18n.t('Rankings'), path: '/' },
      { name: i18n.t('Countries'), path: '/countries' },
    ]),
  ]}
/>

<div class="mx-auto max-w-page px-4 py-4 sm:px-6 sm:py-12">
  <Breadcrumb
    items={[{ label: i18n.t('Rankings'), href: rankingsHref() }, { label: i18n.t('Countries') }]}
  />

  <h1 class="mb-6 {TEXT.title} text-ink sm:mb-10">{i18n.t('Countries')}</h1>

  <Table rows={countries} itemName="country" pageParam="page">
    {#snippet head()}
      <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Country')}</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Athletes')}</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Competitions')}</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Upcoming')}</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Federations')}</th>
    {/snippet}

    {#snippet body(rows)}
      {#each rows as country (country.code)}
        <tr class="transition-colors">
          <td class="{TABLE_CELL} {CELL.identity}">
            <a
              href={resolve('/countries/[code]', { code: country.code.toLowerCase() })}
              class="flex items-center gap-2 hover:text-secondary"
            >
              <Flag countryCode={country.code} class="shrink-0 [--flag-height:1.25em]" />
              <span class="underline">{countryName(country.code, i18n.locale)}</span>
            </a>
          </td>
          <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">{country.athletes}</td>
          <td class="{TABLE_CELL} {CELL.data} {FIGURE}">{country.competitions}</td>
          <td class="{TABLE_CELL} {CELL.data} {FIGURE}">{country.upcoming}</td>
          <td class="{TABLE_CELL} {CELL.data}">
            {#each country.federations as name, index (name)}
              <span class="whitespace-nowrap"
                >{index > 0 ? ', ' : ''}<a
                  href={resolve('/federations/[slug]', { slug: slugify(name) })}
                  class="underline hover:text-ink">{name}</a
                ></span
              >
            {:else}
              {NO_VALUE}
            {/each}
          </td>
        </tr>
      {/each}
    {/snippet}
  </Table>
</div>
