<script lang="ts">
  import { Chart, Points, Spline, Tooltip } from 'layerchart/svg';
  import type { ChartState } from 'layerchart';
  import { resolve } from '$app/paths';
  import { scaleLinear } from 'd3-scale';
  import { curveLinear } from 'd3-shape';
  import { onMount, untrack } from 'svelte';
  import type { RisConstants, RisPerformance } from '$lib/types/ris';
  import { benchmarkTotal, calculateRis } from '$lib/utils/ris';
  import { formatLongDate } from '$lib/utils/format';

  let {
    constants,
    points,
    label,
    active = true,
  }: {
    constants: RisConstants;
    points: RisPerformance[];
    label: string;
    active?: boolean;
  } = $props();

  const id = $props.id();
  let context = $state<ChartState<RisPerformance>>();
  let keyboardIndex = $state(-1);
  let selected = $state<RisPerformance | null>(null);
  let interactive = $state(false);
  onMount(() => {
    interactive = true;
  });
  // Render a complete figure in the initial HTML. CSS scales that first frame
  // to the available space; measured dimensions take over during hydration.
  let chartWidth = $state(848);
  let chartHeight = $state(368);

  const selectedPerformances = $derived.by(() => {
    const point = selected;
    return point
      ? points.filter(
          (performance) =>
            performance.bodyweight === point.bodyweight && performance.total === point.total
        )
      : [];
  });

  function select(point: RisPerformance | null) {
    if (point) selected = point;
  }

  function bounds(values: number[], step: number, fallback: [number, number]): [number, number] {
    if (!values.length) return fallback;
    const low = values.reduce((min, value) => Math.min(min, value), Infinity);
    const high = values.reduce((max, value) => Math.max(max, value), -Infinity);
    const start = Math.floor(low / step) * step;
    const end = Math.ceil(high / step) * step;
    return start === end ? [start - step, end + step] : [start, end];
  }

  const xDomain = $derived(
    bounds(
      points.map((point) => point.bodyweight),
      10,
      [40, 140]
    )
  );

  // Sample the formula itself; interpolation must not alter the scientific curve.
  const curve = $derived(
    Array.from({ length: 161 }, (_, index) => {
      const bodyweight = xDomain[0] + ((xDomain[1] - xDomain[0]) * index) / 160;
      return { bodyweight, total: benchmarkTotal(constants, bodyweight) };
    })
  );

  const yDomain = $derived(
    bounds(
      [...points.map((point) => point.total), ...curve.map((point) => point.total)],
      100,
      [0, 600]
    )
  );

  function navigate(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      context?.tooltip.hide();
      keyboardIndex = -1;
      selected = null;
      return;
    }
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      select(context?.tooltip.data ?? null);
      return;
    }
    if (!['ArrowLeft', 'ArrowRight', 'Home', 'End'].includes(event.key) || !points.length) {
      return;
    }
    event.preventDefault();
    if (event.key === 'Home') keyboardIndex = 0;
    else if (event.key === 'End') keyboardIndex = points.length - 1;
    else {
      keyboardIndex = Math.max(
        0,
        Math.min(points.length - 1, keyboardIndex + (event.key === 'ArrowRight' ? 1 : -1))
      );
    }
    context?.tooltip.show({ data: points[keyboardIndex] });
  }

  // Clear selection when data changes or this cached chart becomes inactive.
  $effect(() => {
    void points;
    void active;
    untrack(() => {
      keyboardIndex = -1;
      selected = null;
      context?.tooltip.hide();
    });
  });
</script>

