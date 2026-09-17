<script lang="ts">
  import type { RankingEntry } from '$lib/types/ranking';
  import {
    Flag,
    Table,
    RisHeader,
    RisScore,
    TABLE_CELL,
    TABLE_HEAD_CELL,
    FROZEN_CELL,
    FROZEN_HEAD_CELL,
    FROZEN_EDGE,
    FROZEN_RANK,
    ATHLETE_COLUMN,
    ATHLETE_CONTENT,
  } from '$lib/components/ui';
  import { resolve } from '$app/paths';
  import { federationPath, formatAthleteName, formatDate, formatWeight } from '$lib/utils';
  import { CELL, FIGURE, SORTED_COLUMN, TEXT_CELL } from '$lib/constants/table';
  import { RANKING_LIFTS } from '$lib/constants/ranking';

  interface Props {
    entries: RankingEntry[];
    showFederation?: boolean;
    busy?: boolean;
  }

  let { entries, showFederation = true, busy = false }: Props = $props();
</script>

<Table {busy}>
  {#snippet head()}
    <th class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_RANK} {FROZEN_EDGE} text-secondary"
      >Rank</th
    >
    <th class="{TABLE_HEAD_CELL} {ATHLETE_COLUMN} text-secondary">Athlete</th>
    <th class="{TABLE_HEAD_CELL} text-secondary">Total</th>
    <th class="{TABLE_HEAD_CELL} text-secondary {SORTED_COLUMN}">
      <RisHeader />
    </th>
    {#each RANKING_LIFTS as lift (lift.value)}
      <th class="{TABLE_HEAD_CELL} text-secondary">{lift.label}</th>
    {/each}
    <th class="{TABLE_HEAD_CELL} text-secondary">Competition</th>
    {#if showFederation}
      <th class="{TABLE_HEAD_CELL} text-secondary">Federation</th>
    {/if}
    <th class="{TABLE_HEAD_CELL} text-secondary">Date</th>
    <th class="{TABLE_HEAD_CELL} text-secondary">Sex</th>
    <th class="{TABLE_HEAD_CELL} text-secondary">Class</th>
  {/snippet}

  {#snippet body()}
    {#each entries as entry (entry.rank + entry.athlete.athlete_id)}
      <tr class="transition-colors">
        <td class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_RANK} {FROZEN_EDGE} {CELL.identity}">
          {entry.rank}
        </td>
        <td class="{TABLE_CELL} {ATHLETE_COLUMN} {CELL.identity}">
          <a
            href={resolve(`/athletes/${entry.athlete.slug}`)}
            class="flex min-w-0 items-center gap-2.5 hover:text-secondary {ATHLETE_CONTENT}"
          >
            <Flag countryCode={entry.athlete.country} class="shrink-0 [--flag-height:1.25em]" />
            <span class="truncate underline">{formatAthleteName(entry.athlete)}</span>
          </a>
        </td>
        <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">{formatWeight(entry.total)}</td>
        <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">
          <RisScore value={entry.ris} source={entry.ris_source} />
        </td>
        <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">{formatWeight(entry.muscleup)}</td>
        <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">{formatWeight(entry.pullup)}</td>
        <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">{formatWeight(entry.dips)}</td>
        <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">{formatWeight(entry.squat)}</td>
        <td class="{TABLE_CELL} {CELL.data}">
          <a
            href={resolve(`/competitions/${entry.competition.slug}`)}
            class="{TEXT_CELL.competition} underline hover:text-secondary"
          >
            {entry.competition.name}
          </a>
        </td>
        {#if showFederation}
          <td class="{TABLE_CELL} {CELL.data}" title={entry.federation.name}>
            <a
              href={resolve(federationPath(entry.federation.name))}
              class="{TEXT_CELL.federation} underline hover:text-ink"
            >
              {entry.federation.abbreviation || entry.federation.name}
            </a>
          </td>
        {/if}
        <td class="{TABLE_CELL} {CELL.data} whitespace-nowrap">
          {formatDate(entry.competition.date)}
        </td>
        <td class="{TABLE_CELL} {CELL.data}">{entry.athlete.gender}</td>
        <td class="{TABLE_CELL} {CELL.data}">{entry.category}</td>
      </tr>
    {/each}
  {/snippet}
</Table>
