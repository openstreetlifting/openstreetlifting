<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { RECORD_BADGE, RECORD_MARK, RECORD_TITLE, RECORD_TONE } from '#lib/constants/table.js';
  import type { RecordMark } from '#lib/types/records.js';

  const i18n = useI18n();

  let {
    mark,
    worldOnly = false,
    badge = false,
  }: {
    mark: RecordMark | null | undefined;
    worldOnly?: boolean;
    badge?: boolean;
  } = $props();

  const shown = $derived(mark && (!worldOnly || mark === 'WR') ? mark : null);
</script>

{#if shown}<abbr
    class="{badge ? RECORD_BADGE : RECORD_MARK} {RECORD_TONE[shown]} no-underline"
    title={i18n.t(RECORD_TITLE[shown])}
    aria-label={i18n.t(RECORD_TITLE[shown])}>{shown}</abbr
  >{/if}
