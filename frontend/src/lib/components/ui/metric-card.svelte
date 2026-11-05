<script lang="ts">
  import Text from './text.svelte';
  import type { Snippet } from 'svelte';
  import type { ResolvedPathname } from '$app/types';
  import Panel from './panel.svelte';
  import { TEXT } from '#lib/constants/typography.js';

  let {
    heading,
    meta,
    href,
    background,
    flagBackground = false,
    children,
    footer,
  }: {
    heading: string;
    meta?: string;
    href?: ResolvedPathname;
    background?: Snippet;
    flagBackground?: boolean;
    children: Snippet;
    footer?: Snippet;
  } = $props();
</script>

<Panel
  {href}
  class="group relative isolate block h-full overflow-hidden transition-colors hover:border-stroke-strong focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus"
>
  {#if background}<div class="ranking-art" class:ranking-flag={flagBackground} aria-hidden="true">
      {@render background()}
    </div>{/if}
  <div class="relative">
    <div class="flex items-baseline justify-between gap-3">
      <Text as="h3" variant="cardLabel">{heading}</Text>
      {#if meta}<span class="{TEXT.fieldLabel} hidden shrink-0 tabular-nums lg:inline">{meta}</span
        >{/if}
    </div>
    <div class="mt-2 flex min-h-8 flex-wrap items-baseline gap-x-2 gap-y-1">
      {@render children()}
    </div>
    {#if footer}<div class="mt-2 {TEXT.caption}">{@render footer()}</div>{/if}
  </div>
</Panel>

<style>
  .ranking-art {
    position: absolute;
    inset: 0 0 0 auto;
    display: flex;
    height: 100%;
    aspect-ratio: 3 / 2;
    align-items: center;
    justify-content: flex-end;
    pointer-events: none;
    opacity: 0.4;
    mask-image: linear-gradient(
      to right,
      transparent,
      color-mix(in srgb, black 30%, transparent) 10%,
      color-mix(in srgb, black 75%, transparent) 24%,
      black 36%
    );
  }
  .ranking-art:not(.ranking-flag) :global(svg) {
    height: 125%;
    width: auto;
    flex: none;
    aspect-ratio: 1;
    stroke-width: 0.7;
  }
  .ranking-flag {
    opacity: 0.6;
  }
</style>
