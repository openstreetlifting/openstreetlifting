<script lang="ts" module>
  const GAPS = {
    1: 'gap-1',
    2: 'gap-2',
    3: 'gap-3',
    4: 'gap-4',
    6: 'gap-6',
    8: 'gap-8',
    12: 'gap-12',
    tile: 'gap-tile',
    section: 'gap-section',
    chapter: 'gap-chapter',
  } as const;
</script>

<script lang="ts">
  import type { Snippet } from 'svelte';
  import { cn } from '#lib/utils/index.js';

  let {
    as = 'div',
    gap = 4,
    class: className,
    children,
    ...rest
  }: {
    as?: keyof HTMLElementTagNameMap;
    gap?: keyof typeof GAPS;
    class?: string;
    children: Snippet;
    [key: string]: unknown;
  } = $props();
</script>

<svelte:element this={as} class={cn('flex flex-col', GAPS[gap], className)} {...rest}>
  {@render children()}
</svelte:element>
