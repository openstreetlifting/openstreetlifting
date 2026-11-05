<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { ChevronIcon } from '#lib/components/icons/index.js';
  import Button from '#lib/components/ui/button.svelte';
  import { FIELD, OVERLAY_SURFACE, TEXT } from '#lib/constants/typography.js';
  import {
    FORMAT_MOVEMENTS,
    formatMovements,
    normalizeEvent,
  } from '#lib/utils/competition-format.js';

  const i18n = useI18n();

  let {
    formats,
    value,
    onChange,
  }: {
    formats: string[];
    value: string;
    onChange: (value: string) => void;
  } = $props();

  let dropdown: HTMLDetailsElement;
  const choices = $derived(
    FORMAT_MOVEMENTS.filter(({ code }) => formats.some((format) => format.includes(code)))
  );
  const label = $derived(
    value
      ? formatMovements(value)
          .map((movement) => i18n.movement(movement))
          .join(', ')
      : i18n.t('All formats')
  );

  function dismiss(event: PointerEvent) {
    if (dropdown && event.target instanceof Node && !dropdown.contains(event.target)) {
      dropdown.open = false;
    }
  }

  function escape(event: KeyboardEvent) {
    if (event.key === 'Escape' && dropdown?.open) {
      dropdown.open = false;
      dropdown.querySelector('summary')?.focus();
    }
  }
</script>

<svelte:window onpointerdown={dismiss} onkeydown={escape} />

<div class="grid min-w-0 gap-1">
  <span class={TEXT.fieldLabel}>{i18n.t('Format')}</span>
  <details bind:this={dropdown} class="group relative min-w-0 w-full open:z-30">
    <summary
      aria-label={i18n.t('Competition format: {label}', { label })}
      class="{FIELD} flex cursor-pointer list-none items-center gap-2 px-3 py-1.5 group-open:border-focus [&::-webkit-details-marker]:hidden"
    >
      <span class="truncate" title={label}>{label}</span>
      <ChevronIcon class="ml-auto h-3.5 w-3.5 shrink-0 group-open:rotate-180" />
    </summary>
    <div class="{OVERLAY_SURFACE} absolute right-0 mt-1.5 w-full min-w-44 overflow-hidden">
      <fieldset class="space-y-0.5 p-1">
        <legend class="sr-only">{i18n.t('Match these movements exactly')}</legend>
        {#each choices as movement (movement.code)}
          {@const selected = value.includes(movement.code)}
          <label
            class="relative flex min-h-11 cursor-pointer items-center gap-2 rounded-control px-3 font-mono text-meta text-ink sm:min-h-8 {selected
              ? 'bg-surface-selected'
              : 'hover:bg-surface-hover'}"
          >
            <input
              type="checkbox"
              checked={selected}
              onchange={(event) =>
                onChange(
                  normalizeEvent(
                    event.currentTarget.checked
                      ? value + movement.code
                      : value.replace(movement.code, '')
                  )
                )}
              class="size-4 shrink-0 cursor-pointer appearance-none rounded-control border border-stroke-strong outline-none checked:border-action checked:bg-action focus-visible:ring-2 focus-visible:ring-focus focus-visible:ring-offset-2 focus-visible:ring-offset-surface"
            />
            {#if selected}
              <svg
                aria-hidden="true"
                viewBox="0 0 16 16"
                fill="none"
                class="pointer-events-none absolute left-3 size-3 text-on-action"
              >
                <path
                  d="m3 8 3.25 3.25L13 4.5"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                />
              </svg>
            {/if}
            <span>{i18n.movement(movement.label)}</span>
          </label>
        {/each}
      </fieldset>
      <div class="border-t border-stroke p-1">
        <Button
          variant="ghost"
          size="control"
          type="button"
          aria-label={i18n.t('Clear movements')}
          disabled={!value}
          onclick={() => onChange('')}
          class="w-full justify-start px-3 text-secondary disabled:text-faint"
          >{i18n.t('Clear selection')}</Button
        >
      </div>
    </div>
  </details>
</div>
