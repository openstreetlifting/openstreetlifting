<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { ResolvedPathname } from '$app/types';
  import Card from './card.svelte';
  import { cn } from '#lib/utils/index.js';

  let {
    class: className,
    children,
    href,
    ...rest
  }: {
    class?: string;
    children: Snippet;
    href?: ResolvedPathname;
    [key: string]: unknown;
  } = $props();
</script>

<Card {href} class={cn('max-w-none p-panel', className)} {...rest}>
  {@render children()}
</Card>
