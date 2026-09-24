<script lang="ts">
  import { ChevronIcon } from '$lib/components/icons';
  import { FIELD } from '$lib/constants/typography';
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

<details bind:this={dropdown} class="relative w-full sm:w-auto">
  <summary
    aria-label="Competition format: {label}"
    class="{FIELD} flex cursor-pointer list-none items-center gap-2 px-3 py-2 [&::-webkit-details-marker]:hidden"
  >
    <span class="truncate sm:max-w-64" title={label}>{label}</span>
    <ChevronIcon class="ml-auto h-3.5 w-3.5 shrink-0" />
  </summary>
  <div
    class="absolute left-0 z-20 mt-2 w-full min-w-64 rounded-lg border border-stroke bg-surface p-3 shadow-lg"
  >
    <fieldset>
      <legend class="mb-2 text-sm text-secondary">Match these movements exactly</legend>
      {#each choices as movement (movement.code)}
        <label
          class="flex cursor-pointer items-center gap-3 rounded px-2 py-2 text-sm text-ink hover:bg-surface-hover"
        >
          <input
            type="checkbox"
            checked={value.includes(movement.code)}
            onchange={(event) =>
              onChange(
                normalizeEvent(
                  event.currentTarget.checked
                    ? value + movement.code
                    : value.replace(movement.code, '')
                )
              )}
            class="h-4 w-4 accent-action"
          />
          {movement.label}
        </label>
      {/each}
    </fieldset>
    <button
      type="button"
      disabled={!value}
      onclick={() => onChange('')}
      class="mt-2 px-2 py-1 text-sm text-secondary underline hover:text-ink disabled:opacity-50"
      >Clear movements</button
    >
  </div>
</details>