<figure class="ris-chart m-0" aria-labelledby="{id}-caption">
  <div class="mb-3 flex items-center justify-between text-xs text-muted">
    <span>Total, kg</span>
    <span class="font-mono">RIS 100 benchmark</span>
  </div>
  <div
    class="chart-viewport h-[19rem] sm:h-[23rem]"
    bind:clientWidth={chartWidth}
    bind:clientHeight={chartHeight}
  >
    <Chart
      ssr
      width={chartWidth}
      height={chartHeight}
      bind:context
      data={points}
      x="bodyweight"
      y="total"
      xScale={scaleLinear()}
      yScale={scaleLinear()}
      {xDomain}
      {yDomain}
      padding={{ top: 12, right: 14, bottom: 30, left: 42 }}
      axis
      grid={{ x: false, y: true }}
      rule={false}
      clip
      motion="none"
      tooltipContext={interactive ? { mode: 'quadtree', touchEvents: 'pan-y' } : false}
      highlight={{
        lines: false,
        points: { r: 4, fill: 'var(--color-ink)', stroke: 'var(--color-canvas)' },
      }}
      props={{
        svg: {
          viewBox: `0 0 ${chartWidth} ${chartHeight}`,
          preserveAspectRatio: 'none',
        },
        xAxis: {
          tickMarks: false,
          tickSpacing: 70,
          format: (value: number) => String(value),
          tickLabelProps: { fontSize: 11, fill: 'var(--color-secondary)' },
        },
        yAxis: {
          tickMarks: false,
          tickSpacing: 60,
          format: (value: number) => String(value),
          tickLabelProps: { fontSize: 11, fill: 'var(--color-secondary)' },
        },
      }}
      role="group"
      aria-label={label}
      aria-describedby="{id}-instructions"
      tabindex={0}
      onkeydown={navigate}
      onTooltipClick={(_event, { data }) => select(data)}
      onblur={() => context?.tooltip.hide()}
      class="relative max-h-full max-w-full rounded-sm focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-muted"
    >
      {#snippet marks({ context })}
        <Points r={2} fill="var(--color-muted)" opacity={0.45} />
        <Spline
          data={curve}
          curve={curveLinear}
          stroke="var(--color-secondary)"
          strokeWidth={1.75}
          motion="none"
        />
        {#if selected}
          <circle
            cx={context.xScale(selected.bodyweight)}
            cy={context.yScale(selected.total)}
            r="5"
            fill="none"
            stroke="var(--color-ink)"
            stroke-width="1.5"
          />
        {/if}
      {/snippet}
      {#snippet tooltip({ context })}
        <Tooltip.Root
          {context}
          variant="none"
          portal={false}
          motion="none"
          fadeDuration={0}
          class="rounded-md border border-stroke-strong bg-canvas px-3 py-2.5 text-xs text-secondary shadow-sm"
        >
          {#snippet children({ data })}
            <p class="max-w-56 font-medium text-ink">{data.athlete_name}</p>
            <p class="mt-1 max-w-56 text-secondary">{data.competition_name}</p>
            <p class="mt-0.5 mb-3 text-muted">{formatLongDate(data.competition_date)}</p>
            <dl class="grid grid-cols-[1fr_auto] gap-x-6 gap-y-1.5">
              <dt>Bodyweight</dt>
              <dd class="text-right font-mono">{data.bodyweight.toFixed(2)} kg</dd>
              <dt>Total</dt>
              <dd class="text-right font-mono">{data.total.toFixed(2)} kg</dd>
              <dt>RIS</dt>
              <dd class="text-right font-mono text-ink">
                {calculateRis(constants, data.bodyweight, data.total).toFixed(2)}
              </dd>
            </dl>
          {/snippet}
        </Tooltip.Root>
      {/snippet}
    </Chart>
  </div>
  <p class="mt-1 text-right text-xs text-muted">Bodyweight, kg</p>
  <p id="{id}-instructions" class="sr-only">
    Hover or touch to inspect a performance. With the chart focused, use the left and right arrow
    keys to browse performances, Home or End to jump, Enter to select, and Escape to clear the
    selection.
  </p>
  <p class="sr-only" aria-live="polite">
    {#if keyboardIndex >= 0 && points[keyboardIndex]}
      {@const point = points[keyboardIndex]}
      {point.athlete_name}, {point.competition_name}, {formatLongDate(point.competition_date)}.
      Bodyweight {point.bodyweight}
      kg, total {point.total} kg, RIS {calculateRis(
        constants,
        point.bodyweight,
        point.total
      ).toFixed(2)}.
    {/if}
  </p>
  <figcaption id="{id}-caption" class="mt-5 text-xs leading-6 text-muted">
    <span class="font-medium text-secondary">Figure 1.</span>
    Competition total by bodyweight for the selected gender category. Each point represents a recorded
    performance; an athlete may contribute more than one. The solid line denotes the RIS 100 benchmark.
    Performances above and below it have scores greater and less than 100, respectively.
    {#if !points.length}No recorded performances are available for this category.{/if}
  </figcaption>
</figure>

{#if selected && selectedPerformances.length}
  <section aria-label="Selected performances" class="mt-5 border-t border-stroke pt-5">
    <div class="mb-4 flex items-start justify-between gap-4">
      <div class="flex flex-wrap gap-x-5 gap-y-2 text-xs text-secondary">
        <span>Bodyweight <span class="font-mono text-ink">{selected.bodyweight} kg</span></span>
        <span>Total <span class="font-mono text-ink">{selected.total} kg</span></span>
        <span
          >RIS <span class="font-mono text-ink"
            >{calculateRis(constants, selected.bodyweight, selected.total).toFixed(2)}</span
          ></span
        >
      </div>
      <button
        type="button"
        onclick={() => (selected = null)}
        class="shrink-0 text-xs text-secondary underline underline-offset-4 hover:text-ink focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-muted"
        >Clear selection</button
      >
    </div>
    {#if selectedPerformances.length > 1}
      <p class="mb-3 text-xs text-muted">
        {selectedPerformances.length} performances have the same recorded bodyweight and total and overlap
        at this point.
      </p>
    {/if}
    <ul class="divide-y divide-stroke/50">
      {#each selectedPerformances as performance (performance.participant_id)}
        <li
          class="flex flex-col gap-1 py-3 first:pt-0 sm:flex-row sm:items-baseline sm:justify-between sm:gap-6"
        >
          <a
            href={resolve('/athletes/[slug]', { slug: performance.athlete_slug })}
            class="text-sm font-medium text-ink underline decoration-stroke-strong underline-offset-4 hover:text-ink"
            >{performance.athlete_name}</a
          >
          <div class="text-xs leading-6 text-muted sm:text-right">
            <a
              href={resolve('/competitions/[slug]', { slug: performance.competition_slug })}
              class="text-secondary underline decoration-stroke-strong underline-offset-4 hover:text-ink"
              >{performance.competition_name}</a
            >
            <span class="ml-2 whitespace-nowrap"
              >{formatLongDate(performance.competition_date)}</span
            >
          </div>
        </li>
      {/each}
    </ul>
  </section>
{/if}

<style>
  .chart-viewport {
    container-type: size;
  }

  /* Size the first SVG to the viewport even before the library measures it. */
  .chart-viewport :global(.lc-layout-svg) {
    width: 100cqw;
    height: 100cqh;
  }

  .ris-chart {
    --color-surface-100: var(--color-stroke);
    --color-surface-300: var(--color-stroke-strong);
    --color-surface-content: var(--color-secondary);
    --color-primary: var(--color-secondary);
  }

  .ris-chart :global(.lc-axis-tick-label) {
    font-family: var(--font-mono);
  }

  .ris-chart :global(.lc-grid-y-rule) {
    stroke: var(--color-stroke);
    stroke-opacity: 0.7;
  }
</style>
