<script lang="ts">
  import { LAYOUT } from '#lib/constants/layout.js';
  import { useI18n } from '#lib/i18n/index.js';
  import { slugify } from '#lib/utils/slug.js';
  import type { PageData } from './$types';
  import { Flag, Table, TABLE_CELL, TABLE_HEAD_CELL, Text } from '#lib/components/ui/index.js';
  import Seo from '#lib/components/seo.svelte';
  import { resolve } from '$app/paths';
  import { absolute } from '#lib/seo/index.js';
  import { countryName } from '#lib/utils/index.js';
  import { CELL, FIGURE, NO_VALUE } from '#lib/constants/table.js';

  const i18n = useI18n();

  let { data }: { data: PageData } = $props();

  const countries = $derived(data.countries);
</script>

<Seo
  title="Streetlifting by country"
  description="Browse Streetlifting rankings by country."
  canonical={absolute('/countries')}
/>

<div class={LAYOUT.page}>
  <Text as="h1" variant="title" class="mb-section text-ink">{i18n.t('Countries')}</Text>

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
              href={i18n.href(resolve('/countries/[code]', { code: country.code.toLowerCase() }))}
              class="flex items-center gap-2 hover:text-secondary"
            >
              <Flag countryCode={country.code} class="shrink-0 [--flag-height:1.25em]" />
              <span class="underline">{countryName(country.code, i18n.locale)}</span>
            </a>
          </td>
          <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">{i18n.number(country.athletes, 0)}</td>
          <td class="{TABLE_CELL} {CELL.data} {FIGURE}">{i18n.number(country.competitions, 0)}</td>
          <td class="{TABLE_CELL} {CELL.data} {FIGURE}">{i18n.number(country.upcoming, 0)}</td>
          <td class="{TABLE_CELL} {CELL.data}">
            {#each country.federations as name, index (name)}
              <span class="whitespace-nowrap"
                >{index > 0 ? ', ' : ''}<a
                  href={i18n.href(resolve('/federations/[slug]', { slug: slugify(name) }))}
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
