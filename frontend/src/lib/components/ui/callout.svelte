<script lang="ts">
  import type { Snippet } from 'svelte';
  import InfoIcon from '#lib/components/icons/InfoIcon.svelte';

  let {
    title,
    class: className = '',
    children,
  }: { title: string; class?: string; children: Snippet } = $props();
</script>

<div role="note" class="border-l-4 border-stroke-strong py-1 pl-4 {className}">
  <p class="flex items-center gap-2 text-meta font-medium text-ink">
    <InfoIcon class="h-4 w-4 shrink-0" />
    {title}
  </p>
  <p class="mt-1 text-body text-secondary">{@render children()}</p>
</div>
