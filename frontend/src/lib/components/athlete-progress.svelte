<script lang="ts">
  import ChartPanel from '#lib/components/ui/chart-panel.svelte';
  import { useI18n } from '#lib/i18n/index.js';
  import { useWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import { CHART, TEXT } from '#lib/constants/typography.js';
  import { AXIS, PLOT_PADDING, yTitle } from '#lib/utils/chart.js';
  import {
    competitionFocusQuery,
    type LiftProgress,
    type ProgressPoint,
  } from '#lib/utils/athlete-progress.js';
  import type { Lift } from '#lib/constants/lift.js';
  import { formatLongDate } from '#lib/utils/format.js';
  import { resolve } from '$app/paths';
  import { Chart, Layer, Axis, Spline, Circle } from 'layerchart/svg';
  import { Tooltip, type ChartState } from 'layerchart';
  import { scaleUtc, scaleLinear } from 'd3-scale';
  import { utcMonth } from 'd3-time';

  const i18n = useI18n();
  const weights = useWeightUnit();

  let chartWidth = $state<number>();
  let measuredHeight = $state<number>();
  const chartHeight = $derived(measuredHeight || 448);

  let { progress, athleteSlug }: { progress: LiftProgress[]; athleteSlug?: string } = $props();

  type PlottedPoint = ProgressPoint & { lift: Lift; plotted: number };

  const series = $derived(
    progress.map(({ lift, points }) => ({
      lift,
      points: points.map((point): PlottedPoint => ({
        ...point,
        lift,
        plotted: weights.value(point.value),
      })),
    }))
  );
  const chartPoints = $derived(series.flatMap(({ points }) => points));
  let chartContext = $state<ChartState<PlottedPoint>>();

  const yDomain = $derived.by(() => {
    const values = chartPoints.map((point) => point.plotted);
    const low = Math.min(...values);
    const high = Math.max(...values);
    const margin = (high - low) * 0.1 || high * 0.1;
    return scaleLinear()
      .domain([Math.max(0, low - margin), high + margin])
      .nice(4)
      .domain();
  });
  const xDomain = $derived.by(() => {
    const times = chartPoints.map((point) => point.timestamp);
    return [
      utcMonth.floor(new Date(Math.min(...times))),
      utcMonth.offset(utcMonth.floor(new Date(Math.max(...times))), 1),
    ];
  });

  function competitionLink(point: PlottedPoint) {
    const query = competitionFocusQuery(point.lift.key, athleteSlug);
    return i18n.href(resolve(`competitions/${point.competition.competition_slug}?${query}`));
  }

  function pointLabel(point: PlottedPoint): string {
    return `${point.competition.competition_name}, ${formatLongDate(point.competition.competition_date, i18n.locale)}, ${i18n.t(point.lift.label)} ${weights.format(point.value, i18n.locale)} ${weights.unit}`;
  }
</script>

<ChartPanel title={i18n.t('Performance over time')}>
  <ul class="{TEXT.meta} mb-2 flex flex-wrap gap-x-4 gap-y-1 text-secondary">
    {#each series as { lift } (lift.key)}
      <li class="flex items-center gap-2">
        <span class="h-0.5 w-4 rounded-full" style:background={lift.tone} aria-hidden="true"></span>
        {i18n.t(lift.label)}
      </li>
    {/each}
  </ul>

  {#key weights.unit}
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
            aria-label={i18n.t('Performance over time')}
            role="group"
          >
            <Axis placement="bottom" {...AXIS} tickMultiline tickOcclusion />
            <Axis
              placement="left"
              {...AXIS}
              {...yTitle(weights.unit)}
              ticks={4}
              format={(value: number) => i18n.number(value)}
              grid={{ class: 'stroke-stroke' }}
            />
            {#each series as { lift, points } (lift.key)}
              {#if points.length > 1}
                <Spline
                  data={points}
                  x="timestamp"
                  y="plotted"
                  stroke={lift.tone}
                  strokeWidth={2}
                />
              {/if}
            {/each}
            {#each chartPoints as point (`${point.lift.key}:${point.competition.competition_id}`)}
              {@const cx = context.xScale(point.timestamp)}
              {@const cy = context.yScale(point.plotted)}
              <Circle
                {cx}
                {cy}
                r={context.tooltip.data === point ? 6 : 4}
                fill={point.lift.tone}
                class="stroke-canvas"
                strokeWidth={2}
              />
              <Circle
                {cx}
                {cy}
                r={16}
                fill="transparent"
                class="cursor-pointer outline-none focus-visible:stroke-focus"
                strokeWidth={2}
                role="button"
                tabindex={0}
                aria-label={pointLabel(point)}
                aria-pressed={context.tooltip.data === point}
                onpointerenter={(event) => {
                  if (event.pointerType !== 'touch') context.tooltip.show({ data: point });
                }}
                onpointerleave={(event) => {
                  if (event.pointerType !== 'touch') context.tooltip.hide();
                }}
                onclick={() => context.tooltip.show({ data: point })}
                onfocus={() => context.tooltip.show({ data: point })}
                onkeydown={(event) => {
                  if (event.key === 'Escape') {
                    context.tooltip.isHoveringTooltipContent = false;
                    context.tooltip.hide();
                  }
                  if (event.key === 'Enter' || event.key === ' ') {
                    event.preventDefault();
                    context.tooltip.show({ data: point });
                  }
                }}
              />
            {/each}
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
                  href={competitionLink(point)}
                  class="{CHART.tooltipTitle} underline decoration-stroke-strong underline-offset-4"
                >
                  {point.competition.competition_name}
                </a>
              </div>
            {/snippet}
          </Tooltip.Root>
        {/snippet}
      </Chart>
    </div>
  {/key}
</ChartPanel>
