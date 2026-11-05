<script lang="ts">
  import type { ResolvedPathname } from '$app/types';
  import ResultState from './result-state.svelte';

  let {
    title,
    description,
    resetHref,
    embedded = false,
  }: {
    title: string;
    description?: string;
    resetHref?: ResolvedPathname;
    embedded?: boolean;
  } = $props();
</script>

<ResultState {title} {description} {resetHref} {embedded} />
