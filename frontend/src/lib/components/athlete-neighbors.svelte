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
  const BOX = '-mx-2 flex min-w-0 items-baseline gap-1.5 rounded-control px-2 py-0.5';
</script>

{#snippet total(row: AthleteNeighbor)}
  <span class="{FIGURE} whitespace-nowrap font-semibold text-ink"
    >{weights.format(row.value, i18n.locale)}&nbsp;{weights.unit}</span
  >
{/snippet}

<section class="mb-section" aria-label={i18n.t('Current rivals')}>
  <div class="mb-4">
    <Text as="h2" variant="heading" class="text-ink"
      >{weightClass
        ? `${i18n.t('Current rivals')} (${weightClass})`
        : i18n.t('Current rivals')}</Text
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
      class="flex flex-wrap items-baseline gap-x-4 gap-y-2 text-body"
      aria-label={i18n.t('Athletes')}
    >
      {#each lineup as row, index (row.athlete_id)}
        {@const isOwn = row.slug === athleteSlug}
        {@const next = lineup[index + 1]}
        {@const gap = Number(row.gap)}
        <li class="flex min-w-0 items-baseline gap-4" aria-current={isOwn ? 'true' : undefined}>
          {#if isOwn}
            <span class="{BOX} bg-surface-selected">
              <Flag countryCode={row.country} class="shrink-0 self-center [--flag-height:1em]" />
              <span class="max-w-40 truncate font-semibold text-ink">{formatAthleteName(row)}</span>
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
              <span class="max-w-40 truncate">{formatAthleteName(row)}</span>
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
            <span class="{FIGURE} text-muted" aria-hidden="true"
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
