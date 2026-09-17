<script lang="ts">
  import type { PageData } from './$types';
  import { Breadcrumb, Flag, Table, TABLE_CELL, TABLE_HEAD_CELL } from '$lib/components/ui';
  import Seo from '$lib/components/seo.svelte';
  import { resolve } from '$app/paths';
  import { rankingsHref } from '$lib/state/rankings-return.svelte';
  import { absolute, breadcrumbLd } from '$lib/seo';
  import { countryName, countryPath, federationPath } from '$lib/utils';
  import { CELL, FIGURE, NO_VALUE } from '$lib/constants/table';
  import { TEXT } from '$lib/constants/typography';

  let { data }: { data: PageData } = $props();

  const countries = $derived(data.countries);
  const athleteCount = $derived(countries.reduce((sum, country) => sum + country.athletes, 0));
</script>

<Seo
  title="Streetlifting by country"
  description="Streetlifting results and rankings for {countries.length} countries, covering {athleteCount} athletes in the OpenStreetlifting archive."
  canonical={absolute('/countries')}
  jsonLd={[
    breadcrumbLd([
      { name: 'Rankings', path: '/' },
      { name: 'Countries', path: '/countries' },
    ]),
  ]}
/>

<div class="mx-auto max-w-page px-4 py-4 sm:px-6 sm:py-12">
  <Breadcrumb items={[{ label: 'Rankings', href: rankingsHref() }, { label: 'Countries' }]} />

  <h1 class="mb-6 {TEXT.title} text-ink sm:mb-10">Countries</h1>

  <Table>
    {#snippet head()}
      <th class="{TABLE_HEAD_CELL} text-secondary">Country</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">Athletes</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">Competitions</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">Upcoming</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">Federations</th>
    {/snippet}

    {#snippet body()}
      {#each countries as country (country.code)}
        <tr class="transition-colors">
          <td class="{TABLE_CELL} {CELL.identity}">
            <a
              href={resolve(countryPath(country.code))}
              class="flex items-center gap-2 hover:text-secondary"
            >
              <Flag countryCode={country.code} class="shrink-0 [--flag-height:1.25em]" />
              <span class="underline">{countryName(country.code)}</span>
            </a>
          </td>
          <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">{country.athletes}</td>
          <td class="{TABLE_CELL} {CELL.data} {FIGURE}">{country.competitions}</td>
          <td class="{TABLE_CELL} {CELL.data} {FIGURE}">{country.upcoming}</td>
          <td class="{TABLE_CELL} {CELL.data}">
            {#each country.federations as name, index (name)}
              <span class="whitespace-nowrap"
                >{index > 0 ? ', ' : ''}<a
                  href={resolve(federationPath(name))}
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
