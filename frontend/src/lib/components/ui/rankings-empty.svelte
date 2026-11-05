<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import type { ResolvedPathname } from '$app/types';
  import SearchEmpty from './search-empty.svelte';

  const i18n = useI18n();

  let {
    canReset,
    resetHref,
    embedded = false,
  }: { canReset: boolean; resetHref: ResolvedPathname; embedded?: boolean } = $props();
</script>

<SearchEmpty
  {embedded}
  title={canReset ? i18n.t('Oops, no athletes found.') : i18n.t('No athletes to show yet.')}
  description={canReset ? undefined : i18n.t('No ranked results in this view yet.')}
  resetHref={canReset ? resetHref : undefined}
/>
