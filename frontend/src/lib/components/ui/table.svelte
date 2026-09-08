<script module lang="ts">
  export const TABLE_HEAD_CELL = 'px-1.5 py-1 text-left align-middle font-medium sm:px-3 sm:py-2';
  export const TABLE_CELL = 'px-1.5 py-1 sm:px-3 sm:py-2';
  export const FROZEN_CELL = 'sticky z-10 bg-[var(--row-bg)]';
  export const FROZEN_HEAD_CELL = 'sticky z-10 bg-surface';
  export const FROZEN_EDGE = 'border-r border-stroke sm:border-r-0';
  export const FROZEN_RANK = 'w-10 min-w-10 text-right sm:w-14 sm:min-w-14 left-0';
  export const ATHLETE_COLUMN = 'w-44 min-w-44 pl-1 sm:w-56 sm:min-w-56 sm:pl-3';
  export const ATHLETE_CONTENT = 'max-w-[10.5rem] sm:max-w-[12.5rem]';
</script>

<script lang="ts">
  import type { Snippet } from 'svelte';
  import { EDGE_TO_EDGE } from '$lib/constants/table';

  let { head, body, busy = false }: { head: Snippet; body: Snippet; busy?: boolean } = $props();
</script>

<div class="osl-table overflow-x-auto {EDGE_TO_EDGE}">
  <table class="w-full text-[0.7rem] whitespace-nowrap sm:text-xs" aria-busy={busy}>
    <thead class="sticky top-0 z-20">
      <tr class="border-b border-stroke bg-surface">
        {@render head()}
      </tr>
    </thead>
    <tbody>
      {@render body()}
    </tbody>
  </table>
</div>

<style>
  .osl-table :global(tbody) {
    transition: opacity 120ms ease-out;
  }

  .osl-table :global(table[aria-busy='true'] tbody) {
    opacity: 0.55;
  }

  .osl-table :global(tbody tr) {
    --row-bg: var(--table-row);
    background-color: var(--row-bg);
    border-bottom: 1px solid var(--color-stroke);
  }

  .osl-table :global(tbody tr:nth-child(even)) {
    --row-bg: var(--table-row-alt);
  }

  .osl-table :global(tbody tr:hover) {
    --row-bg: var(--table-row-hover);
  }

  .osl-table :global(tbody tr[data-focused]) {
    --row-bg: var(--table-row-focus);
  }
</style>
