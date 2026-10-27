<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import type { ResolvedPathname } from '$app/types';
  import SearchEmpty from './search-empty.svelte';

  const i18n = useI18n();

  let { canReset, resetHref }: { canReset: boolean; resetHref: ResolvedPathname } = $props();
</script>

<SearchEmpty
  title={canReset ? i18n.t('Oops, no athletes found.') : i18n.t('No athletes to show yet.')}
  description={canReset
    ? i18n.t('Lighten the filters or try another name.')
    : i18n.t('No ranked results in this view yet.')}
  resetHref={canReset ? resetHref : undefined}
/>
