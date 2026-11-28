<script module lang="ts">
  import type { Lift } from '#lib/constants/lift.js';
  import type { Movement } from '#lib/types/enums.js';

  export type ResultColumns = {
    event: string | null;
    ris: boolean;
    risPublished: boolean;
    lifts: readonly Lift[];
    withAttempts: Set<Movement>;
    field: boolean;
    classed: boolean;
    age: boolean;
    weight: boolean;
  };
</script>

<script lang="ts">
  import RecordMark from '#lib/components/ui/record-mark.svelte';
  import {
    Flag,
    Table,
    RisHeader,
    RisScore,
    TABLE_CELL,
    TABLE_HEAD_CELL,
    FROZEN_CELL,
    FROZEN_HEAD_CELL,
    FROZEN_RANK,
    FROZEN_ATHLETE,
    ATHLETE_COLUMN_FILL,
    ATHLETE_CONTENT_FILL,
    RankHeader,
  } from '#lib/components/ui/index.js';
  import {
    ATTEMPT_ROW,
    BEST_ROW,
    CELL,
    STATUS_FLAG,
    SORTED_COLUMN,
    NO_VALUE,
    NO_RESULT,
  } from '#lib/constants/table.js';
  import { ATHLETE_STATUS_LABEL, athleteStatusTitle } from '#lib/constants/athlete-status.js';
  import { useI18n } from '#lib/i18n/index.js';
  import { useWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import { formatAthleteName } from '#lib/utils/index.js';
  import type { ResultRow } from '#lib/utils/competition-results.js';
  import type { Attempt, Participant } from '#lib/types/competition.js';
  import type { RecordMark as RecordMarkValue } from '#lib/types/records.js';
  import type { TablePagination } from '#lib/types/pagination.js';
  import { resolve } from '$app/paths';

  const i18n = useI18n();
  const weights = useWeightUnit();

  interface Props {
    rows: ResultRow[];
    columns: ResultColumns;
    sorted: string;
    focused: string | null;
    label: string;
    pagination?: TablePagination;
  }

  let { rows, columns, sorted, focused, label, pagination }: Props = $props();

  const highlight = (column: string) => (sorted === column ? SORTED_COLUMN : '');

  type LiftCell =
    | { kind: 'absent' }
    | { kind: 'bombed' }
    | { kind: 'best'; best: number; record: RecordMarkValue | null }
    | {
        kind: 'attempts';
        attempts: Attempt[];
        best: number | null;
        record: RecordMarkValue | null;
      };

  function liftCell(participant: Participant, lift: Lift): LiftCell {
    if (!columns.event?.includes(lift.code)) return { kind: 'absent' };
    const result = participant.lifts.find(({ movement_name }) => movement_name === lift.movement);
    if (!result) return { kind: 'absent' };
    const best = result.best_weight == null ? null : Number(result.best_weight);
    if (result.attempts.length) {
      return { kind: 'attempts', attempts: result.attempts, best, record: result.record };
    }
    return best === null ? { kind: 'bombed' } : { kind: 'best', best, record: result.record };
  }
</script>

<Table
  embedded
  {rows}
  {label}
  frozenColumns="athlete"
  itemName="athlete"
  paged={pagination != null}
  {pagination}
>
  {#snippet head()}
    <th class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_RANK} text-secondary">
      <RankHeader />
    </th>
    <th
      class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_ATHLETE} {ATHLETE_COLUMN_FILL} text-secondary"
    >
      {i18n.t('Athlete')}
    </th>
    <th class="{TABLE_HEAD_CELL} whitespace-normal text-secondary {highlight('total')}">
      {i18n.t('Total')}
      <span class="mt-0.5 block text-micro font-normal text-muted">{weights.unit}</span>
    </th>
    {#if columns.ris}
      <th class="{TABLE_HEAD_CELL} text-secondary {highlight('ris')}">
        <RisHeader published={columns.risPublished} />
      </th>
    {/if}
    {#each columns.lifts as lift (lift.key)}
      <th class="{TABLE_HEAD_CELL} text-secondary {highlight(lift.key)}">
        {i18n.t(lift.label)}
        {#if columns.withAttempts.has(lift.movement)}
          <span class="{ATTEMPT_ROW} mt-0.5 text-micro font-normal text-muted">
            <span class="text-right">1</span>
            <span class="text-right">2</span>
            <span class="text-right">3</span>
            <span class="text-right">{i18n.t('Best')}</span>
          </span>
        {/if}
      </th>
    {/each}
    {#if columns.field}
      <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Sex')}</th>
      {#if columns.classed}
        <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Class')}</th>
      {/if}
    {/if}
    {#if columns.age}
      <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Age')}</th>
    {/if}
    {#if columns.weight}
      <th class="{TABLE_HEAD_CELL} whitespace-normal text-secondary">
        {i18n.t('Weight')}
        <span class="mt-0.5 block text-micro font-normal text-muted">{weights.unit}</span>
      </th>
    {/if}
  {/snippet}

  {#snippet body(visible)}
    {#each visible as { participant, category, rank } (participant.participant_id)}
      {@const competed = participant.status === 'competed'}
      <tr
        class="transition-colors"
        data-focused={participant.athlete.slug === focused ? '' : undefined}
      >
        <td
          class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_RANK} {rank == null
            ? CELL.absent
            : CELL.identity}"
        >
          {rank ?? NO_VALUE}
        </td>
        <td
          class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_ATHLETE} {ATHLETE_COLUMN_FILL} {CELL.identity}"
        >
          <span class="flex items-center gap-1.5 {ATHLETE_CONTENT_FILL}">
            <span class="flex min-w-0 items-center gap-2">
              <Flag
                link
                countryCode={participant.athlete.country}
                class="shrink-0 [--flag-height:1.25em]"
              />
              <a
                href={i18n.href(resolve(`athletes/${participant.athlete.slug}`))}
                class="truncate underline hover:text-secondary"
              >
                {formatAthleteName(participant.athlete)}
              </a>
            </span>
            {#if !competed}
              <span
                class="shrink-0 text-micro font-medium tracking-wide uppercase {STATUS_FLAG}"
                title={athleteStatusTitle(
                  participant.status,
                  participant.status_reason,
                  i18n.locale
                )}
              >
                {ATHLETE_STATUS_LABEL[participant.status]}
              </span>
            {/if}
          </span>
        </td>
        <td class="{TABLE_CELL} {CELL.counted}">
          {#if competed}
            {weights.format(participant.total, i18n.locale)}<RecordMark
              worldOnly
              mark={participant.total_record}
            />
          {:else}
            {NO_RESULT}
          {/if}
        </td>
        {#if columns.ris}
          <td class="{TABLE_CELL} {CELL.counted}">
            {#if competed}
              <RisScore value={participant.ris_score} source={participant.ris_source} />
            {:else}
              {NO_RESULT}
            {/if}
          </td>
        {/if}
        {#each columns.lifts as lift (lift.key)}
          {@const cell = liftCell(participant, lift)}
          <td class="{TABLE_CELL} whitespace-nowrap">
            <span class={columns.withAttempts.has(lift.movement) ? ATTEMPT_ROW : BEST_ROW}>
              {#if cell.kind === 'attempts'}
                {#each [1, 2, 3] as slot (slot)}
                  {@const attempt = cell.attempts.find((a) => a.attempt_number === slot)}
                  <span
                    class="text-right {attempt && !attempt.is_successful
                      ? CELL.discounted
                      : CELL.data}"
                    title={attempt
                      ? i18n.t('Attempt {slot}: {weight} {unit}, {result}', {
                          unit: weights.unit,
                          slot,
                          weight: weights.format(attempt.weight, i18n.locale),
                          result: attempt.is_successful ? i18n.t('good lift') : i18n.t('no lift'),
                        })
                      : i18n.t('Attempt {slot} not recorded', { slot })}
                  >
                    {attempt ? weights.format(attempt.weight, i18n.locale) : ''}
                  </span>
                {/each}
              {:else if columns.withAttempts.has(lift.movement)}
                <span></span>
                <span></span>
                <span></span>
              {/if}
              <span class="text-right {CELL.counted}">
                {#if cell.kind === 'absent'}
                  <span class="font-normal {CELL.absent}">{NO_VALUE}</span>
                {:else if cell.kind === 'bombed'}
                  <span
                    class="font-normal {CELL.nothing}"
                    title={i18n.t('No successful {movement}', {
                      movement: i18n.t(lift.label).toLowerCase(),
                    })}
                  >
                    {NO_RESULT}
                  </span>
                {:else}
                  <span style:color={lift.tone}>
                    {weights.format(cell.best, i18n.locale)}
                  </span><RecordMark worldOnly mark={cell.record} />
                {/if}
              </span>
            </span>
          </td>
        {/each}
        {#if columns.field}
          <td class="{TABLE_CELL} {CELL.data}">{participant.athlete.gender}</td>
          {#if columns.classed}
            <td class="{TABLE_CELL} {CELL.data}">
              {weights.category(category.weight_class, i18n.locale)}
            </td>
          {/if}
        {/if}
        {#if columns.age}
          <td class="{TABLE_CELL} {CELL.data}">
            {#if participant.age == null}
              <span class={CELL.absent}>{NO_VALUE}</span>
            {:else}
              {i18n.number(participant.age, 0)}
            {/if}
          </td>
        {/if}
        {#if columns.weight}
          <td class="{TABLE_CELL} {CELL.data}">
            {#if participant.bodyweight == null}
              <span class={CELL.absent}>{NO_VALUE}</span>
            {:else}
              {weights.format(participant.bodyweight, i18n.locale)}
            {/if}
          </td>
        {/if}
      </tr>
    {/each}
  {/snippet}
</Table>
