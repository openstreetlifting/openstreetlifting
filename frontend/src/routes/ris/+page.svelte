<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { REPORTED_GLYPH, REPORTED_MARK } from '#lib/constants/table.js';
  import RisCurve from '#lib/components/ui/ris-curve.svelte';
  import RisCategory from '#lib/components/ui/ris-category.svelte';
  import RisSimulator from '#lib/components/ui/ris-simulator.svelte';
  import Seo from '#lib/components/seo.svelte';
  import { absolute } from '#lib/seo/index.js';
  import { Text } from '#lib/components/ui/index.js';
  import Message from '#lib/components/ui/message.svelte';
  import type { PageData } from './$types';
  import { page } from '$app/state';
  import { goto } from '$app/navigation';
  import { readRisScenario, writeRisScenario, type RisScenario } from '#lib/utils/ris-scenario.js';

  const i18n = useI18n();

  let { data }: { data: PageData } = $props();

  const current = $derived(data.editions.find((edition) => edition.is_current));

  const scenarioUrl = $derived(page.shallow?.url ?? page.url);
  const scenario = $derived(readRisScenario(scenarioUrl.searchParams));

  function updateScenario(next: RisScenario) {
    const url = writeRisScenario(scenarioUrl, next);
    // The calculator is local; changing it does not need another distribution request.
    return goto(i18n.href(`${url.pathname}${url.search}${url.hash}`), {
      shallow: true,
      replace: true,
      reset: false,
      state: page.state,
    });
  }
</script>

<Seo
  title="The Relative Index for Streetlifting"
  description="Learn about RIS and calculate your score."
  canonical={absolute('/ris')}
/>

<article class="mx-auto max-w-reading px-gutter pt-chapter pb-16">
  <header class="mb-chapter">
    <Text as="h1" variant="title" class="text-ink"
      >{i18n.t('The Relative Index for Streetlifting')}</Text
    >
    <p class="mt-3 text-lead leading-7 text-secondary">
      {i18n.t(
        'The Relative Index for Streetlifting (RIS) compares competition totals across bodyweights within the same gender category. It uses the combined total of Muscle-up, Pull-up, Dips and Squat.'
      )}
    </p>
    <p class="mt-3 text-lead leading-7 text-secondary">
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
    <RisSimulator
      edition={current}
      bind:sex={() => scenario.sex, (sex) => updateScenario({ ...scenario, sex })}
      bind:fields={() => scenario.fields, (fields) => updateScenario({ ...scenario, fields })}
      bind:targetField={() => scenario.target, (target) => updateScenario({ ...scenario, target })}
    />
  {/if}

  <section aria-labelledby="interpretation-heading" class="mb-chapter">
    <Text as="h2" variant="heading" class="text-ink" id="interpretation-heading">
      {i18n.t('What does a score of 100 mean?')}
    </Text>
    <Text variant="body" class="mt-3 leading-6 text-secondary">
      {i18n.t(
        'A RIS of 100 means your total matches the formula’s reference total for your bodyweight and category. A score of 90 means you lifted 90% of that reference; 110 means you lifted 110%.'
      )}
    </Text>
    <Text variant="body" class="mt-3 leading-6 text-secondary">
      {i18n.t(
        'The reference comes from competition results among leading athletes. A score of 100 is not a maximum.'
      )}
    </Text>
  </section>

  {#if current}
    <section aria-labelledby="explore-heading" class="mb-chapter">
      <div class="mb-8">
        <Text as="h2" variant="heading" class="text-ink" id="explore-heading">
          {i18n.t('Explore competition results')}
        </Text>
        <Text variant="body" class="mt-3 leading-6 text-secondary">
          {i18n.t(
            'Each point represents a performance in the archive. The curve shows the total needed to reach 100 RIS at each bodyweight, using the {year} edition.',
            { year: current.year }
          )}
        </Text>
        <Text variant="body" class="mt-3 leading-6 text-secondary">
          {i18n.t('Select a point to see the athlete, competition and score.')}
        </Text>
      </div>
      <div class="mb-6">
        <RisCategory bind:sex={() => scenario.sex, (sex) => updateScenario({ ...scenario, sex })} />
      </div>

      {#key scenario.sex}
        <RisCurve
          constants={current.constants[scenario.sex]}
          points={data.distribution[scenario.sex]}
          label={i18n.t(
            'Competition total by bodyweight with the RIS 100 benchmark, {category}, {year} edition',
            {
              category: scenario.sex === 'men' ? i18n.t('Men') : i18n.t('Women'),
              year: current.year,
            }
          )}
        />
      {/key}
    </section>
  {/if}

  <section aria-labelledby="comparability-heading" class="mb-chapter">
    <Text as="h2" variant="heading" class="text-ink" id="comparability-heading">
      {i18n.t('Why a score may differ from the published result')}
    </Text>
    <Text variant="body" class="mt-3 leading-6 text-secondary">
      {i18n.t(
        'The RIS formula changes between editions. OpenStreetlifting uses the current edition wherever possible, including for older competitions, so recalculated scores share the same reference.'
      )}
    </Text>

    {#snippet mark()}<span class={REPORTED_MARK}>{REPORTED_GLYPH}</span>{/snippet}
    <Text variant="body" class="mt-3 leading-6 text-secondary">
      <Message
        message={'When a score cannot be recalculated, we keep the published value and mark it with {mark}. These scores may use an older edition.'}
        values={{ mark }}
      />
    </Text>
  </section>
</article>
