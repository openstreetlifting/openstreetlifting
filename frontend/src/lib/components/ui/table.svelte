<script lang="ts">
  import type { Snippet } from 'svelte';
  import Card from './card.svelte';

  let { head, body }: { head: Snippet; body: Snippet } = $props();
</script>

<Card class="p-4">
  <div class="overflow-x-auto">
    <table class="w-full text-sm">
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
