<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { ResolvedPathname } from '$app/types';
  import MetricCard from './metric-card.svelte';
  import { TEXT } from '#lib/constants/typography.js';
  import { useI18n } from '#lib/i18n/index.js';

  const i18n = useI18n();
  let {
    heading,
    place,
    field,
    href,
    missingLabel,
    background,
    flagBackground = false,
    children,
  }: {
    heading: string;
    place?: number;
    field?: number;
    href?: ResolvedPathname;
    missingLabel: string;
    background?: Snippet;
    flagBackground?: boolean;
    children?: Snippet;
  } = $props();
</script>

<MetricCard {heading} {href} {background} {flagBackground} footer={children}>
  {#if place !== undefined && field !== undefined}
    <span class={TEXT.cardFigure}>#{i18n.number(place, 0)}</span>
    <span class="font-mono text-meta text-secondary">/ {i18n.number(field, 0)}</span>
  {:else}<span class="text-body text-secondary">{missingLabel}</span>{/if}
</MetricCard>
