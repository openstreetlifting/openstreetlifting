<script lang="ts">
  import type { Snippet } from 'svelte';
  import { cn } from '#lib/utils/index.js';

  let {
    label,
    labelId,
    labelClass,
    class: className,
    children,
  }: {
    label: string;
    labelId?: string;
    labelClass?: string;
    class?: string;
    children: Snippet;
  } = $props();
</script>

<dt id={labelId} class={cn('text-muted', labelClass)}>{label}</dt>
<dd class={cn('min-w-0 text-ink', className)}>{@render children()}</dd>
