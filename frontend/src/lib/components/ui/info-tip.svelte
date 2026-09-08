<script lang="ts">
  import type { Snippet } from 'svelte';
  import CloseIcon from '$lib/components/icons/CloseIcon.svelte';

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
  <button
    type="button"
    popovertarget={panelId}
    popovertargetaction="hide"
    aria-label="Close information"
    class="float-right -mt-1 -mr-1 mb-1 ml-3 inline-flex h-8 w-8 cursor-pointer items-center justify-center rounded-md text-muted transition-colors hover:bg-surface-hover hover:text-ink focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus active:scale-95"
  >
    <CloseIcon class="h-4 w-4" />
  </button>
  {@render children()}
</div>
