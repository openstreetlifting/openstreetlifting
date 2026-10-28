<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { useWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import { weightUnit } from '#lib/utils/weight-unit.js';
  import { FIELD } from '#lib/constants/typography.js';

  const i18n = useI18n();
  const weights = useWeightUnit();
</script>

<label for="weight-unit" class="sr-only">{i18n.t('Weight unit')}</label>
<select
  id="weight-unit"
  value={weights.unit}
  onchange={(event) => {
    weights.select(weightUnit(event.currentTarget.value));
  }}
  class="{FIELD} px-2 py-1 text-sm"
>
  <option value="kg">kg</option>
  <option value="lb">lb</option>
</select>
