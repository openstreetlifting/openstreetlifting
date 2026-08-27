<script module lang="ts">
  // Cell padding lives here so the three tables cannot drift apart. Colour is
  // left to the caller, since an active or highlighted cell overrides it.
  export const TABLE_HEAD_CELL = 'px-1.5 py-1.5 text-left font-medium sm:px-3 sm:py-2';
  export const TABLE_CELL = 'px-1.5 py-1.5 sm:px-3 sm:py-2';

  // A results table is wider than a phone and always will be, so the cost of
  // reading one is horizontal scrolling. What that destroys is knowing whose
  // row you are on, so the columns that answer it stay pinned while the rest
  // pans underneath. They repaint the row background themselves, since what
  // scrolls behind them would otherwise show through.
  export const FROZEN_CELL = 'sticky z-10 bg-[var(--row-bg)]';
  export const FROZEN_HEAD_CELL = 'sticky z-10 bg-zinc-900';
  /** The edge that shows the columns to its right have moved. */
  export const FROZEN_EDGE = 'border-r border-zinc-800';

  // Both pinned columns carry their width here rather than taking it from the
  // longest value in them, so a long name truncates instead of pushing the
  // rest of its row off screen, and so the left offsets stay exact: the
  // athlete column starts where the rank column ends.
  // The min-widths are not decoration. An auto-layout table gives a column its
  // content's width and treats `w-` as advisory, so without them the rank
  // column renders narrower than the offset the athlete column is pinned at,
  // and the gap between the two shows whatever is scrolling past underneath.
  export const FROZEN_RANK = 'w-11 min-w-11 sm:w-14 sm:min-w-14 left-0';
  export const FROZEN_ATHLETE = 'w-32 min-w-32 sm:w-56 sm:min-w-56 left-11 sm:left-14';

  // An auto-layout table treats a width on a cell as a suggestion and widens
  // the column to whatever its longest value wants, which for a name column is
  // most of the screen. Capping the content is what actually holds the column,
  // so this belongs on whatever sits inside a pinned athlete cell.
  export const FROZEN_ATHLETE_CONTENT = 'max-w-[7rem] sm:max-w-[12.5rem]';
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
</style>
