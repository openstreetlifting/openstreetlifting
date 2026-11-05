<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { ResolvedPathname } from '$app/types';
  import Card from './card.svelte';
  import { TEXT } from '#lib/constants/typography.js';
  import { LAYOUT } from '#lib/constants/layout.js';

  let {
    heading,
    href,
    background,
    flagBackground = false,
    children,
    footer,
  }: {
    heading: string;
    href?: ResolvedPathname;
    background?: Snippet;
    flagBackground?: boolean;
    children: Snippet;
    footer?: Snippet;
  } = $props();
</script>

<Card
  {href}
  class="group relative isolate block h-full max-w-none overflow-hidden {LAYOUT.panel} transition-colors hover:border-stroke-strong focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus"
>
  {#if background}<div class="ranking-art" class:ranking-flag={flagBackground} aria-hidden="true">
      {@render background()}
    </div>{/if}
  <div class="relative">
    <h3 class={TEXT.cardLabel}>{heading}</h3>
    <div class="mt-2 flex min-h-8 flex-wrap items-baseline gap-x-2 gap-y-1">
      {@render children()}
    </div>
    {#if footer}<div class="mt-2 {TEXT.caption}">{@render footer()}</div>{/if}
  </div>
</Card>

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
