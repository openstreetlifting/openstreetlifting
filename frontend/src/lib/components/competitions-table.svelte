<script lang="ts">
  import type { Competition } from '$lib/types/competition';
  import { Table, TABLE_CELL, TABLE_HEAD_CELL } from '$lib/components/ui';
  import { resolve } from '$app/paths';
  import { federationPath, formatCountdown, formatDate, formatLocation } from '$lib/utils';
  import { CELL, FIGURE, TEXT_CELL } from '$lib/constants/table';

  interface Props {
    competitions: Competition[];
    upcoming?: boolean;
    showFederation?: boolean;
    busy?: boolean;
  }

  let { competitions, upcoming = false, showFederation = true, busy = false }: Props = $props();

  function competitionDates(start: string | null, end: string | null): string {
    const from = formatDate(start);
    return end && end !== start ? `${from} - ${formatDate(end)}` : from;
  }
</script>

<Table {busy}>
  {#snippet head()}
    <th class="{TABLE_HEAD_CELL} text-secondary">Competition</th>
    <th class="{TABLE_HEAD_CELL} text-secondary">{upcoming ? 'When' : 'Lifters'}</th>
    <th class="{TABLE_HEAD_CELL} text-secondary">Date</th>
    <th class="{TABLE_HEAD_CELL} text-secondary">Location</th>
    {#if showFederation}
      <th class="{TABLE_HEAD_CELL} text-secondary">Federation</th>
    {/if}
  {/snippet}

  {#snippet body()}
    {#each competitions as competition (competition.slug)}
      <tr class="transition-colors">
        <td class="{TABLE_CELL} {CELL.identity}">
          <a
            href={resolve(`/competitions/${competition.slug}`)}
            class="{TEXT_CELL.competition} underline hover:text-secondary"
          >
            {competition.name}
          </a>
        </td>
        <td class="{TABLE_CELL} {CELL.data} whitespace-nowrap {upcoming ? '' : FIGURE}">
          {upcoming ? formatCountdown(competition.start_date) : (competition.lifter_count ?? 0)}
        </td>
        <td class="{TABLE_CELL} whitespace-nowrap text-secondary">
          {competitionDates(competition.start_date, competition.end_date)}
        </td>
        <td class="{TABLE_CELL} {CELL.data}">
          <span class={TEXT_CELL.location}>
            {formatLocation(competition.country, competition.region, competition.city)}
          </span>
        </td>
        {#if showFederation}
          <td class="{TABLE_CELL} {CELL.data}" title={competition.federation.name}>
            <a
              href={resolve(federationPath(competition.federation.name))}
              class="{TEXT_CELL.federation} underline hover:text-ink"
            >
              {competition.federation.abbreviation || competition.federation.name}
            </a>
          </td>
        {/if}
      </tr>
    {/each}
  {/snippet}
</Table>
