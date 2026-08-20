<script module lang="ts">
  // Cell padding lives here so the three tables cannot drift apart. Colour is
  // left to the caller, since an active or highlighted cell overrides it.
  export const TABLE_HEAD_CELL = 'px-1.5 py-1.5 text-left font-medium sm:px-3 sm:py-2';
  export const TABLE_CELL = 'px-1.5 py-1.5 sm:px-3 sm:py-2';
</script>

<script lang="ts">
  import type { Snippet } from 'svelte';
  import Card from './card.svelte';

  let { head, body, busy = false }: { head: Snippet; body: Snippet; busy?: boolean } = $props();
</script>

<!-- On a phone the table runs edge to edge, so the page gutter and the card
     chrome are cancelled below sm rather than eating 50px of a 375px screen. -->
<div class="-mx-4 sm:mx-0">
  <Card class="rounded-none border-x-0 p-0 sm:rounded-xl sm:border-x sm:p-4">
    <div class="overflow-x-auto">
      <table
        class="w-full text-xs transition-opacity sm:text-sm {busy ? 'opacity-50' : ''}"
        aria-busy={busy}
      >
        <thead class="sticky top-0 z-10">
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
