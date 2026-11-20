<script lang="ts">
  import Select from '#lib/components/ui/select.svelte';
  import { useI18n } from '#lib/i18n/index.js';
  import { useWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import { weightUnit } from '#lib/utils/weight-unit.js';
  import posthog from 'posthog-js';

  const i18n = useI18n();
  const weights = useWeightUnit();
</script>

<label for="weight-unit" class="sr-only">{i18n.t('Weight unit')}</label>
<Select
  id="weight-unit"
  value={weights.unit}
  onchange={(event) => {
    const unit = weightUnit(event.currentTarget.value);
    weights.select(unit);
    posthog.capture('weight_unit_selected', { weight_unit: unit });
  }}
  class="w-auto px-2"
>
  <option value="kg">kg</option>
  <option value="lb">lb</option>
</Select>
