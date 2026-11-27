<script lang="ts">
  import ChartPanel from '#lib/components/ui/chart-panel.svelte';
  import Select from '#lib/components/ui/select.svelte';
  import { useI18n } from '#lib/i18n/index.js';
  import { useWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import type { AthleteCompetitionSummary } from '#lib/types/athlete.js';
  import { RANKING_SORTS } from '#lib/constants/ranking.js';
  import { CHART } from '#lib/constants/typography.js';
  import { AXIS, PLOT_PADDING, withUnit, yTitle } from '#lib/utils/chart.js';
  import {
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
  const chartHeight = $derived(measuredHeight || 288);

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
  const label = $derived(i18n.t(RANKING_SORTS.find((option) => option.value === metric)!.label));

  const best = $derived(points.length ? Math.max(...points.map((point) => point.value)) : null);

  const minimum = $derived(points.length ? Math.min(...points.map((point) => point.value)) : 0);
  const yDomain = $derived(
    scaleLinear()
      .domain(
        [Math.min(minimum, (best ?? 0) * 0.75), (best ?? 0) * 1.05].map((value) =>
          metric === 'ris' ? value : weights.value(value)
        )
      )
      .nice(4)
      .domain()
  );
  const first = $derived(utcMonth.floor(new Date(points[0]?.timestamp ?? 0)));
  const last = $derived(
    utcMonth.offset(utcMonth.floor(new Date(points.at(-1)?.timestamp ?? 0)), 1)
  );

  const xDomain = $derived([points.length === 1 ? utcMonth.offset(first, -1) : first, last]);

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

<ChartPanel title={i18n.t('Performance over time')}>
  {#snippet controls()}
    <label class="flex items-center gap-2 text-meta text-muted">
      <span class="sr-only">{i18n.t('Performance')}</span>
      <Select
        name="performance"
        bind:value={metric}
        onchange={(event) => onMetricChange?.(event.currentTarget.value as ProgressMetric)}
        class="px-3 py-1.5"
      >
        {#each RANKING_SORTS as option (option.value)}
          <option value={option.value}>{i18n.t(option.label)}</option>
        {/each}
      </Select>
    </label>
  {/snippet}

  {#if points.length && best !== null}
    {#key metric + weights.unit}
      <div
        class="min-h-64 w-full flex-1"
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
    {#if points.length === 1}
      <p class="mt-3 text-meta leading-relaxed text-muted">
        {i18n.t('One result recorded. A trend will appear after another competition.')}
      </p>
    {/if}
    {#if metric === 'ris'}
      <p class="mt-3 text-meta leading-relaxed text-muted">
        {i18n.t(
          'RIS scores are shown as recorded; formula editions and reported scores may differ.'
        )}
      </p>
    {/if}
  {:else}
    <p class="border-y border-stroke py-8 text-body text-secondary">
      {i18n.t('No dated {movement} results available for {format}.', {
        movement: label,
        format,
      })}
    </p>
  {/if}
</ChartPanel>
