<script lang="ts">
  import Text from './text.svelte';
  import type { Snippet } from 'svelte';
  import Card from './card.svelte';
  import { LAYOUT } from '#lib/constants/layout.js';
  import { TEXT } from '#lib/constants/typography.js';

  const headingId = $props.id();
  let {
    title,
    controls,
    notes,
    children,
  }: { title: string; controls?: Snippet; notes?: Snippet; children: Snippet } = $props();
</script>

<section aria-labelledby={headingId} class="@container flex h-full min-w-0 flex-col">
  <div class="mb-3 flex min-h-control flex-wrap items-center justify-between gap-3">
    <Text as="h2" variant="heading" id={headingId} class="text-ink">{title}</Text>
    {#if controls}<div class="flex flex-wrap items-center gap-2">{@render controls()}</div>{/if}
  </div>
  <Card
    class="flex min-h-0 max-w-none min-w-0 flex-1 flex-col border-stroke-strong bg-surface-subtle {LAYOUT.panel}"
  >
    {@render children()}
  </Card>
  {#if notes}<div class="{TEXT.caption} mt-3">{@render notes()}</div>{/if}
</section>
