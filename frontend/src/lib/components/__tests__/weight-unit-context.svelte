<script lang="ts">
  import { provideWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import WeightUnitPicker from '../weight-unit-picker.svelte';

  provideWeightUnit();
</script>

<WeightUnitPicker />
