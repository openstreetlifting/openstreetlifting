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
  title="Streetlifting federations"
  description="{federations.length} streetlifting federations and organisers in the OpenStreetlifting archive, with {competitionCount} competitions of muscle up, pull up, dips and squat results."
  canonical={absolute('/federations')}
  jsonLd={[
    breadcrumbLd([
      { name: 'Rankings', path: '/' },
      { name: 'Federations', path: '/federations' },
    ]),
  ]}
/>

<div class="mx-auto max-w-page px-4 py-4 sm:px-6 sm:py-12">
  <Breadcrumb items={[{ label: 'Rankings', href: rankingsHref() }, { label: 'Federations' }]} />

  <h1 class="mb-6 {TEXT.title} text-ink sm:mb-10">Federations</h1>

  <Table>
    {#snippet head()}
      <th class="{TABLE_HEAD_CELL} text-secondary">Federation</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">Country</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">Competitions</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">Entries</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">Upcoming</th>
      <th class="{TABLE_HEAD_CELL} text-secondary">Active</th>
    {/snippet}

    {#snippet body()}
      {#each federations as federation (federation.slug)}
        <tr class="transition-colors">
          <td class="{TABLE_CELL} {CELL.identity}">
            <a
              href={resolve(federationPath(federation.name))}
              class="underline hover:text-secondary"
            >
              {federation.name}
            </a>
          </td>
          <td class="{TABLE_CELL} {CELL.data}">
            {#if federation.country}
              <a
                href={resolve(countryPath(federation.country))}
                class="flex items-center gap-2 hover:text-ink"
              >
                <Flag countryCode={federation.country} class="shrink-0 [--flag-height:1.25em]" />
                <span class="underline">{countryName(federation.country)}</span>
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
