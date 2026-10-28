<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import Message from './message.svelte';
  import RisCategory from './ris-category.svelte';
  import { FIELD, TEXT } from '#lib/constants/typography.js';
  import type { RisFormula } from '#lib/types/ris.js';
  import { benchmarkTotal, bodyweightForRis, calculateRis } from '#lib/utils/ris.js';

  const i18n = useI18n();

  let {
    edition,
    sex = $bindable('men'),
  }: {
    edition: RisFormula;
    sex?: 'men' | 'women';
  } = $props();

  const id = $props.id();
  const controls = [
    { key: 'bodyweight', label: 'Bodyweight', min: 30, max: 220, step: 0.1 },
    { key: 'total', label: 'Total lifted', min: 0, max: 1000, step: 0.25 },
  ] as const;
  let fields = $state({ bodyweight: '', total: '', target: '' });
  const values = $derived({
    bodyweight: parseWeight(fields.bodyweight),
    total: parseWeight(fields.total),
  });
  const target = $derived(parseWeight(fields.target));

  function parseWeight(value: string): number | undefined {
    const normalized = value.trim().replace(',', '.');
    if (!/^\d+(\.\d*)?$/.test(normalized)) return undefined;
    const number = Number(normalized);
    return Number.isFinite(number) ? number : undefined;
  }

  const constants = $derived(edition.constants[sex]);
  const performance = $derived.by(() => {
    const { bodyweight, total } = values;
    if (
      bodyweight === undefined ||
      total === undefined ||
      bodyweight < 30 ||
      bodyweight > 220 ||
      total < 0 ||
      total > 1000
    )
      return null;
    return { bodyweight, total, score: calculateRis(constants, bodyweight, total) };
  });
  const scenario = $derived.by(() => {
    if (!performance || target === undefined || target <= 0) return null;
    return {
      total: (benchmarkTotal(constants, performance.bodyweight) * target) / 100,
      bodyweight: bodyweightForRis(constants, performance.total, target),
    };
  });

  const displayedScore = $derived(performance ? Number(performance.score.toFixed(2)) : null);
  const targetMet = $derived(
    scenario !== null && displayedScore !== null && target !== undefined && displayedScore >= target
  );
</script>

