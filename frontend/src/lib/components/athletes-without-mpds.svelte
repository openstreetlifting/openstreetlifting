<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { useWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import { resolve } from '$app/paths';
  import { LAYOUT } from '#lib/constants/layout.js';
  import { RANKING_LIFTS } from '#lib/constants/ranking.js';
  import { liftTone } from '#lib/constants/lift.js';
  import { CELL, FIGURE, NO_VALUE, TEXT_CELL } from '#lib/constants/table.js';
  import { TEXT } from '#lib/constants/typography.js';
  import { formatAthleteName, formatDate } from '#lib/utils/index.js';
  import { slugify } from '#lib/utils/slug.js';
  import {
    Flag,
    Table,
    Text,
    TABLE_CELL,
    TABLE_HEAD_CELL,
    FROZEN_CELL,
    FROZEN_HEAD_CELL,
    ATHLETE_COLUMN,
    ATHLETE_CONTENT,
  } from '#lib/components/ui/index.js';
  import type { AthleteSearchHit } from '#lib/types/athlete.js';

  const i18n = useI18n();
  const weights = useWeightUnit();

  let { hits, total }: { hits: AthleteSearchHit[]; total: number } = $props();

  const hidden = $derived(Math.max(0, total - hits.length));
  const title = $derived(i18n.t('Athletes found without an MPDS result'));
</script>

<section class={LAYOUT.section} aria-labelledby="athletes-without-mpds">
  <Text as="h2" id="athletes-without-mpds" variant="heading" class="mb-3 text-ink">
    {title} ({total})
  </Text>
  <div class="min-w-0 rounded-panel border border-stroke-strong">
    <Table embedded frozenColumns="rank" rows={hits} itemName="athlete" label={title}>
      {#snippet head()}
        <th class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} left-0 {ATHLETE_COLUMN} text-secondary">
          {i18n.t('Athlete')}
        </th>
        {#each RANKING_LIFTS as lift (lift.value)}
          <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t(lift.label)}</th>
        {/each}
        <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Format')}</th>
        <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Competition')}</th>
        <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Federation')}</th>
        <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Date')}</th>
        <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Sex')}</th>
        <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Class')}</th>
      {/snippet}

      {#snippet body(rows)}
        {#each rows as hit (hit.slug)}
          {@const entry = hit.best_entry}
          <tr class="transition-colors">
            <td class="{TABLE_CELL} {FROZEN_CELL} left-0 {ATHLETE_COLUMN} {CELL.identity}">
              <span class="flex items-center gap-1.5 {ATHLETE_CONTENT}">
                <span class="flex min-w-0 items-center gap-2">
                  <Flag link countryCode={hit.country} class="shrink-0 [--flag-height:1.25em]" />
                  <a
                    href={i18n.href(resolve(`athletes/${hit.slug}`))}
                    class="truncate underline hover:text-secondary"
                  >
                    {formatAthleteName(hit)}
                  </a>
                </span>
              </span>
            </td>
            {#each RANKING_LIFTS as lift (lift.value)}
              <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">
                <span style:color={liftTone(lift.value)}>
                  {weights.format(entry?.[lift.value], i18n.locale)}
                </span>
              </td>
            {/each}
            <td class="{TABLE_CELL} {CELL.data}">{entry?.event ?? NO_VALUE}</td>
            <td class="{TABLE_CELL} {CELL.data}">
              {#if entry}
                <a
                  href={i18n.href(resolve(`competitions/${entry.competition.slug}`))}
                  class="{TEXT_CELL.resultCompetition} underline hover:text-secondary"
                >
                  {entry.competition.name}
                </a>
              {:else}
                {NO_VALUE}
              {/if}
            </td>
            <td class="{TABLE_CELL} {CELL.data}" title={entry?.federation.name}>
              {#if entry}
                <a
                  href={i18n.href(
                    resolve('/federations/[slug]', { slug: slugify(entry.federation.name) })
                  )}
                  class="{TEXT_CELL.federation} underline hover:text-ink"
                >
                  {entry.federation.abbreviation || entry.federation.name}
                </a>
              {:else}
                {NO_VALUE}
              {/if}
            </td>
            <td class="{TABLE_CELL} {CELL.data} whitespace-nowrap">
              {entry?.competition.date ? formatDate(entry.competition.date, i18n.locale) : NO_VALUE}
            </td>
            <td class="{TABLE_CELL} {CELL.data}">{entry?.gender ?? hit.gender}</td>
            <td class="{TABLE_CELL} {CELL.data}">
              {(entry && weights.category(entry.category, i18n.locale)) || NO_VALUE}
            </td>
          </tr>
        {/each}
      {/snippet}
    </Table>
  </div>
  {#if hidden > 0}
    <p class="mt-2 {TEXT.caption}">
      {i18n.t('{count} more. Type more of the name to narrow the list.', { count: hidden })}
    </p>
  {/if}
</section>
