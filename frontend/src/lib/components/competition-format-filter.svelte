<script lang="ts">
  import { ChevronIcon } from '$lib/components/icons';
  import { FORMAT_MOVEMENTS, formatMovements, normalizeEvent } from '$lib/utils/competition-format';

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
  const label = $derived(value ? formatMovements(value).join(', ') : 'All formats');

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

<details bind:this={dropdown} class="group relative w-full open:z-30 sm:w-48">
  <summary
    aria-label="Competition format: {label}"
    class="flex cursor-pointer list-none items-center gap-2 rounded-lg border border-stroke bg-surface px-3 py-2 text-xs text-secondary outline-none hover:border-stroke-strong hover:text-ink focus-visible:ring-2 focus-visible:ring-focus focus-visible:ring-offset-2 focus-visible:ring-offset-canvas group-open:border-stroke-strong group-open:text-ink [&::-webkit-details-marker]:hidden"
  >
    <span class="truncate" title={label}>{label}</span>
    <ChevronIcon class="ml-auto h-3.5 w-3.5 shrink-0 group-open:rotate-180" />
  </summary>
  <div
    class="absolute left-0 mt-1.5 w-full overflow-hidden rounded-lg border border-stroke-strong bg-surface shadow-lg shadow-overlay/20"
  >
    <fieldset class="space-y-0.5 p-1">
      <legend class="sr-only">Match these movements exactly</legend>
      {#each choices as movement (movement.code)}
        {@const selected = value.includes(movement.code)}
        <label
          class="relative flex min-h-11 cursor-pointer items-center gap-2.5 rounded px-2.5 text-sm text-ink sm:min-h-9 {selected
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
            class="size-4 shrink-0 cursor-pointer appearance-none rounded border border-stroke-strong outline-none checked:border-action checked:bg-action focus-visible:ring-2 focus-visible:ring-focus focus-visible:ring-offset-2 focus-visible:ring-offset-surface"
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
          <span>{movement.label}</span>
        </label>
      {/each}
    </fieldset>
    <div class="border-t border-stroke p-1">
      <button
        type="button"
        aria-label="Clear movements"
        disabled={!value}
        onclick={() => onChange('')}
        class="min-h-11 w-full rounded px-2.5 text-left text-xs text-secondary outline-none enabled:cursor-pointer enabled:hover:bg-surface-hover enabled:hover:text-ink enabled:active:scale-[0.97] focus-visible:ring-2 focus-visible:ring-focus disabled:text-faint sm:min-h-8"
        >Clear selection</button
      >
    </div>
  </div>
</details>
