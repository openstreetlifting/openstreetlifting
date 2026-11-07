<script lang="ts" module>
  export type BodyweightPoint = {
    id: string;
    name: string;
    sex: 'men' | 'women';
    bodyweight: number;
    total: number;
  };

  export const MIN_POINTS = 8;
</script>

<script lang="ts">
  import ChartPanel from '#lib/components/ui/chart-panel.svelte';
  import InfoTip from '#lib/components/ui/info-tip.svelte';
  import { useI18n } from '#lib/i18n/index.js';
  import { useWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import { Chart, Spline, Tooltip } from 'layerchart/svg';
  import { scaleLinear } from 'd3-scale';
  import { curveLinear } from 'd3-shape';
  import { onMount } from 'svelte';
  import type { RisFormula } from '#lib/types/ris.js';
  import { benchmarkTotal, calculateRis } from '#lib/utils/ris.js';
  import { kilograms } from '#lib/utils/weight-unit.js';
  import { CHART, TEXT } from '#lib/constants/typography.js';

  const i18n = useI18n();
  const weights = useWeightUnit();

  let { points, formula }: { points: BodyweightPoint[]; formula: RisFormula } = $props();

  let interactive = $state(false);
  onMount(() => {
    interactive = true;
  });
  let chartWidth = $state(848);
  let chartHeight = $state(288);

  const sexes = $derived(
    (['men', 'women'] as const).filter((sex) => points.some((p) => p.sex === sex))
  );

  function bounds(values: number[], step: number): [number, number] {
    const low = Math.floor(Math.min(...values) / step) * step;
    const high = Math.ceil(Math.max(...values) / step) * step;
    return low === high ? [low - step, high + step] : [low, high];
  }

  const xDomain = $derived(
    bounds(
      points.map((p) => p.bodyweight),
      10
    )
  );

  const curves = $derived(
    sexes.map((sex) => ({
      sex,
      line: Array.from({ length: 81 }, (_, index) => {
        const bodyweight = xDomain[0] + ((xDomain[1] - xDomain[0]) * index) / 80;
        return { sex, bodyweight, total: benchmarkTotal(formula.constants[sex], bodyweight) };
      }),
    }))
  );

  const yDomain = $derived(
    bounds(
      [...points.map((p) => p.total), ...curves.flatMap((curve) => curve.line.map((p) => p.total))],
      50
    )
  );

  const above = $derived(
    points.filter((p) => calculateRis(formula.constants[p.sex], p.bodyweight, p.total) >= 100)
      .length
  );

  const byId = $derived(new Map(points.map((point) => [point.id, point])));
  const athlete = (data: unknown) =>
    data && typeof data === 'object' && 'id' in data ? byId.get(String(data.id)) : undefined;

  type BenchmarkPoint = (typeof curves)[number]['line'][number];
  const samples = $derived(new Set<unknown>(curves.flatMap((curve) => curve.line)));
  const benchmark = (data: unknown) => (samples.has(data) ? (data as BenchmarkPoint) : undefined);

  const sexLabel = (sex: 'men' | 'women') => (sex === 'men' ? i18n.t('Men') : i18n.t('Women'));
</script>

<ChartPanel title={i18n.t('Total by bodyweight')}>
  {#snippet controls()}
    <InfoTip label={i18n.t('About total by bodyweight')}>
      <p class="font-medium text-ink">{i18n.t('Total by bodyweight')}</p>
      <p class="mt-3">
        {i18n.t(
          'Each point is an athlete with a recorded bodyweight. The lines show the total needed for a RIS of 100 at each bodyweight, using the {year} edition.',
          { year: formula.year }
        )}
      </p>
      <p class="mt-3">
        {i18n.t('Points above their line scored more than 100 RIS.')}
      </p>
    </InfoTip>
  {/snippet}

  <ul class="{TEXT.meta} mb-2 flex flex-wrap gap-x-4 gap-y-1 text-secondary">
    {#each sexes as sex (sex)}
      <li class="flex items-center gap-2">
        <svg class="size-2" viewBox="0 0 8 8" aria-hidden="true">
          <circle
            cx="4"
            cy="4"
            r="3"
            fill={sex === 'men' ? 'var(--color-ink)' : 'none'}
            stroke="var(--color-ink)"
            stroke-width="1.5"
          />
        </svg>
        {sexLabel(sex)}
      </li>
    {/each}
    {#each sexes as sex (sex)}
      <li class="flex items-center gap-2">
        <span
          class="w-4 border-t-2 border-secondary {sex === 'women' ? 'border-dashed' : ''}"
          aria-hidden="true"
        ></span>
        {i18n.t('RIS 100, {sex}', { sex: sexLabel(sex).toLowerCase() })}
      </li>
    {/each}
  </ul>

  <div class="h-72 w-full min-w-0" bind:clientWidth={chartWidth} bind:clientHeight={chartHeight}>
    <Chart
      ssr
      width={chartWidth}
      height={chartHeight}
      data={points}
      x="bodyweight"
      y="total"
      xScale={scaleLinear()}
      yScale={scaleLinear()}
      {xDomain}
      {yDomain}
      padding={{ top: 8, right: 12, bottom: 28, left: 44 }}
      axis
      grid={{ x: false, y: true }}
      rule={false}
      clip
      tooltipContext={interactive ? { mode: 'quadtree', touchEvents: 'pan-y' } : false}
      props={{
        svg: { viewBox: `0 0 ${chartWidth} ${chartHeight}`, preserveAspectRatio: 'none' },
        xAxis: {
          tickMarks: false,
          ticks: scaleLinear()
            .domain(xDomain.map((value) => weights.value(value)))
            .ticks(Math.max(2, Math.floor(chartWidth / 90)))
            .map((value) => kilograms(value, weights.unit)),
          format: (value: number) => weights.format(value, i18n.locale),
          tickLabelProps: { fontSize: 11, class: CHART.tick },
        },
        yAxis: {
          tickMarks: false,
          ticks: scaleLinear()
            .domain(yDomain.map((value) => weights.value(value)))
            .ticks(5)
            .map((value) => kilograms(value, weights.unit)),
          format: (value: number) => weights.format(value, i18n.locale),
          tickLabelProps: { fontSize: 11, class: CHART.tick },
        },
      }}
      role="img"
      aria-label={i18n.t(
        'Total by bodyweight for {count} athletes, {above} of them above a RIS of 100.',
        { count: i18n.number(points.length, 0), above: i18n.number(above, 0) }
      )}
      class="relative max-h-full max-w-full"
    >
      {#snippet marks({ context })}
        {#each curves as curve (curve.sex)}
          <Spline
            data={curve.line}
            curve={curveLinear}
            stroke="var(--color-secondary)"
            strokeWidth={1.5}
            class={curve.sex === 'women' ? '[stroke-dasharray:4_4]' : ''}
            motion="none"
          />
        {/each}
        {#each points as point (point.id)}
          <circle
            cx={context.xScale(point.bodyweight)}
            cy={context.yScale(point.total)}
            r="3"
            fill={point.sex === 'men' ? 'var(--color-ink)' : 'var(--color-canvas)'}
            stroke="var(--color-ink)"
            stroke-width="1.5"
          />
        {/each}
      {/snippet}
      {#snippet tooltip({ context })}
        <Tooltip.Root
          {context}
          variant="none"
          portal={false}
          motion="none"
          fadeDuration={0}
          class="{CHART.tooltip} text-meta text-secondary"
        >
          {#snippet children({ data })}
            {@const performance = athlete(data)}
            {@const point = performance ?? benchmark(data)}
            {@const digits = performance ? undefined : 2}
            {#if point}
              <p class="max-w-56 font-medium text-ink">
                {performance
                  ? performance.name
                  : i18n.t('RIS 100, {sex}', { sex: sexLabel(point.sex).toLowerCase() })}
              </p>
              <dl class="mt-2 grid grid-cols-[1fr_auto] gap-x-6 gap-y-1">
                <dt>{i18n.t('Bodyweight')}</dt>
                <dd class="text-right font-mono">
                  {weights.format(point.bodyweight, i18n.locale, digits)}
                  {weights.unit}
                </dd>
                <dt>{i18n.t('Total')}</dt>
                <dd class="text-right font-mono">
                  {weights.format(point.total, i18n.locale, digits)}
                  {weights.unit}
                </dd>
                <dt>RIS</dt>
                <dd class="text-right font-mono text-ink">
                  {i18n.number(
                    calculateRis(formula.constants[point.sex], point.bodyweight, point.total),
                    2
                  )}
                </dd>
              </dl>
            {/if}
          {/snippet}
        </Tooltip.Root>
      {/snippet}
    </Chart>
  </div>
  <p class="{TEXT.caption} mt-1 flex justify-between gap-4">
    <span>{i18n.t('Total, {unit}', { unit: weights.unit })}</span>
    <span>{i18n.t('Bodyweight, {unit}', { unit: weights.unit })}</span>
  </p>
</ChartPanel>
