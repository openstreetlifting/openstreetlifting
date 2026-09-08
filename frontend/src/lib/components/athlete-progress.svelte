<script lang="ts">
  import type { AthleteCompetitionSummary } from '$lib/types/athlete';
  import { RANKING_SORTS } from '$lib/constants/ranking';
  import { CHART, FIELD, TEXT } from '$lib/constants/typography';
  import {
    progressPoints,
    type ProgressMetric,
    type ProgressPoint,
  } from '$lib/utils/athlete-progress';
  import { formatLongDate, formatScore, formatWeight } from '$lib/utils/format';
  import { resolve } from '$app/paths';
  import { SvelteURLSearchParams } from 'svelte/reactivity';
  import { Chart, Layer, Axis, Spline, Points, Circle } from 'layerchart/svg';
  import { Tooltip } from 'layerchart';
  import { scaleUtc, scaleLinear } from 'd3-scale';
  import { utcMonth } from 'd3-time';

  let chartWidth = $state<number>();

  let {
    competitions,
    formats,
    athleteSlug,
    metric = $bindable<ProgressMetric>('total'),
    format = $bindable('MPDS'),
    onMetricChange,
    onFormatChange,
  }: {
    competitions: AthleteCompetitionSummary[];
    formats: string[];
    athleteSlug?: string;
    metric?: ProgressMetric;
    format?: string;
    onMetricChange?: (metric: ProgressMetric) => void;
    onFormatChange?: (format: string) => void;
  } = $props();
  const points = $derived(progressPoints(competitions, metric, format));
  const label = $derived(RANKING_SORTS.find((option) => option.value === metric)!.label);
  const best = $derived(points.length ? Math.max(...points.map((point) => point.value)) : null);
  const bestPoint = $derived(points.findLast((point) => point.value === best));
  const minimum = $derived(points.length ? Math.min(...points.map((point) => point.value)) : 0);
  const padding = $derived(Math.max(((best ?? 0) - minimum) * 0.15, (best ?? 0) * 0.03, 1));
  const yDomain = $derived(
    scaleLinear()
      .domain([Math.max(0, minimum - padding), (best ?? 0) + padding])
      .nice(4)
      .domain()
  );
  const first = $derived(utcMonth.floor(new Date(points[0]?.timestamp ?? 0)));
  const last = $derived(
    utcMonth.offset(utcMonth.floor(new Date(points.at(-1)?.timestamp ?? 0)), 1)
  );
  const xDomain = $derived([points.length === 1 ? utcMonth.offset(first, -1) : first, last]);
  const monthLabel = new Intl.DateTimeFormat('en', {
    month: 'short',
    year: 'numeric',
    timeZone: 'UTC',
  });

  function competitionLink(slug: string) {
    const query = new SvelteURLSearchParams({ movement: metric });
    if (athleteSlug) query.set('athlete', athleteSlug);
    return resolve(`/competitions/${slug}?${query}`);
  }

  function valueLabel(value: number): string {
    return metric === 'ris' ? formatScore(value) : `${formatWeight(value)} kg`;
  }

  function pointLabel(point: ProgressPoint): string {
    return `${point.competition.competition_name}, ${formatLongDate(point.competition.competition_date)}, ${valueLabel(point.value)}`;
  }
</script>

