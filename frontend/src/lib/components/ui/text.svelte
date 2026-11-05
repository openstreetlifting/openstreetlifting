<script lang="ts">
  import type { Snippet } from 'svelte';
  import { TEXT } from '#lib/constants/typography.js';
  import { cn } from '#lib/utils/index.js';

  let {
    as = 'p',
    variant = 'body',
    class: className,
    children,
    ...rest
  }: {
    as?: keyof HTMLElementTagNameMap;
    variant?: keyof typeof TEXT;
    class?: string;
    children: Snippet;
    [key: string]: unknown;
  } = $props();
</script>

<svelte:element this={as} class={cn(TEXT[variant], className)} {...rest}>
  {@render children()}
</svelte:element>
