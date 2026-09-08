<script lang="ts">
  import InfoTip from '$lib/components/ui/info-tip.svelte';
  import type { StrengthProfile } from '$lib/types/athlete';
  import { Chart, Layer, Axis, Spline, Points, Circle } from 'layerchart/svg';
  import { Tooltip } from 'layerchart';
  import { scalePoint } from 'd3-scale';
  import { CHART, TEXT } from '$lib/constants/typography';
  import { formatWeight } from '$lib/utils/format';

  let { profile }: { profile: StrengthProfile | null | undefined } = $props();
  let chartWidth = $state<number>();
  const directions = ['Muscle-up', 'Pull-up', 'Squat', 'Dips'];
  const levels = [20, 40, 60, 80, 100];
  const lifts = $derived(
    profile?.lifts.toSorted(
      (a, b) => directions.indexOf(a.movement_name) - directions.indexOf(b.movement_name)
    ) ?? []
  );
  const scored = $derived(lifts.filter((lift) => lift.percentile !== null));
  const complete = $derived(scored.length === directions.length);
  const outline = $derived([...lifts, lifts[0]]);
  const percent = new Intl.NumberFormat('en', { maximumFractionDigits: 1 });
</script>

<section class="@container min-w-0 max-w-reading" aria-labelledby="strength-heading">
  <div class="mb-4 flex min-h-8 items-center gap-2">
    <h2 id="strength-heading" class="{TEXT.heading} text-ink">Strength profile</h2>
    {#if profile}
      <InfoTip label="About Strength profile">
        <p class="font-medium text-ink">Strength profile</p>
        <p class="mt-2">{profile.category} · All time</p>
        <p class="mt-3">
          Compares each best lift with other athletes’ bests. Percentile is the percentage of peers
          below that result, counting ties as half.
        </p>
        <p class="mt-3">Peer counts vary by lift.</p>
      </InfoTip>
    {/if}
  </div>
  {#if profile}
    <div
      class="grid items-center gap-4 @min-[36rem]:grid-cols-[minmax(0,1fr)_auto] @min-[36rem]:gap-6 2xl:pt-15"
    >
      <div class="mx-auto h-80 w-full min-w-0 max-w-md" bind:clientWidth={chartWidth}>
        <Chart
          ssr
          width={chartWidth || 400}
          class="relative w-full! overflow-hidden [container-type:inline-size]"
          data={lifts}
          x="movement_name"
          y="percentile"
          xScale={scalePoint()}
          xDomain={directions}
          xRange={[0, Math.PI * 1.5]}
          yDomain={[0, 100]}
          yRange={({ width, height }) => [0, Math.min(width, height) / 2]}
          radial
          height={320}
          padding={{ left: 62, right: 62, top: 38, bottom: 38 }}
          tooltipContext={{ mode: 'manual' }}
        >
          {#snippet children({ context })}
            <Layer
              viewBox={`0 0 ${context.containerWidth} ${context.containerHeight}`}
              style="width: 100cqw; height: 100%"
              center
              role="group"
              aria-label="Lift percentiles compared with other athletes in {profile.category}"
            >
              <Axis
                placement="angle"
                grid={{ class: 'stroke-stroke' }}
                stroke="var(--color-stroke)"
                tickLabelProps={{ class: CHART.label }}
              />
              {#each levels as level (level)}
                <Spline data={outline} y={() => level} class="stroke-stroke" strokeWidth={1} />
              {/each}
              {#if complete}
                <Spline
                  data={outline}
                  class="fill-chart-athlete/15 stroke-chart-athlete"
                  strokeWidth={2}
                />
              {/if}
              <Axis
                placement="radius"
                ticks={levels}
                tickMarks={false}
                tickLabelProps={{
                  dx: 24,
                  dy: -8,
                  textAnchor: 'start',
                  class: CHART.tick,
                }}
              />
              <Points data={scored}>
                {#snippet children({ points })}
                  {#each points as point (point.data.movement_name)}
                    <Circle
                      cx={point.x}
                      cy={point.y}
                      r={4}
                      class="fill-chart-athlete stroke-canvas"
                      strokeWidth={2}
                    />
                    <Circle
                      cx={point.x}
                      cy={point.y}
                      r={16}
                      fill="transparent"
                      role="button"
                      tabindex={0}
                      class="cursor-pointer outline-none focus-visible:stroke-focus"
                      strokeWidth={2}
                      aria-label="{point.data.movement_name}: percentile {percent.format(
                        point.data.percentile
                      )}"
                      onclick={() => context.tooltip.show({ data: point.data })}
                      onfocus={() => context.tooltip.show({ data: point.data })}
                      onpointerenter={(event) => {
                        if (event.pointerType !== 'touch')
                          context.tooltip.show({ data: point.data });
                      }}
                      onpointerleave={(event) => {
                        if (event.pointerType !== 'touch') context.tooltip.hide();
                      }}
                      onkeydown={(event) => {
                        if (event.key === 'Escape') context.tooltip.hide();
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
              motion="none"
              fadeDuration={0}
              variant="none"
              classes={{
                content:
                  'max-w-56 rounded-lg border border-stroke-strong bg-surface p-3 text-ink shadow-lg shadow-overlay/40',
              }}
            >
              {#snippet children({ data: lift })}
                <div role="tooltip">
                  <p class="text-sm font-medium">
                    {lift.movement_name} · {formatWeight(lift.value)} kg
                  </p>
                  <p class="mt-1 font-mono text-lg font-bold">
                    {percent.format(lift.percentile)} percentile
                  </p>
                  <p class="mt-1 text-xs text-muted">
                    Compared with {lift.field} other {lift.field === 1 ? 'athlete' : 'athletes'}
                  </p>
                </div>
              {/snippet}
            </Tooltip.Root>
          {/snippet}
        </Chart>
      </div>
      <dl
        aria-label="Lift percentiles"
        class="mx-auto grid w-full max-w-xs gap-3 text-sm @min-[36rem]:w-auto"
      >
        {#each lifts as lift (lift.movement_name)}
          <div class="flex items-baseline justify-between gap-5 whitespace-nowrap">
            <dt class="text-secondary">{lift.movement_name}</dt>
            <dd class="text-right">
              {#if lift.percentile !== null}
                <span class="font-mono font-semibold text-ink"
                  >{percent.format(lift.percentile)}</span
                >
                <span class="text-xs text-muted">percentile</span>
              {:else}
                <span class="text-muted" aria-label="Not enough data">—</span>
              {/if}
            </dd>
          </div>
        {/each}
      </dl>
    </div>
    {#if scored.some((lift) => lift.field < 5)}
      <p class="mt-3 text-xs text-muted">
        Limited comparison: fewer than five peers for some lifts.
      </p>
    {/if}
    {#if !complete}
      <p class="mt-3 text-xs text-muted">Not enough data to score all four lifts.</p>
    {/if}
  {:else}
    <p class="border-y border-stroke py-8 text-sm text-secondary">
      A completed competition is needed to establish a comparison category.
    </p>
  {/if}
</section>