<section class="min-w-0 max-w-reading" aria-labelledby="progress-heading">
  <div class="mb-4 flex min-h-8 flex-wrap items-center justify-between gap-3">
    <h2 id="progress-heading" class="{TEXT.heading} text-ink">Performance over time</h2>
    <div class="flex flex-wrap items-center gap-3">
      <label class="flex items-center gap-2 text-xs text-muted">
        <span class="sr-only">Performance</span>
        <select
          bind:value={metric}
          onchange={(event) => onMetricChange?.(event.currentTarget.value as ProgressMetric)}
          class="{FIELD} px-2.5 py-1.5"
        >
          {#each RANKING_SORTS as option (option.value)}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </label>
      {#if metric === 'total' && formats.length > 1}
        <label class="flex items-center gap-2 text-xs text-muted">
          Event
          <select
            bind:value={format}
            onchange={(event) => onFormatChange?.(event.currentTarget.value)}
            class="{FIELD} px-2.5 py-1.5"
          >
            {#each formats as option (option)}<option value={option}
                >{option === 'MPDS' ? 'All four lifts' : option}</option
              >{/each}
          </select>
        </label>
      {/if}
    </div>
  </div>

  {#if points.length && best !== null && bestPoint}
    <div class="mb-3 flex justify-end 2xl:min-h-12">
      <p class="text-xs leading-relaxed text-muted">
        <span class="font-medium uppercase">Best</span>
        <span class="ml-1 font-mono font-semibold text-ink">{valueLabel(best)}</span>
        on {formatLongDate(bestPoint.competition.competition_date)} at
        <a
          href={competitionLink(bestPoint.competition.competition_slug)}
          class="text-secondary underline decoration-stroke-strong underline-offset-4 hover:text-ink"
          >{bestPoint.competition.competition_name}</a
        >
      </p>
    </div>
    {#key metric + format}
      <div class="h-80 w-full" bind:clientWidth={chartWidth}>
        <Chart
          ssr
          width={chartWidth || 400}
          class="relative w-full! overflow-hidden [container-type:inline-size]"
          height={320}
          data={points}
          x="timestamp"
          y="value"
          xScale={scaleUtc()}
          {xDomain}
          {yDomain}
          padding={{ left: 52, right: 28, top: 20, bottom: 52 }}
          tooltipContext={{ mode: 'manual' }}
        >
          {#snippet children({ context })}
            <Layer
              viewBox={`0 0 ${context.containerWidth} ${context.containerHeight}`}
              style="width: 100cqw; height: 100%"
              aria-label="{label} over time"
              role="group"
            >
              <Axis
                placement="bottom"
                label="Date"
                labelProps={{ class: CHART.label }}
                ticks={{
                  interval: utcMonth.every(
                    Math.max(
                      1,
                      Math.ceil(
                        utcMonth.count(xDomain[0], xDomain[1]) /
                          Math.max(1, Math.floor(context.width / 90))
                      )
                    )
                  ),
                }}
                format={(date: Date) => monthLabel.format(date)}
                tickLabelProps={{ class: CHART.tick }}
                tickOcclusion
                grid={{ class: 'stroke-stroke' }}
                stroke="var(--color-stroke)"
              />
              <Axis
                placement="left"
                label={metric === 'ris' ? 'RIS' : 'kg'}
                labelProps={{ class: CHART.label }}
                ticks={4}
                tickLabelProps={{ class: CHART.tick }}
                grid={{ class: 'stroke-stroke' }}
                stroke="var(--color-stroke)"
              />
              {#if points.length > 1}<Spline class="stroke-chart-athlete" strokeWidth={2} />{/if}
              <Points>
                {#snippet children({ points: marks })}
                  {#each marks as point (point.data.competition.competition_id)}
                    <Circle
                      cx={point.x}
                      cy={point.y}
                      r={context.tooltip.data === point.data ? 6 : 4}
                      class="fill-chart-athlete stroke-canvas"
                      strokeWidth={2}
                    />
                    <Circle
                      cx={point.x}
                      cy={point.y}
                      r={16}
                      fill="transparent"
                      class="cursor-pointer outline-none focus-visible:stroke-focus"
                      strokeWidth={2}
                      role="button"
                      tabindex={0}
                      aria-label={pointLabel(point.data)}
                      aria-pressed={context.tooltip.data === point.data}
                      onpointerenter={(event) => {
                        if (event.pointerType !== 'touch')
                          context.tooltip.show({ data: point.data });
                      }}
                      onpointerleave={(event) => {
                        if (event.pointerType !== 'touch') context.tooltip.hide();
                      }}
                      onclick={() => context.tooltip.show({ data: point.data })}
                      onfocus={() => context.tooltip.show({ data: point.data })}
                      onkeydown={(event) => {
                        if (event.key === 'Escape') {
                          context.tooltip.isHoveringTooltipContent = false;
                          context.tooltip.hide();
                        }
                        if (event.key === 'Enter' || event.key === ' ') {
                          event.preventDefault();
                          context.tooltip.show({ data: point.data });
                        }
                      }}
                    />
                  {/each}
                {/snippet}
              </Points>
            </Layer>
            <Tooltip.Root
              {context}
              x="data"
              y="data"
              yOffset={12}
              anchor="bottom"
              contained="container"
              portal={false}
              pointerEvents
              motion="none"
              fadeDuration={0}
              variant="none"
              onkeydown={(event) => {
                if (event.key === 'Escape') {
                  context.tooltip.isHoveringTooltipContent = false;
                  context.tooltip.hide();
                }
              }}
              classes={{
                content:
                  'max-w-64 rounded-lg border border-stroke-strong bg-surface p-3 text-ink shadow-lg shadow-overlay/40',
              }}
            >
              {#snippet children({ data: point })}
                <div role="tooltip" aria-live="polite">
                  <p class="font-mono text-lg font-bold">{valueLabel(point.value)}</p>
                  <a
                    href={competitionLink(point.competition.competition_slug)}
                    class="mt-1 inline-block text-sm text-secondary underline decoration-stroke-strong underline-offset-4 hover:text-ink"
                    >{point.competition.competition_name}</a
                  >
                  <p class="mt-1 text-xs text-muted">
                    {formatLongDate(point.competition.competition_date)} · {point.competition
                      .category_name}{point.competition.division
                      ? ` · ${point.competition.division}`
                      : ''}
                  </p>
                  {#if metric === 'ris'}<p class="mt-1 text-xs text-muted">
                      {point.competition.ris_source === 'reported'
                        ? 'Reported'
                        : point.competition.ris_source === 'computed'
                          ? 'Computed'
                          : 'Recorded'} RIS
                    </p>{/if}
                </div>
              {/snippet}
            </Tooltip.Root>
          {/snippet}
        </Chart>
      </div>
    {/key}
    {#if points.length === 1}
      <p class="mt-3 text-xs leading-relaxed text-muted">
        One result recorded. A trend will appear after another competition.
      </p>
    {/if}
    {#if metric === 'ris'}
      <p class="mt-3 text-xs leading-relaxed text-muted">
        RIS scores are shown as recorded; formula editions and reported scores may differ.
      </p>
    {/if}
  {:else}
    <p class="border-y border-stroke py-8 text-sm text-secondary">
      No dated {label} results available{metric === 'total'
        ? ` for ${format === 'MPDS' ? 'all four lifts' : format}`
        : ''}.
    </p>
  {/if}
</section>
