<script lang="ts">
  import ChartPanel from '#lib/components/ui/chart-panel.svelte';
  import InfoTip from '#lib/components/ui/info-tip.svelte';
  import { useI18n } from '#lib/i18n/index.js';
  import { useWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import type { LiftBalance } from '#lib/types/athlete.js';
  import { Chart, Layer, Axis, Spline, Points, Circle } from 'layerchart/svg';
  import { Tooltip, type ChartState } from 'layerchart';
  import { scalePoint } from 'd3-scale';
  import { CHART, TEXT } from '#lib/constants/typography.js';
  import Text from '#lib/components/ui/text.svelte';

  const MIN_PEERS = 10;
  const RANGE = 3;
  const DIRECTIONS = ['Muscle-up', 'Pull-up', 'Squat', 'Dips'];
  const RINGS = [-RANGE / 2, RANGE / 2, RANGE];

  const i18n = useI18n();
  const weights = useWeightUnit();

  let { balance }: { balance: LiftBalance | null | undefined } = $props();
  let chartWidth = $state<number>();

  const lifts = $derived(
    (balance?.lifts ?? [])
      .toSorted((a, b) => DIRECTIONS.indexOf(a.movement_name) - DIRECTIONS.indexOf(b.movement_name))
      .map((lift) => {
        const { share, class_share: average, class_spread: spread } = lift;
        const gap = average === null ? null : share - average;
        const distance = gap === null || !spread ? null : gap / spread;
        return {
          ...lift,
          gap,
          position: distance === null ? 0 : Math.max(-RANGE, Math.min(RANGE, distance)),
        };
      })
  );
  const comparable = $derived(
    (balance?.field ?? 0) >= MIN_PEERS && lifts.every((lift) => lift.class_spread)
  );
  const outline = $derived([...lifts, lifts[0]]);
  let chartContext = $state<ChartState<(typeof lifts)[number]>>();

  const percent = $derived(
    new Intl.NumberFormat(i18n.locale, {
      style: 'percent',
      minimumFractionDigits: 1,
      maximumFractionDigits: 1,
    })
  );
  const signedPercent = $derived(
    new Intl.NumberFormat(i18n.locale, {
      style: 'percent',
      signDisplay: 'exceptZero',
      minimumFractionDigits: 1,
      maximumFractionDigits: 1,
    })
  );
  const share = (value: number) => percent.format(value / 100);
  const roomy = $derived((chartWidth ?? 0) >= 400);
  const axisLabel = (movement: string) => {
    const gap = lifts.find((lift) => lift.movement_name === movement)?.gap;
    const name = i18n.movement(movement);
    if (!roomy || gap === null || gap === undefined) return name;
    return `${name} ${signedPercent.format(gap / 100)}`;
  };
  const weightClass = $derived(
    weights
      .category(balance?.category, i18n.locale)
      .match(/[+-]\d+(?:[.,]\d+)?\s*(?:kg|lb)?$/i)?.[0] ?? null
  );
  const reference = $derived(
    weightClass ? i18n.t('Reference ({class})', { class: weightClass }) : i18n.t('Reference')
  );
</script>

<ChartPanel title={i18n.t('Lift balance')}>
  {#snippet controls()}
    {#if balance}
      <InfoTip label={i18n.t('About lift balance')}>
        <p class="font-medium text-ink">{i18n.t('Lift balance')}</p>
        <p class="mt-3">
          {i18n.t(
            'How this athlete’s best total is split across the four lifts, compared with others in their weight class.'
          )}
        </p>
        <p class="mt-3">
          {i18n.t(
            'Further out means that lift takes a bigger share of the total than usual, not that it is better.'
          )}
        </p>
        {#if comparable}
          <p class="mt-3">
            {i18n.t('Reference: {count} {athletes} in {category}', {
              count: i18n.number(balance.field, 0),
              athletes: i18n.count('athlete', balance.field),
              category: weights.category(balance.category, i18n.locale),
            })}
          </p>
        {/if}
        <p class="mt-3">
          {i18n.t('Based on {total} {unit} at {competition}', {
            total: weights.format(balance.total, i18n.locale),
            unit: weights.unit,
            competition: balance.competition_name,
          })}
        </p>
      </InfoTip>
    {/if}
  {/snippet}

  {#if balance}
    <div class="grid items-center gap-6 @min-[36rem]:grid-cols-[minmax(0,1fr)_auto]">
      {#if comparable}
        <div class="min-w-0">
          <ul class="{TEXT.meta} mb-2 flex flex-wrap justify-center gap-x-4 gap-y-1 text-secondary">
            <li class="flex items-center gap-2">
              <span class="w-4 border-t-2 border-chart-athlete" aria-hidden="true"></span>
              {i18n.t('Athlete')}
            </li>
            <li class="flex items-center gap-2">
              <span class="w-4 border-t-2 border-dashed border-muted" aria-hidden="true"></span>
              {reference}
            </li>
          </ul>
          <div
            class="mx-auto h-72 w-full min-w-0 max-w-md"
            bind:clientWidth={chartWidth}
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
              class="relative w-full! overflow-hidden [container-type:inline-size]"
              data={lifts}
              x="movement_name"
              y="position"
              xScale={scalePoint()}
              xDomain={DIRECTIONS}
              xRange={[0, Math.PI * 1.5]}
              yDomain={[-RANGE, RANGE]}
              yRange={({ width, height }) => [0, Math.min(width, height) / 2]}
              radial
              height={288}
              padding={roomy
                ? { left: 84, right: 84, top: 30, bottom: 30 }
                : { left: 56, right: 56, top: 30, bottom: 30 }}
              tooltipContext={{ mode: 'manual' }}
            >
              {#snippet children({ context })}
                <Layer
                  viewBox={`0 0 ${context.containerWidth} ${context.containerHeight}`}
                  style="width: 100cqw; height: 100%"
                  center
                  role="group"
                  aria-label={i18n.t('Lift balance compared with other athletes in {category}', {
                    category: weights.category(balance.category, i18n.locale),
                  })}
                >
                  <Axis
                    placement="angle"
                    format={axisLabel}
                    grid={{ class: 'stroke-stroke' }}
                    stroke="var(--color-stroke)"
                    tickLabelProps={{ fontSize: 11, class: CHART.label }}
                  />
                  {#each RINGS as ring (ring)}
                    <Spline data={outline} y={() => ring} class="stroke-stroke" strokeWidth={1} />
                  {/each}
                  <Spline
                    data={outline}
                    y={() => 0}
                    class="stroke-muted [stroke-dasharray:4_4]"
                    strokeWidth={1.5}
                  />
                  <Spline
                    data={outline}
                    class="fill-chart-athlete/10 stroke-chart-athlete"
                    strokeWidth={2}
                  />
                  <Points data={lifts}>
                    {#snippet children({ points: dots })}
                      {#each dots as dot (dot.data.movement_name)}
                        <Circle
                          cx={dot.x}
                          cy={dot.y}
                          r={4}
                          class="fill-chart-athlete stroke-canvas"
                          strokeWidth={2}
                        />
                        <Circle
                          cx={dot.x}
                          cy={dot.y}
                          r={16}
                          fill="transparent"
                          role="button"
                          tabindex={0}
                          class="cursor-pointer outline-none focus-visible:stroke-focus"
                          strokeWidth={2}
                          aria-label={i18n.t('{movement}: {share} of total, reference {average}', {
                            movement: i18n.movement(dot.data.movement_name),
                            share: share(dot.data.share),
                            average: share(dot.data.class_share ?? 0),
                          })}
                          onclick={() => context.tooltip.show({ data: dot.data })}
                          onfocus={() => context.tooltip.show({ data: dot.data })}
                          onpointerenter={(event) => {
                            if (event.pointerType !== 'touch')
                              context.tooltip.show({ data: dot.data });
                          }}
                          onpointerleave={(event) => {
                            if (event.pointerType !== 'touch') context.tooltip.hide();
                          }}
                          onkeydown={(event) => {
                            if (event.key === 'Escape') context.tooltip.hide();
                            if (event.key === 'Enter' || event.key === ' ') {
                              event.preventDefault();
                              context.tooltip.show({ data: dot.data });
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
                  motion="none"
                  fadeDuration={0}
                  variant="none"
                  classes={{ content: CHART.tooltip }}
                >
                  {#snippet children({ data: lift })}
                    <div role="tooltip">
                      <p class="text-body font-medium">
                        {i18n.movement(lift.movement_name)} · {weights.format(
                          lift.value,
                          i18n.locale
                        )}
                        {weights.unit}
                      </p>
                      <p class="mt-1 font-mono text-subtitle font-bold">
                        {share(lift.share)}
                      </p>
                      <p class="mt-1 text-meta text-muted">
                        {i18n.t('of total · reference {average}', {
                          average: share(lift.class_share ?? 0),
                        })}
                      </p>
                    </div>
                  {/snippet}
                </Tooltip.Root>
              {/snippet}
            </Chart>
          </div>
        </div>
      {/if}

      <dl
        aria-label={i18n.t('Lift shares of best total')}
        class="grid grid-cols-1 gap-x-6 gap-y-4 @min-[20rem]:grid-cols-2 @min-[36rem]:w-56 @min-[36rem]:grid-cols-1 @min-[36rem]:gap-0 @min-[36rem]:divide-y @min-[36rem]:divide-stroke"
      >
        {#each lifts as lift (lift.movement_name)}
          <div class="min-w-0 @min-[36rem]:py-1.5 @min-[36rem]:first:pt-0 @min-[36rem]:last:pb-0">
            <Text as="dt" variant="fieldLabel">{i18n.movement(lift.movement_name)}</Text>
            <dd>
              {#if comparable && lift.class_share !== null && lift.gap !== null}
                <Text as="span" variant="figure" class="block leading-8 tabular-nums">
                  {signedPercent.format(lift.gap / 100)}
                </Text>
                <Text as="span" variant="caption" class="block tabular-nums">
                  {i18n.t('Athlete: {share} of total', { share: share(lift.share) })}
                </Text>
                <Text as="span" variant="caption" class="block tabular-nums">
                  {i18n.t('Reference: {share} of total', { share: share(lift.class_share) })}
                </Text>
              {:else}
                <Text as="span" variant="figure" class="block leading-8 tabular-nums">
                  {share(lift.share)}
                </Text>
                <Text as="span" variant="caption" class="block">{i18n.t('of total')}</Text>
              {/if}
            </dd>
          </div>
        {/each}
      </dl>
    </div>

    {#if !comparable}
      <Text variant="caption" class="mt-3">
        {i18n.t('Too few athletes in this weight class to compare.')}
      </Text>
    {/if}
  {:else}
    <p class="border-y border-stroke py-8 text-body text-secondary">
      {i18n.t('A completed competition with all four lifts is needed to show lift balance.')}
    </p>
  {/if}
</ChartPanel>
