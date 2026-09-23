<script lang="ts">
  import type { ResolvedPathname } from '$app/types';
  import SearchEmpty from './search-empty.svelte';

  let { canReset, resetHref }: { canReset: boolean; resetHref: ResolvedPathname } = $props();
</script>

<SearchEmpty
  title={canReset ? 'Oops, no athletes found.' : 'No athletes to show yet.'}
  description={canReset
    ? 'Lighten the filters or try another name.'
    : 'No ranked results in this view yet.'}
  resetHref={canReset ? resetHref : undefined}
/>
