<script lang="ts">
  import { slugify } from '#lib/utils/slug.js';
  import type { TablePagination } from '#lib/types/pagination.js';
  import type { Competition } from '#lib/types/competition.js';
  import { Table, TABLE_CELL, TABLE_HEAD_CELL } from '#lib/components/ui/index.js';
  import { resolve } from '$app/paths';
  import { formatCountdown, formatDate, formatLocation } from '#lib/utils/index.js';
  import { TEXT } from '#lib/constants/typography.js';
  import { CELL, FIGURE, TEXT_CELL } from '#lib/constants/table.js';

  interface Props {
    competitions: Competition[];
    upcoming?: boolean;
    showFederation?: boolean;
    busy?: boolean;
    pagination?: TablePagination;
  }

  let {
    competitions,
    upcoming = false,
    showFederation = true,
    busy = false,
    pagination,
  }: Props = $props();

  function competitionDates(start: string | null, end: string | null): string {
    const from = formatDate(start);
    return end && end !== start ? `${from} - ${formatDate(end)}` : from;
  }
</script>

<Table
  rows={competitions}
  itemName="competition"
  {pagination}
  {busy}
  pageParam={upcoming ? 'upcoming_page' : 'competitions_page'}
>
  {#snippet head()}
    <th class="{TABLE_HEAD_CELL} text-secondary">Competition</th>
    <th class="{TABLE_HEAD_CELL} text-secondary">Format</th>
    <th class="{TABLE_HEAD_CELL} text-secondary">{upcoming ? 'When' : 'Lifters'}</th>
    <th class="{TABLE_HEAD_CELL} text-secondary">Date</th>
    <th class="{TABLE_HEAD_CELL} text-secondary">Location</th>
    {#if showFederation}
      <th class="{TABLE_HEAD_CELL} text-secondary">Federation</th>
    {/if}
  {/snippet}

  {#snippet body(rows)}
    {#each rows as competition (competition.slug)}
      <tr class="transition-colors">
        <td class="{TABLE_CELL} {CELL.identity}">
          <a
            href={resolve(`competitions/${competition.slug}`)}
            class="{TEXT_CELL.competition} underline hover:text-secondary"
          >
            {competition.name}
          </a>
          {#if competition.status === 'live'}
            <span class="ml-2 {TEXT.micro} text-secondary">In Progress</span>
          {/if}
        </td>
        <td
          class="{TABLE_CELL} {CELL.data} whitespace-nowrap"
          title={competition.movements.map(({ movement_name }) => movement_name).join(' · ')}
          >{competition.event_code ?? '—'}</td
        >

        <td class="{TABLE_CELL} {CELL.data} whitespace-nowrap {upcoming ? '' : FIGURE}">
          {upcoming ? formatCountdown(competition.start_date) : (competition.lifter_count ?? 0)}
        </td>

        <td class="{TABLE_CELL} whitespace-nowrap text-secondary"
          >{competitionDates(competition.start_date, competition.end_date)}</td
        >

        <td class="{TABLE_CELL} {CELL.data}">
          <span class={TEXT_CELL.location}>
            {formatLocation(competition.country, competition.region, competition.city)}
          </span>
        </td>
        {#if showFederation}
          <td class="{TABLE_CELL} {CELL.data}" title={competition.federation.name}>
            <a
              href={resolve('/federations/[slug]', { slug: slugify(competition.federation.name) })}
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
