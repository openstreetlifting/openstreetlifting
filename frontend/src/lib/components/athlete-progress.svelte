<script lang="ts">
  import ChartPanel from '#lib/components/ui/chart-panel.svelte';
  import Select from '#lib/components/ui/select.svelte';
  import { useI18n } from '#lib/i18n/index.js';
  import { useWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import type { AthleteCompetitionSummary } from '#lib/types/athlete.js';
  import { RANKING_SORTS } from '#lib/constants/ranking.js';
  import { CHART } from '#lib/constants/typography.js';
  import { liftTone } from '#lib/constants/lift.js';
  import { AXIS, PLOT_PADDING, withUnit, yTitle } from '#lib/utils/chart.js';
  import {
    chartableMetrics,
    competitionFocusQuery,
    progressPoints,
    type ProgressMetric,
    type ProgressPoint,
  } from '#lib/utils/athlete-progress.js';
  import { formatLongDate, formatScore } from '#lib/utils/format.js';
  import { resolve } from '$app/paths';
  import { Chart, Layer, Axis, Spline, Points, Circle } from 'layerchart/svg';
  import { Tooltip, type ChartState } from 'layerchart';
  import { scaleUtc, scaleLinear } from 'd3-scale';
  import { utcMonth } from 'd3-time';

  const i18n = useI18n();
  const weights = useWeightUnit();

  let chartWidth = $state<number>();
  let measuredHeight = $state<number>();
  const chartHeight = $derived(measuredHeight || 448);

  let {
    competitions,
    athleteSlug,
    metric = $bindable<ProgressMetric>('total'),
    onMetricChange,
  }: {
    competitions: AthleteCompetitionSummary[];
    athleteSlug?: string;
    metric?: ProgressMetric;
    onMetricChange?: (metric: ProgressMetric) => void;
  } = $props();
  const format = 'MPDS';
  const points = $derived(progressPoints(competitions, metric, format));
  const chartPoints = $derived(
    points.map((point) => ({
      ...point,
      plotted: metric === 'ris' ? point.value : weights.value(point.value),
    }))
  );
  let chartContext = $state<ChartState<(typeof chartPoints)[number]>>();
  const metrics = $derived.by(() => {
    const chartable = chartableMetrics(competitions, format);
    return RANKING_SORTS.filter((option) => chartable.includes(option.value));
  });
  const tone = $derived(liftTone(metric) ?? 'var(--color-chart-athlete)');
  const label = $derived(i18n.t(RANKING_SORTS.find((option) => option.value === metric)!.label));

  const best = $derived(Math.max(...points.map((point) => point.value)));
  const minimum = $derived(Math.min(...points.map((point) => point.value)));
  const yDomain = $derived(
    scaleLinear()
      .domain(
        [Math.min(minimum, best * 0.75), best * 1.05].map((value) =>
          metric === 'ris' ? value : weights.value(value)
        )
      )
      .nice(4)
      .domain()
  );
  const xDomain = $derived([
    utcMonth.floor(new Date(points[0].timestamp)),
    utcMonth.offset(utcMonth.floor(new Date(points.at(-1)!.timestamp)), 1),
  ]);

  function competitionLink(slug: string) {
    const query = competitionFocusQuery(metric, athleteSlug);
    return i18n.href(resolve(`competitions/${slug}?${query}`));
  }

  function valueLabel(value: number): string {
    return metric === 'ris'
      ? formatScore(value, i18n.locale)
      : `${weights.format(value, i18n.locale)} ${weights.unit}`;
  }

  function pointLabel(point: ProgressPoint): string {
    return `${point.competition.competition_name}, ${formatLongDate(point.competition.competition_date, i18n.locale)}, ${valueLabel(point.value)}`;
  }
</script>

{#snippet risNote()}
  {i18n.t('RIS scores are shown as recorded; formula editions and reported scores may differ.')}
{/snippet}

<ChartPanel title={i18n.t('Performance over time')} notes={metric === 'ris' ? risNote : undefined}>
  {#snippet controls()}
    <label class="flex items-center gap-2 text-meta text-muted">
      <span class="sr-only">{i18n.t('Performance')}</span>
      <Select
        name="performance"
        bind:value={metric}
        onchange={(event) => onMetricChange?.(event.currentTarget.value as ProgressMetric)}
        class="px-3 py-1.5"
      >
        {#each metrics as option (option.value)}
          <option value={option.value}>{i18n.t(option.label)}</option>
        {/each}
      </Select>
    </label>
  {/snippet}

  {#key metric + weights.unit}
    <div
      class="{CHART.plot} w-full"
      bind:clientWidth={chartWidth}
      bind:clientHeight={measuredHeight}
      onfocusout={(event) => {
        if (
          event.relatedTarget instanceof Node &&
          event.currentTarget.contains(event.relatedTarget)
        )
          return;
        if (chartContext) {
          chartContext.tooltip.isHoveringTooltipContent = false;
          chartContext.tooltip.hide();
        }
      }}
    >
      <Chart
        bind:context={chartContext}
        ssr
        width={chartWidth || 400}
        class="relative h-full! w-full! overflow-hidden [container-type:inline-size]"
        height={chartHeight}
        data={chartPoints}
        x="timestamp"
        y="plotted"
        xScale={scaleUtc()}
        {xDomain}
        {yDomain}
        padding={{ ...PLOT_PADDING, right: 28, bottom: 36 }}
        tooltipContext={{ mode: 'manual' }}
      >
        {#snippet children({ context })}
          <Layer
            viewBox={`0 0 ${context.containerWidth} ${context.containerHeight}`}
            style="width: 100cqw; height: 100%"
            aria-label={i18n.t('{label} over time', { label })}
            role="group"
          >
            <Axis placement="bottom" {...AXIS} tickMultiline tickOcclusion />
            <Axis
              placement="left"
              {...AXIS}
              {...yTitle(metric === 'ris' ? 'RIS' : withUnit(label, weights.unit))}
              ticks={4}
              format={(value: number) => i18n.number(value)}
              grid={{ class: 'stroke-stroke' }}
            />
            {#if points.length > 1}<Spline stroke={tone} strokeWidth={2} />{/if}
            <Points>
              {#snippet children({ points: marks })}
                {#each marks as point (point.data.competition.competition_id)}
                  <Circle
                    cx={point.x}
                    cy={point.y}
                    r={context.tooltip.data === point.data ? 6 : 4}
                    class="stroke-canvas"
                    fill={tone}
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
                      if (event.pointerType !== 'touch') context.tooltip.show({ data: point.data });
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
              content: CHART.tooltip,
            }}
          >
            {#snippet children({ data: point })}
              <div role="tooltip" aria-live="polite">
                <a
                  href={competitionLink(point.competition.competition_slug)}
                  class="{CHART.tooltipTitle} underline decoration-stroke-strong underline-offset-4"
                >
                  {point.competition.competition_name}
                </a>
                <p class={CHART.tooltipNote}>
                  {formatLongDate(point.competition.competition_date, i18n.locale)} · {weights.category(
                    point.competition.category_name,
                    i18n.locale
                  )}{point.competition.division ? ` · ${point.competition.division}` : ''}
                </p>
                <dl class={CHART.tooltipRows}>
                  <dt>
                    {metric === 'ris'
                      ? `${
                          point.competition.ris_source === 'reported'
                            ? i18n.t('Reported')
                            : point.competition.ris_source === 'computed'
                              ? i18n.t('Computed')
                              : i18n.t('Recorded')
                        } RIS`
                      : label}
                  </dt>
                  <dd class={CHART.tooltipValue}>{valueLabel(point.value)}</dd>
                </dl>
              </div>
            {/snippet}
          </Tooltip.Root>
        {/snippet}
      </Chart>
    </div>
  {/key}
</ChartPanel>
