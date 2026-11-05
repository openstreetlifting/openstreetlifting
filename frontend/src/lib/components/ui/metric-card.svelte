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
    inset: 0 -1rem 0 auto;
    display: flex;
    width: 55%;
    align-items: center;
    justify-content: flex-end;
    pointer-events: none;
    opacity: 0.65;
    mask-image: linear-gradient(to right, transparent, black 75%);
  }
  .ranking-art :global(svg) {
    stroke-width: 0.7;
  }
  .ranking-flag {
    inset: -15% -1rem auto auto;
    width: auto;
    height: 130%;
    aspect-ratio: 36 / 26;
  }
  @media (width < 40rem) {
    .ranking-flag {
      inset: 0;
      width: 100%;
      height: 100%;
      aspect-ratio: auto;
      opacity: 0.2;
      mask-image: none;
    }
  }
</style>
