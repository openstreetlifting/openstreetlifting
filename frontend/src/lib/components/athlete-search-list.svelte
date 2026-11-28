<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { resolve } from '$app/paths';
  import Flag from '#lib/components/ui/flag.svelte';
  import { formatAthleteName } from '#lib/utils/index.js';
  import { rankingLabels } from '#lib/utils/athlete-search.js';
  import type { AthleteSearchHit } from '#lib/types/athlete.js';

  const i18n = useI18n();

  let {
    hits,
    active = -1,
    onpick,
  }: { hits: AthleteSearchHit[]; active?: number; onpick?: () => void } = $props();

  function where(hit: AthleteSearchHit): string {
    const labels = rankingLabels(hit);
    return labels.length
      ? labels.map((label) => i18n.t(label)).join(' · ')
      : i18n.t('No valid lift');
  }
</script>

<ul class="divide-y divide-stroke">
  {#each hits as hit, index (hit.slug)}
    <li>
      <a
        href={i18n.href(resolve(`athletes/${hit.slug}`))}
        onclick={onpick}
        data-active={index === active ? '' : undefined}
        class="flex min-h-11 items-center gap-3 px-gutter py-2 text-meta transition-colors hover:bg-surface focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-focus data-active:bg-surface"
      >
        <Flag countryCode={hit.country} />
        <span class="min-w-0 flex-1 truncate text-ink">{formatAthleteName(hit)}</span>
        <span class="shrink-0 font-mono text-micro text-muted">{where(hit)}</span>
      </a>
    </li>
  {/each}
</ul>
