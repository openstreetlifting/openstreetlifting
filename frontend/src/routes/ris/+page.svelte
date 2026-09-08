<script lang="ts">
  import 'katex/dist/katex.min.css';
  import { TEXT } from '$lib/constants/typography';
  import { REPORTED_GLYPH, REPORTED_MARK } from '$lib/constants/table';
  import RisCurve from '$lib/components/ui/ris-curve.svelte';
  import RisCategory from '$lib/components/ui/ris-category.svelte';
  import RisSimulator from '$lib/components/ui/ris-simulator.svelte';
  import Seo from '$lib/components/seo.svelte';
  import { absolute } from '$lib/seo';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  const current = $derived(data.editions.find((edition) => edition.is_current));

  let sex = $state<'men' | 'women'>('men');
</script>

<Seo
  title="The Relative Index for Streetlifting"
  description="The Relative Index for Streetlifting: score interpretation, an interactive calculator, and archived performances in relation to the bodyweight benchmark."
  canonical={absolute('/ris')}
/>

<article class="mx-auto max-w-reading px-4 pt-10 pb-16 sm:px-6 sm:pt-12">
  <header class="mb-10">
    <h1 class="{TEXT.title} text-ink">The Relative Index for Streetlifting</h1>
    <p class="mt-3 text-base leading-7 text-secondary">
      The Relative Index for Streetlifting (RIS) enables comparison of athletes’ performances across
      weight classes. Developed by Waris Radji and Mathieu Ardoin, it accounts for bodyweight when
      evaluating a competition total. The formula applies only to the full MPDS total, combining
      muscle-up, pull-up, dips, and squat. Separate models are used for men and women, and their
      coefficients are currently updated each year. Athletes are therefore compared within the same
      gender category using the same formula edition.
    </p>
    <p class="mt-3 text-base leading-7 text-secondary">
      RIS is based on the relationship between bodyweight and performance in a selected group of
      leading athletes.
    </p>
  </header>

  {#if current}
    <RisSimulator edition={current} bind:sex />
  {/if}

  <section aria-labelledby="comparability-heading" class="mb-10 sm:mb-12">
    <h2 id="comparability-heading" class="{TEXT.heading} text-ink">
      Recalculating the OpenStreetlifting archive
    </h2>
    <p class="mt-3 {TEXT.body} leading-6 text-secondary">
      OpenStreetlifting recalculates eligible performances throughout the archive with the current
      RIS edition, regardless of the competition date. Using one edition gives computed scores a
      consistent reference within each gender category. Results may differ from federation
      publications that used another edition.
    </p>
    <p class="mt-3 {TEXT.body} leading-6 text-secondary">
      Each score is calculated from the athlete’s bodyweight and competition total. When bodyweight
      is missing, it may be estimated from the published score using the original formula edition.
    </p>
    <p class="mt-3 {TEXT.body} leading-6 text-secondary">
      Scores that cannot be recalculated retain the reported value, marked
      <span class={REPORTED_MARK}>{REPORTED_GLYPH}</span>.
    </p>
  </section>

  {#if current}
    <section aria-labelledby="explore-heading" class="mb-10 sm:mb-12">
      <div class="mb-7">
        <h2 id="explore-heading" class="{TEXT.heading} text-ink">
          Observed performances and the benchmark
        </h2>
        <p class="mt-3 {TEXT.body} leading-6 text-secondary">
          Figure 1 shows archived totals in relation to bodyweight and the {current.year} reference curve.
          Select a point to inspect the athlete, competition, and calculated score.
        </p>
      </div>
      <div class="mb-6"><RisCategory bind:sex /></div>

      <!-- Keep both datasets rendered: switching only changes visibility, not thousands of points.
           Grid overlap preserves dimensions for the inactive chart's responsive measurements. -->
      <div class="grid">
        {#each ['men', 'women'] as const as category (category)}
          <div
            class="min-w-0 [grid-area:1/1]"
            class:invisible={sex !== category}
            inert={sex !== category}
            aria-hidden={sex !== category}
          >
            <RisCurve
              constants={current.constants[category]}
              points={data.distribution[category]}
              active={sex === category}
              label="Competition total by bodyweight with the RIS 100 benchmark, {category}, {current.year} edition"
            />
          </div>
        {/each}
      </div>
    </section>
  {/if}

  <section aria-labelledby="interpretation-heading" class="mb-10 sm:mb-12">
    <h2 id="interpretation-heading" class="{TEXT.heading} text-ink">
      What a RIS of 100 represents
    </h2>
    <p class="mt-3 {TEXT.body} leading-6 text-secondary">
      A RIS of 100 corresponds to the reference performance at an athlete’s bodyweight and gender
      category. This reference, <span class="font-mono">f(BW)</span>, comes from the curve fitted to
      the selected group of athletes. The score expresses the athlete’s total as a percentage of
      that reference:
    </p>

    <!-- KaTeX output from a fixed, server-owned expression. -->
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    <div class="my-5 overflow-x-auto py-2 text-ink">{@html data.scoreEquation}</div>

    <p class="{TEXT.body} leading-6 text-secondary">
      A total equal to the reference produces a score of 100. Higher and lower totals produce
      proportionally higher and lower scores. Athletes of different bodyweights receive the same RIS
      when their totals represent the same percentage of their respective benchmarks.
    </p>
  </section>
</article>
