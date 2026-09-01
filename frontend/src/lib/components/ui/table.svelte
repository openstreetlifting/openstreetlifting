<script module lang="ts">
  // Cell padding lives here so the three tables cannot drift apart. Colour is
  // left to the caller, since an active or highlighted cell overrides it.
  export const TABLE_HEAD_CELL = 'px-1.5 py-1 text-left font-medium sm:px-3 sm:py-2';
  export const TABLE_CELL = 'px-1.5 py-1 sm:px-3 sm:py-2';

  // A results table is wider than a phone and always will be, so the cost of
  // reading one is horizontal scrolling. The rank stays pinned through it, since
  // an ordered list read sideways still has to say which place each row is.
  // The name does not: it sits beside Total and RIS, which is what the table is
  // scrolled to for, and pinning it too would spend two thirds of a phone screen
  // holding an anchor still while the columns it anchors are read through what
  // is left. Pinned cells repaint the row background themselves, since what
  // scrolls behind them would otherwise show through.
  export const FROZEN_CELL = 'sticky z-10 bg-[var(--row-bg)]';
  export const FROZEN_HEAD_CELL = 'sticky z-10 bg-zinc-900';
  /**
   * The edge that shows the columns to its right have moved. Only below sm:
   * from there the table fits without scrolling, nothing slides under the
   * pinned column, and the rule is then just a line through the row.
   */
  export const FROZEN_EDGE = 'border-r border-zinc-800 sm:border-r-0';

  export const FROZEN_RANK = 'w-10 min-w-10 text-right sm:w-14 sm:min-w-14 left-0';

  export const ATHLETE_COLUMN = 'w-44 min-w-44 pl-1 sm:w-56 sm:min-w-56 sm:pl-3';

  // An auto-layout table treats a width on a cell as a suggestion and widens
  // the column to whatever its longest value wants, which for a name column is
  // most of the screen. Capping the content is what actually holds the column,
  // so this belongs on whatever sits inside an athlete cell.
  export const ATHLETE_CONTENT = 'max-w-[10.5rem] sm:max-w-[12.5rem]';
</script>

<script lang="ts">
  import type { Snippet } from 'svelte';
  import Card from './card.svelte';
  import { EDGE_TO_EDGE } from '$lib/constants/table';

  let { head, body, busy = false }: { head: Snippet; body: Snippet; busy?: boolean } = $props();
</script>

<!-- On a phone the table runs edge to edge, so the page gutter and the card
 chrome are cancelled below sm rather than eating 50px of a 375px screen. -->
<div class={EDGE_TO_EDGE}>
  <Card class="rounded-none border-x-0 p-0 sm:rounded-xl sm:border-x sm:p-4">
    <div class="osl-table overflow-x-auto">
      <table class="w-full text-[0.7rem] sm:text-xs" aria-busy={busy}>
        <thead class="sticky top-0 z-20">
          <tr class="border-b border-zinc-800 bg-zinc-900 shadow-lg shadow-zinc-950/50">
            {@render head()}
          </tr>
        </thead>
        <tbody>
          {@render body()}
        </tbody>
      </table>
    </div>
  </Card>
</div>

<style>
  /* The rows are the caller's, so the row colour is published to them as a
 custom property that the pinned cells can paint themselves with. Setting
 it here is what keeps a pinned cell and the rest of its row the same
 colour through zebra striping and hover. */
  /* Rows that are being replaced fade rather than empty out, so the wait keeps
 the table's height and the reader's place in it. */
  .osl-table :global(tbody) {
    transition: opacity 120ms ease-out;
  }

  .osl-table :global(table[aria-busy='true'] tbody) {
    opacity: 0.55;
  }

  .osl-table :global(tbody tr) {
    --row-bg: var(--table-row);
    background-color: var(--row-bg);
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
