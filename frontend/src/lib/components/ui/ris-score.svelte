<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { formatScore } from '#lib/utils/index.js';
  import { REPORTED_GLYPH, REPORTED_MARK, REPORTED_TITLE } from '#lib/constants/table.js';
  import type { RisSource } from '#lib/types/competition.js';

  const i18n = useI18n();

  let {
    value,
    source,
  }: {
    value: number | string | null;
    source: RisSource | null;
  } = $props();
</script>

{formatScore(value, i18n.locale)}{#if source === 'reported'}<span
    class={REPORTED_MARK}
    title={i18n.t(REPORTED_TITLE)}
    aria-label={i18n.t(REPORTED_TITLE)}>{REPORTED_GLYPH}</span
  >{/if}
