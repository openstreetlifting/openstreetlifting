<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { TEXT } from '#lib/constants/typography.js';
  import AthleteSearchList from './athlete-search-list.svelte';
  import type { AthleteSearchHit } from '#lib/types/athlete.js';

  const i18n = useI18n();

  let { hits, total }: { hits: AthleteSearchHit[]; total: number } = $props();

  const hidden = $derived(Math.max(0, total - hits.length));
</script>

<section
  class="border-b border-stroke-strong bg-surface-subtle"
  aria-label={i18n.t('Not in this ranking')}
>
  <h2 class="px-gutter pt-3 pb-1 {TEXT.fieldLabel}">
    {i18n.t('Not in this ranking')} ({total})
  </h2>
  <AthleteSearchList {hits} />
  {#if hidden > 0}
    <p class="px-gutter pb-3 {TEXT.caption}">
      {i18n.t('{count} more. Type more of the name to narrow the list.', { count: hidden })}
    </p>
  {/if}
</section>
