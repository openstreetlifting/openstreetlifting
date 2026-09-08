<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';

  let {
    class: className,
    children,
    ...restProps
  }: {
    class?: string;
    children?: Snippet;
    [key: string]: unknown;
  } = $props();
</script>

<div
  class={cn('w-full max-w-card rounded-xl border border-stroke bg-surface text-ink', className)}
  {...restProps}
>
  {#if children}
    {@render children()}
  {/if}
</div>
