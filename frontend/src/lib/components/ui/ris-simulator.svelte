<script lang="ts">
  import RisCategory from './ris-category.svelte';
  import { FIELD, TEXT } from '$lib/constants/typography';
  import type { RisFormula } from '$lib/types/ris';
  import { benchmarkTotal, bodyweightForRis, calculateRis } from '$lib/utils/ris';

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
  <h2 id="{id}-heading" class="{TEXT.heading} text-ink">RIS Calculator</h2>

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
            >{control.label}</label
          >
          <div class="relative">
            <input
              id="{id}-{control.key}"
              type="text"
              inputmode="decimal"
              placeholder={control.key === 'bodyweight' ? 'Bodyweight' : 'MPDS total'}
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
            Enter a number between {control.min} and {control.max} kg.
          </p>
        {/if}
        {#if value !== undefined && !invalid}
          <input
            type="range"
            aria-label="{control.label} slider"
            aria-valuetext="{values[control.key]} kg"
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
            <span>{control.min.toLocaleString('en-US')} kg</span>
            <span>{control.max.toLocaleString('en-US')} kg</span>
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
        aria-live="off">{performance.score.toFixed(2)}</output
      >
    </div>

    <section aria-labelledby="{id}-target-heading" class="mt-8 border-y border-stroke py-5">
      <h2 id="{id}-target-heading" class="{TEXT.heading} text-ink">Target RIS</h2>
      <div class="mt-5 mb-5 flex flex-wrap items-center gap-3">
        <label for="{id}-target" class="text-sm text-secondary">Target score</label>
        <input
          id="{id}-target"
          aria-label="Target RIS"
          type="text"
          inputmode="decimal"
          placeholder="Target"
          bind:value={fields.target}
          class="{FIELD} w-24 border-stroke-strong px-3 py-2 text-base font-mono"
        />
        <span class="text-sm text-secondary">RIS</span>
      </div>
      {#if targetMet}
        <p class="text-sm leading-6 text-ink">
          {#if displayedScore !== null && target !== undefined && displayedScore > target}
            The calculated score is above the target of {target} RIS.
          {:else}
            The displayed score equals the target of {target} RIS.
          {/if}
        </p>
      {:else if scenario}
        <div class="space-y-4 text-sm leading-6 text-secondary">
          <p>
            At a fixed bodyweight of {performance.bodyweight} kg, {target} RIS corresponds to a total
            of
            <strong class="font-medium text-ink">{scenario.total.toFixed(1)} kg</strong>, or
            <strong class="font-medium text-ink"
              >{(scenario.total - performance.total).toFixed(1)} kg more</strong
            > than the entered total.
          </p>
          <p>
            {#if scenario.bodyweight.kind === 'bodyweight'}
              {#if scenario.bodyweight.value >= 30 && scenario.bodyweight.value <= 220}
                At a fixed total of {performance.total} kg, {target} RIS corresponds to
                <strong class="font-medium text-ink"
                  >{scenario.bodyweight.value.toFixed(1)} kg bodyweight</strong
                >, or
                <strong class="font-medium text-ink"
                  >{(performance.bodyweight - scenario.bodyweight.value).toFixed(1)} kg less</strong
                > than the entered bodyweight.
              {:else}
                At the entered total, the bodyweight corresponding to this target lies outside the
                calculator’s 30–220 kg input range.
              {/if}
            {:else if scenario.bodyweight.kind === 'already-met'}
              At the entered total, the model yields a score at or above this target for any finite
              bodyweight.
            {:else}
              At the entered total, no finite bodyweight in the model yields this target score.
            {/if}
          </p>
        </div>
      {:else}
        <p class="text-xs leading-6 text-secondary">
          Enter a positive target score to calculate the corresponding total and bodyweight.
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
