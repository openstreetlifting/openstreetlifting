<script lang="ts">
  import type { Snippet } from 'svelte';
  let {
    title,
    live = false,
    children,
  }: { title: string; live?: boolean; children?: Snippet } = $props();
  const headingId = $props.id();
</script>

<section
  aria-labelledby={headingId}
  class="mb-6 rounded-panel border border-stroke-strong bg-surface-subtle px-4 py-3"
>
  <h2
    id={headingId}
    class="flex items-center gap-2 font-mono text-xs font-medium leading-5 text-ink sm:text-sm"
  >
    {#if live}<span aria-hidden="true" class="size-1.5 shrink-0 rounded-full bg-success"
      ></span>{/if}
    {title}
  </h2>
  {#if children}<div class="mt-2">{@render children()}</div>{/if}
</section>
