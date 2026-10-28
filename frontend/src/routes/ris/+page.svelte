<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { TEXT } from '#lib/constants/typography.js';
  import { REPORTED_GLYPH, REPORTED_MARK } from '#lib/constants/table.js';
  import RisCurve from '#lib/components/ui/ris-curve.svelte';
  import RisCategory from '#lib/components/ui/ris-category.svelte';
  import RisSimulator from '#lib/components/ui/ris-simulator.svelte';
  import Seo from '#lib/components/seo.svelte';
  import { absolute, breadcrumbLd } from '#lib/seo/index.js';
  import { Breadcrumb } from '#lib/components/ui/index.js';
  import Message from '#lib/components/ui/message.svelte';
  import { rankingsHref } from '#lib/state/rankings-return.svelte.js';
  import type { PageData } from './$types';

  const i18n = useI18n();

  let { data }: { data: PageData } = $props();

  const current = $derived(data.editions.find((edition) => edition.is_current));

  let sex = $state<'men' | 'women'>('men');
</script>

<Seo
  title={i18n.t('The Relative Index for Streetlifting')}
  description={i18n.t(
    'Understand RIS, calculate your score, and explore Streetlifting competition results across bodyweights.'
  )}
  canonical={absolute('/ris')}
  jsonLd={[
    breadcrumbLd(
      [
        { name: i18n.t('Rankings'), path: '/' },
        { name: 'RIS', path: '/ris' },
      ],
      i18n.locale
    ),
  ]}
/>

<article class="mx-auto max-w-reading px-4 pt-10 pb-16 sm:px-6 sm:pt-12">
  <Breadcrumb items={[{ label: i18n.t('Rankings'), href: rankingsHref() }, { label: 'RIS' }]} />

  <header class="mb-10">
    <h1 class="{TEXT.title} text-ink">{i18n.t('The Relative Index for Streetlifting')}</h1>
    <p class="mt-3 text-base leading-7 text-secondary">
      {i18n.t(
        'The Relative Index for Streetlifting (RIS) compares competition totals across bodyweights within the same gender category. It uses the combined total of Muscle-up, Pull-up, Dips and Squat.'
      )}
    </p>
    <p class="mt-3 text-base leading-7 text-secondary">
      {#snippet authors()}<a
          href="https://warisradji.com/ris/"
          class="text-ink underline underline-offset-4"
          >{i18n.t('Waris Radji and Mathieu Ardoin')}</a
        >{/snippet}
      <Message
        message={'RIS was developed by {authors}, using results from leading Streetlifting athletes.'}
        values={{ authors }}
      />
    </p>
  </header>

  {#if current}
    <RisSimulator edition={current} bind:sex />
  {/if}

  <section aria-labelledby="interpretation-heading" class="mb-10 sm:mb-12">
    <h2 id="interpretation-heading" class="{TEXT.heading} text-ink">
      {i18n.t('What does a score of 100 mean?')}
    </h2>
    <p class="mt-3 {TEXT.body} leading-6 text-secondary">
      {i18n.t(
        'A RIS of 100 means your total matches the formula’s reference total for your bodyweight and category. A score of 90 means you lifted 90% of that reference; 110 means you lifted 110%.'
      )}
    </p>
    <p class="mt-3 {TEXT.body} leading-6 text-secondary">
      {i18n.t(
        'The reference comes from competition results among leading athletes. A score of 100 is not a maximum.'
      )}
    </p>
  </section>

  {#if current}
    <section aria-labelledby="explore-heading" class="mb-10 sm:mb-12">
      <div class="mb-7">
        <h2 id="explore-heading" class="{TEXT.heading} text-ink">
          {i18n.t('Explore competition results')}
        </h2>
        <p class="mt-3 {TEXT.body} leading-6 text-secondary">
          {i18n.t(
            'Each point represents a performance in the archive. The curve shows the total needed to reach 100 RIS at each bodyweight, using the {year} edition.',
            { year: current.year }
          )}
        </p>
        <p class="mt-3 {TEXT.body} leading-6 text-secondary">
          {i18n.t('Select a point to see the athlete, competition and score.')}
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
              label={i18n.t(
                'Competition total by bodyweight with the RIS 100 benchmark, {category}, {year} edition',
                {
                  category: category === 'men' ? i18n.t('Men') : i18n.t('Women'),
                  year: current.year,
                }
              )}
            />
          </div>
        {/each}
      </div>
    </section>
  {/if}

  <section aria-labelledby="comparability-heading" class="mb-10 sm:mb-12">
    <h2 id="comparability-heading" class="{TEXT.heading} text-ink">
      {i18n.t('Why a score may differ from the published result')}
    </h2>
    <p class="mt-3 {TEXT.body} leading-6 text-secondary">
      {i18n.t(
        'The RIS formula changes between editions. OpenStreetlifting uses the current edition wherever possible, including for older competitions, so recalculated scores share the same reference.'
      )}
    </p>

    <p class="mt-3 {TEXT.body} leading-6 text-secondary">
      {#snippet mark()}<span class={REPORTED_MARK}>{REPORTED_GLYPH}</span>{/snippet}
      <Message
        message={'When a score cannot be recalculated, we keep the published value and mark it with {mark}. These scores may use an older edition.'}
        values={{ mark }}
      />
    </p>
  </section>
</article>
