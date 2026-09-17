<script lang="ts">
  import { TEXT } from '$lib/constants/typography';
  import { REPORTED_GLYPH, REPORTED_MARK } from '$lib/constants/table';
  import RisCurve from '$lib/components/ui/ris-curve.svelte';
  import RisCategory from '$lib/components/ui/ris-category.svelte';
  import RisSimulator from '$lib/components/ui/ris-simulator.svelte';
  import Seo from '$lib/components/seo.svelte';
  import { absolute, breadcrumbLd } from '$lib/seo';
  import { Breadcrumb } from '$lib/components/ui';
  import { rankingsHref } from '$lib/state/rankings-return.svelte';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  const current = $derived(data.editions.find((edition) => edition.is_current));

  let sex = $state<'men' | 'women'>('men');
</script>

<Seo
  title="The Relative Index for Streetlifting"
  description="Understand RIS, calculate your score, and explore Streetlifting competition results across bodyweights."
  canonical={absolute('/ris')}
  jsonLd={[
    breadcrumbLd([
      { name: 'Rankings', path: '/' },
      { name: 'RIS', path: '/ris' },
    ]),
  ]}
/>

<article class="mx-auto max-w-reading px-4 pt-10 pb-16 sm:px-6 sm:pt-12">
  <Breadcrumb items={[{ label: 'Rankings', href: rankingsHref() }, { label: 'RIS' }]} />

  <header class="mb-10">
    <h1 class="{TEXT.title} text-ink">The Relative Index for Streetlifting</h1>
    <p class="mt-3 text-base leading-7 text-secondary">
      The Relative Index for Streetlifting (RIS) compares competition totals across bodyweights
      within the same gender category. It uses the combined total of muscle-up, pull-up, dips and
      squat.
    </p>
    <p class="mt-3 text-base leading-7 text-secondary">
      RIS was developed by
      <a href="https://warisradji.com/ris/" class="text-ink underline underline-offset-4"
        >Waris Radji and Mathieu Ardoin</a
      >, using results from leading Streetlifting athletes.
    </p>
  </header>

  {#if current}
    <RisSimulator edition={current} bind:sex />
  {/if}

  <section aria-labelledby="interpretation-heading" class="mb-10 sm:mb-12">
    <h2 id="interpretation-heading" class="{TEXT.heading} text-ink">
      What does a score of 100 mean?
    </h2>
    <p class="mt-3 {TEXT.body} leading-6 text-secondary">
      A RIS of 100 means your total matches the formula’s reference total for your bodyweight and
      category. A score of 90 means you lifted 90% of that reference; 110 means you lifted 110%.
    </p>
    <p class="mt-3 {TEXT.body} leading-6 text-secondary">
      The reference comes from competition results among leading athletes. A score of 100 is not a
      maximum.
    </p>
  </section>

  {#if current}
    <section aria-labelledby="explore-heading" class="mb-10 sm:mb-12">
      <div class="mb-7">
        <h2 id="explore-heading" class="{TEXT.heading} text-ink">Explore competition results</h2>
        <p class="mt-3 {TEXT.body} leading-6 text-secondary">
          Each point represents a performance in the archive. The curve shows the total needed to
          reach 100 RIS at each bodyweight, using the {current.year} edition.
        </p>
        <p class="mt-3 {TEXT.body} leading-6 text-secondary">
          Select a point to see the athlete, competition and score.
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

  <section aria-labelledby="comparability-heading" class="mb-10 sm:mb-12">
    <h2 id="comparability-heading" class="{TEXT.heading} text-ink">
      Why a score may differ from the published result
    </h2>
    <p class="mt-3 {TEXT.body} leading-6 text-secondary">
      The RIS formula changes between editions. OpenStreetlifting uses the current edition wherever
      possible, including for older competitions, so recalculated scores share the same reference.
    </p>

    <p class="mt-3 {TEXT.body} leading-6 text-secondary">
      When a score cannot be recalculated, we keep the published value and mark it with
      <span class={REPORTED_MARK}>{REPORTED_GLYPH}</span>. These scores may use an older edition.
    </p>
  </section>
</article>
