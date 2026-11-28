<script lang="ts" module>
  export const MIN_ATTEMPTS = 8;
</script>

<script lang="ts">
  import ChartPanel from '#lib/components/ui/chart-panel.svelte';
  import InfoTip from '#lib/components/ui/info-tip.svelte';
  import { useI18n } from '#lib/i18n/index.js';
  import { Chart, Tooltip } from 'layerchart/svg';
  import { scaleBand, scaleLinear } from 'd3-scale';
  import { onMount } from 'svelte';
  import type { AttemptRate, MovementAttempts } from '#lib/utils/competition-results.js';
  import { CHART, TEXT } from '#lib/constants/typography.js';
  import { AXIS, BAND_AXIS, PLOT_PADDING } from '#lib/utils/chart.js';
  import { liftTone } from '#lib/constants/lift.js';

  const i18n = useI18n();

  let { rates }: { rates: MovementAttempts[] } = $props();

  let interactive = $state(false);
  onMount(() => {
    interactive = true;
  });
  let chartWidth = $state(848);
  let chartHeight = $state(448);

  const COLUMN_PADDING = 0.3;
  const BAR_WIDTH = 20;
  const BAR_GAP = 2;

  const ATTEMPTS = [
    { number: 1, fill: 'var(--color-faint)', strength: 65 },
    { number: 2, fill: 'var(--color-secondary)', strength: 82 },
    { number: 3, fill: 'var(--color-ink)', strength: 100 },
  ] as const;

  function barFill(movement: string, attempt: number): string {
    const { fill, strength } = ATTEMPTS[attempt - 1];
    const tone = liftTone(movement);
    return tone ? `color-mix(in oklab, ${tone} ${strength}%, var(--color-surface))` : fill;
  }

  const percent = $derived(
    new Intl.NumberFormat(i18n.locale, { style: 'percent', maximumFractionDigits: 0 })
  );
  const share = (rate: AttemptRate) => (rate.taken > 0 ? rate.made / rate.taken : null);

  const summary = $derived(
    rates
      .map((column) =>
        [
          i18n.movement(column.movement),
          ...column.rates.map((rate) => {
            const value = share(rate);
            return `${i18n.t('Attempt {number}', { number: rate.attempt })} ${value === null ? '—' : percent.format(value)}`;
          }),
        ].join(', ')
      )
      .join('; ')
  );
</script>

<ChartPanel title={i18n.t('Attempt success')}>
  {#snippet controls()}
    <InfoTip label={i18n.t('About attempt success')}>
      <p class="font-medium text-ink">{i18n.t('Attempt success')}</p>
      <p class="mt-3">
        {i18n.t('The share of first, second and third attempts judged good, for each movement.')}
      </p>
    </InfoTip>
  {/snippet}

  <ul class="{TEXT.meta} mb-2 flex flex-wrap gap-x-4 gap-y-1 text-secondary">
    {#each ATTEMPTS as attempt (attempt.number)}
      <li class="flex items-center gap-2">
        <span class="size-2 rounded-sm" style:background={attempt.fill} aria-hidden="true"></span>
        {i18n.t('Attempt {number}', { number: attempt.number })}
      </li>
    {/each}
  </ul>

  <div
    class="{CHART.plot} w-full min-w-0"
    bind:clientWidth={chartWidth}
    bind:clientHeight={chartHeight}
  >
    <Chart
      ssr
      width={chartWidth}
      height={chartHeight}
      data={rates}
      x="movement"
      y={() => 1}
      xScale={scaleBand()
        .paddingInner(COLUMN_PADDING)
        .paddingOuter(COLUMN_PADDING / 2)}
      yScale={scaleLinear()}
      yDomain={[0, 1]}
      padding={{ ...PLOT_PADDING, top: 8, bottom: 28 }}
      axis
      grid={{ x: false, y: true }}
      rule={false}
      tooltipContext={interactive ? { mode: 'manual', touchEvents: 'pan-y' } : false}
      props={{
        svg: { viewBox: `0 0 ${chartWidth} ${chartHeight}`, preserveAspectRatio: 'none' },
        xAxis: {
          ...BAND_AXIS,
          format: (movement: string) => i18n.movement(movement),
        },
        yAxis: {
          ...AXIS,
          ticks: [0, 0.25, 0.5, 0.75, 1],
          format: (value: number) => percent.format(value),
        },
      }}
      role="img"
      aria-label={summary}
      class="relative max-h-full max-w-full"
    >
      {#snippet marks({ context })}
        {@const band = context.xScale.bandwidth?.() ?? 0}
        {@const group = Math.min(band, BAR_WIDTH * 3 + BAR_GAP * 2)}
        {@const inner = scaleBand<number>()
          .domain([1, 2, 3])
          .range([(band - group) / 2, (band + group) / 2])
          .paddingInner(BAR_GAP / (group / 3))}
        {#each rates as column (column.movement)}
          {@const start = context.xScale(column.movement) ?? 0}
          {#each column.rates as rate (rate.attempt)}
            {@const value = share(rate)}
            {#if value !== null}
              <rect
                x={start + (inner(rate.attempt) ?? 0)}
                y={context.yScale(value)}
                width={inner.bandwidth()}
                height={Math.max(0, context.yScale(0) - context.yScale(value))}
                rx="2"
                fill={barFill(column.movement, rate.attempt)}
                role="presentation"
                onpointermove={(event) => context.tooltip.show(event, column)}
                onpointerleave={() => context.tooltip.hide()}
              />
            {/if}
          {/each}
        {/each}
      {/snippet}
      {#snippet tooltip({ context })}
        <Tooltip.Root
          {context}
          variant="none"
          portal={false}
          motion="none"
          fadeDuration={0}
          class={CHART.tooltip}
        >
          {#snippet children({ data })}
            <p class={CHART.tooltipTitle}>
              <span>{i18n.movement(data.movement)}</span>
            </p>
            <dl class={CHART.tooltipRows}>
              {#each data.rates as rate (rate.attempt)}
                {@const value = share(rate)}
                <dt>{i18n.t('Attempt {number}', { number: rate.attempt })}</dt>
                <dd class={CHART.tooltipValue}>
                  {value === null ? '—' : percent.format(value)}
                </dd>
              {/each}
            </dl>
          {/snippet}
        </Tooltip.Root>
      {/snippet}
    </Chart>
  </div>
</ChartPanel>
