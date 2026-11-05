<script lang="ts">
  import Text from './text.svelte';
  import type { Snippet } from 'svelte';
  import Card from './card.svelte';
  import { LAYOUT } from '#lib/constants/layout.js';

  const headingId = $props.id();
  let { title, controls, children }: { title: string; controls?: Snippet; children: Snippet } =
    $props();
</script>

<Card
  role="region"
  aria-labelledby={headingId}
  class="@container flex h-full min-w-0 max-w-none flex-col border-stroke-strong bg-surface-subtle"
>
  <div
    class="flex min-h-16 flex-wrap items-center justify-between gap-3 border-b border-stroke px-panel py-3"
  >
    <Text as="h3" variant="panelTitle" id={headingId}>{title}</Text>
    {#if controls}<div class="flex flex-wrap items-center gap-2">{@render controls()}</div>{/if}
  </div>
  <div class="flex min-h-0 min-w-0 flex-1 flex-col {LAYOUT.panel}">{@render children()}</div>
</Card>
