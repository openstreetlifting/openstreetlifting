<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    label,
    children,
  }: {
    label: string;
    children: Snippet;
  } = $props();

  const panelId = $props.id();
</script>

<button
  type="button"
  popovertarget={panelId}
  aria-label={label}
  class="ml-1 inline-flex h-3.5 w-3.5 shrink-0 cursor-pointer items-center justify-center rounded-full border border-stroke-strong align-middle text-[0.6rem] leading-none font-medium text-secondary transition-colors hover:border-focus hover:text-ink"
>
  ?
</button>

<div
  id={panelId}
  popover="auto"
  class="m-auto max-h-[calc(100dvh-2rem)] w-[min(30rem,calc(100vw-2rem))] overflow-y-auto overscroll-contain rounded-xl whitespace-normal border border-stroke-strong bg-surface p-5 text-left sm:p-6 text-sm leading-relaxed font-normal text-secondary shadow-2xl shadow-overlay/70 backdrop:bg-overlay/70"
>
  {@render children()}
</div>
