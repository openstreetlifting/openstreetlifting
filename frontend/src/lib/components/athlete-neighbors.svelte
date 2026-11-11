<script lang="ts">
  import { resolve } from '$app/paths';
  import { useI18n } from '#lib/i18n/index.js';
  import { useWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import type { AthleteNeighbor, AthleteNeighborhood } from '#lib/types/athlete.js';
  import { countryName, formatAthleteName } from '#lib/utils/index.js';
  import { Flag, SegmentedControl, Text } from '#lib/components/ui/index.js';
  import { FIGURE } from '#lib/constants/table.js';

  let {
    neighbors,
    athleteSlug,
    country,
  }: {
    neighbors: AthleteNeighborhood | null | undefined;
    athleteSlug: string;
    country: string | null;
  } = $props();
  const i18n = useI18n();
  const weights = useWeightUnit();
  const SCOPES = ['global', 'national'] as const;
  let scope = $state<(typeof SCOPES)[number]>('global');
  const rows = $derived(neighbors?.[scope] ?? []);
  const own = $derived(rows.find((row) => row.slug === athleteSlug));
  const lineup = $derived(rows.toReversed());
  const weightClass = $derived(weights.weightClass(neighbors?.category, i18n.locale));
  const signed = (gap: number) =>
    `${gap > 0 ? '+' : '−'}${weights.format(Math.abs(gap), i18n.locale)}`;
  const BOX =
    '-mx-2 flex w-full min-w-0 items-baseline gap-1.5 rounded-control px-2 py-1.5 @3xl:w-auto @3xl:py-0.5';
  const NAME = 'min-w-0 flex-1 truncate @3xl:max-w-40 @3xl:flex-none';
</script>

{#snippet total(row: AthleteNeighbor)}
  <span class="{FIGURE} whitespace-nowrap font-semibold text-ink"
    >{weights.format(row.value, i18n.locale)}&nbsp;{weights.unit}</span
  >
{/snippet}

<section class="@container mb-section" aria-label={i18n.t('Rivals')}>
  <div class="mb-4">
    <Text as="h2" variant="heading" class="text-ink"
      >{weightClass
        ? i18n.t('Rivals (class: {weightClass})', { weightClass })
        : i18n.t('Rivals')}</Text
    >
    <SegmentedControl
      class="mt-3"
      items={SCOPES}
      key={(option) => option}
      selected={(option) => option === scope}
      onselect={(option) => (scope = option)}
      label={i18n.t('Ranking scope')}
    >
      {#snippet children(option)}
        {option === 'global'
          ? i18n.t('Global')
          : country
            ? countryName(country, i18n.locale)
            : i18n.t('National')}
      {/snippet}
    </SegmentedControl>
  </div>
  {#if scope === 'national' && !country}
    <Text variant="body" class="text-muted">{i18n.t('Country needed for national ranking')}</Text>
  {:else if !neighbors || !own}
    <Text variant="body" class="text-muted"
      >{i18n.t(
        'An All4 total in the latest All4 category is needed to show nearby athletes.'
      )}</Text
    >
  {:else}
    <ol
      class="flex flex-col-reverse gap-0.5 text-body @3xl:flex-row @3xl:flex-wrap @3xl:items-baseline @3xl:gap-x-4 @3xl:gap-y-2"
      aria-label={i18n.t('Athletes')}
    >
      {#each lineup as row, index (row.athlete_id)}
        {@const isOwn = row.slug === athleteSlug}
        {@const next = lineup[index + 1]}
        {@const gap = Number(row.gap)}
        <li
          class="flex min-w-0 items-baseline @3xl:gap-4"
          aria-current={isOwn ? 'true' : undefined}
        >
          {#if isOwn}
            <span class="{BOX} bg-surface-selected">
              <Flag countryCode={row.country} class="shrink-0 self-center [--flag-height:1em]" />
              <span class="{NAME} font-semibold text-ink">{formatAthleteName(row)}</span>
              <span class="sr-only">{i18n.t('Profile athlete')}</span>
              <a
                class="shrink-0 whitespace-nowrap underline-offset-2 hover:underline"
                href={i18n.href(resolve('/competitions/[slug]', { slug: row.competition_slug }))}
                >{@render total(row)}</a
              >
            </span>
          {:else}
            <a
              class="{BOX} text-secondary transition-colors hover:bg-surface-selected hover:text-ink focus-visible:bg-surface-selected focus-visible:ring-2 focus-visible:ring-focus focus-visible:outline-none"
              href={i18n.href(resolve('/athletes/[slug]', { slug: row.slug }))}
            >
              <Flag countryCode={row.country} class="shrink-0 self-center [--flag-height:1em]" />
              <span class={NAME}>{formatAthleteName(row)}</span>
              {#if gap === 0}
                {@render total(row)}
              {:else}
                <span class="{FIGURE} shrink-0 whitespace-nowrap font-semibold text-ink"
                  >{signed(gap)}&nbsp;{weights.unit}</span
                >
                <span class="shrink-0 whitespace-nowrap text-muted"
                  >({weights.format(row.value, i18n.locale)}&nbsp;{weights.unit})</span
                >
              {/if}
            </a>
          {/if}
          {#if next}
            <span class="{FIGURE} hidden text-muted @3xl:inline" aria-hidden="true"
              >{Number(next.value) === Number(row.value) ? '=' : '<'}</span
            >
          {/if}
        </li>
      {/each}
    </ol>
    {#if own.field === 1}
      <Text variant="meta" class="mt-3 text-muted"
        >{i18n.t('The only ranked athlete in this category and scope.')}</Text
      >
    {/if}
  {/if}
</section>
