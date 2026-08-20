<script lang="ts">
  import type { Snippet } from 'svelte';
  import Card from './card.svelte';

  let { head, body, busy = false }: { head: Snippet; body: Snippet; busy?: boolean } = $props();
</script>

<Card class="p-2 sm:p-4">
  <div class="overflow-x-auto">
    <table class="w-full text-sm transition-opacity {busy ? 'opacity-50' : ''}" aria-busy={busy}>
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
