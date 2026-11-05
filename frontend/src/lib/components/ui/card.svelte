<script lang="ts">
  import { cn } from '#lib/utils/index.js';
  import type { Snippet } from 'svelte';
  import type { ResolvedPathname } from '$app/types';

  let {
    class: className,
    children,
    href,
    ...restProps
  }: {
    class?: string;
    children?: Snippet;
    href?: ResolvedPathname;
    [key: string]: unknown;
  } = $props();
  const classes = $derived(
    cn('w-full max-w-card rounded-panel border border-stroke bg-surface text-ink', className)
  );
</script>

{#if href}
  <a {href} class={classes} {...restProps}>{@render children?.()}</a>
{:else}
  <div class={classes} {...restProps}>{@render children?.()}</div>
{/if}
