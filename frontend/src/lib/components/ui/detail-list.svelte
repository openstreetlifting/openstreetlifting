<script lang="ts">
  import type { Snippet } from 'svelte';
  import { cn } from '#lib/utils/index.js';

  let {
    class: className,
    children,
    ...rest
  }: {
    class?: string;
    children: Snippet;
    [key: string]: unknown;
  } = $props();
</script>

<dl
  class={cn(
    'grid grid-cols-[7rem_minmax(0,1fr)] content-start gap-x-3 gap-y-1.5 text-body',
    className
  )}
  {...rest}
>
  {@render children()}
</dl>
