<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';

  const i18n = useI18n();
</script>

<span title={i18n.t('Rank')}
  ><span aria-hidden="true">#</span><span class="sr-only">{i18n.t('Rank')}</span></span
>