<section aria-labelledby="{id}-heading" class="mb-10 sm:mb-12">
  <h2 id="{id}-heading" class="{TEXT.heading} text-ink">{i18n.t('Calculate your RIS')}</h2>
  <p class="mt-3 {TEXT.body} leading-6 text-secondary">
    {i18n.t(
      'Choose your category and enter your bodyweight and MPDS total. Add a target score to explore the corresponding total or bodyweight.'
    )}
  </p>

  <div class="mt-7 grid items-start gap-6 sm:grid-cols-[auto_1fr_1fr]">
    <div>
      <RisCategory bind:sex prominent />
    </div>
    {#each controls as control (control.key)}
      {@const value = values[control.key]}
      {@const invalid =
        fields[control.key] !== '' &&
        (value === undefined || value < control.min || value > control.max)}
      <div>
        <div class="mb-3">
          <label for="{id}-{control.key}" class="mb-2 block text-sm font-medium text-ink"
            >{i18n.t(control.label)}</label
          >
          <div class="relative">
            <input
              id="{id}-{control.key}"
              type="text"
              inputmode="decimal"
              placeholder={control.key === 'bodyweight'
                ? i18n.t('Bodyweight')
                : i18n.t('MPDS total')}
              bind:value={fields[control.key]}
              aria-invalid={invalid}
              aria-describedby={invalid ? `${id}-${control.key}-error` : undefined}
              class="{FIELD} h-12 w-full border-stroke-strong bg-surface px-3 pr-10 text-base text-ink placeholder:text-sm placeholder:text-secondary focus:border-secondary"
            />
            <span
              class="pointer-events-none absolute inset-y-0 right-3 flex items-center text-sm text-secondary"
              >kg</span
            >
          </div>
        </div>
        {#if invalid}
          <p id="{id}-{control.key}-error" class="text-xs text-secondary">
            {i18n.t('Enter a number between {min} and {max} kg.', {
              min: control.min,
              max: control.max,
            })}
          </p>
        {/if}
        {#if value !== undefined && !invalid}
          <input
            type="range"
            aria-label={i18n.t('{label} slider', { label: i18n.t(control.label) })}
            aria-valuetext="{i18n.number(value)} kg"
            min={control.min}
            max={control.max}
            step={control.step}
            value={values[control.key]}
            oninput={(event) => {
              fields[control.key] = event.currentTarget.value;
            }}
            class="ris-slider block h-6 w-full cursor-pointer accent-ink focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-muted"
          />
          <div class="mt-1 flex justify-between text-xs text-muted" aria-hidden="true">
            <span>{control.min.toLocaleString(i18n.locale)} kg</span>
            <span>{control.max.toLocaleString(i18n.locale)} kg</span>
          </div>
        {/if}
      </div>
    {/each}
  </div>

  {#if performance}
    <div class="mt-7 flex flex-wrap items-baseline gap-3 font-normal text-ink">
      <span id="{id}-score-label" class="text-sm">RIS:</span>
      <output
        data-ris-live-score
        aria-labelledby="{id}-score-label"
        class="font-mono text-2xl font-normal"
        aria-live="off">{i18n.number(performance.score, 2)}</output
      >
    </div>

    <section aria-labelledby="{id}-target-heading" class="mt-8 border-y border-stroke py-5">
      <h2 id="{id}-target-heading" class="{TEXT.heading} text-ink">{i18n.t('Target RIS')}</h2>
      <div class="mt-5 mb-5 flex flex-wrap items-center gap-3">
        <label for="{id}-target" class="text-sm text-secondary">{i18n.t('Target score')}</label>
        <input
          id="{id}-target"
          aria-label={i18n.t('Target RIS')}
          type="text"
          inputmode="decimal"
          placeholder={i18n.t('Target')}
          bind:value={fields.target}
          class="{FIELD} w-24 border-stroke-strong px-3 py-2 text-base font-mono"
        />
        <span class="text-sm text-secondary">RIS</span>
      </div>
      {#if targetMet}
        <p class="text-sm leading-6 text-ink">
          {#if displayedScore !== null && target !== undefined && displayedScore > target}
            {i18n.t('The calculated score is above the target of {target} RIS.', {
              target: i18n.number(target!),
            })}
          {:else}
            {i18n.t('The displayed score equals the target of {target} RIS.', {
              target: i18n.number(target!),
            })}
          {/if}
        </p>
      {:else if scenario}
        <div class="space-y-4 text-sm leading-6 text-secondary">
          <p>
            {#snippet total()}<strong class="font-medium text-ink"
                >{i18n.number(scenario.total, 1)} kg</strong
              >{/snippet}
            {#snippet difference()}<strong class="font-medium text-ink"
                >{i18n.t('{difference} kg more', {
                  difference: i18n.number(scenario.total - performance.total, 1),
                })}</strong
              >{/snippet}
            <Message
              message={'At a fixed bodyweight of {bodyweight} kg, {target} RIS corresponds to a total of {total}, or {difference} than the entered total.'}
              values={{
                bodyweight: i18n.number(performance.bodyweight),
                target: i18n.number(target!),
                total,
                difference,
              }}
            />
          </p>
          <p>
            {#if scenario.bodyweight.kind === 'bodyweight'}
              {#if scenario.bodyweight.value >= 30 && scenario.bodyweight.value <= 220}
                {@const calculatedWeight = scenario.bodyweight.value}
                {#snippet bodyweight()}<strong class="font-medium text-ink"
                    >{i18n.t('{bodyweight} kg bodyweight', {
                      bodyweight: i18n.number(calculatedWeight, 1),
                    })}</strong
                  >{/snippet}
                {#snippet difference()}<strong class="font-medium text-ink"
                    >{i18n.t('{difference} kg less', {
                      difference: i18n.number(performance.bodyweight - calculatedWeight, 1),
                    })}</strong
                  >{/snippet}
                <Message
                  message={'At a fixed total of {total} kg, {target} RIS corresponds to {bodyweight}, or {difference} than the entered bodyweight.'}
                  values={{
                    total: i18n.number(performance.total),
                    target: i18n.number(target!),
                    bodyweight,
                    difference,
                  }}
                />
              {:else}
                {i18n.t(
                  'At the entered total, the bodyweight corresponding to this target lies outside the calculator’s 30–220 kg input range.'
                )}
              {/if}
            {:else if scenario.bodyweight.kind === 'already-met'}
              {i18n.t(
                'At the entered total, the model yields a score at or above this target for any finite bodyweight.'
              )}
            {:else}
              {i18n.t(
                'At the entered total, no finite bodyweight in the model yields this target score.'
              )}
            {/if}
          </p>
        </div>
      {:else}
        <p class="text-xs leading-6 text-secondary">
          {i18n.t(
            'Enter a positive target score to calculate the corresponding total and bodyweight.'
          )}
        </p>
      {/if}
    </section>
  {/if}
</section>

<style>
  .ris-slider {
    appearance: none;
    background: transparent;
  }
  .ris-slider::-webkit-slider-runnable-track {
    height: 3px;
    border-radius: 2px;
    background: var(--color-stroke-strong);
  }
  .ris-slider::-moz-range-track {
    height: 3px;
    border-radius: 2px;
    background: var(--color-stroke-strong);
  }
  .ris-slider::-webkit-slider-thumb {
    appearance: none;
    width: 16px;
    height: 16px;
    margin-top: -6.5px;
    border-radius: 50%;
    background: var(--color-ink);
    border: 2px solid var(--color-canvas);
  }
  .ris-slider::-moz-range-thumb {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: var(--color-ink);
    border: 2px solid var(--color-canvas);
  }
</style>
