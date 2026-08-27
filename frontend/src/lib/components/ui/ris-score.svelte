<script lang="ts">
  import { formatScore } from '$lib/utils';
  import { REPORTED_GLYPH, REPORTED_MARK, REPORTED_TITLE } from '$lib/constants/table';
  import type { RisSource } from '$lib/types/competition';

  let {
    value,
    source,
  }: {
    value: number | string | null;
    /** Null wherever no score exists, which the database enforces. */
    source: RisSource | null;
  } = $props();
</script>

{formatScore(value)}{#if source === 'reported'}<span
    class={REPORTED_MARK}
    title={REPORTED_TITLE}
    aria-label={REPORTED_TITLE}>{REPORTED_GLYPH}</span
  >{/if}
