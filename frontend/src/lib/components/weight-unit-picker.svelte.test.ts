import { afterEach, expect, it } from 'vitest';
import { render } from 'vitest-browser-svelte';
import { I18N_CONTEXT, I18n } from '#lib/i18n/index.js';
import { WEIGHT_UNIT_CONTEXT, WeightPreference } from '#lib/state/weight-unit.svelte.js';
import { WEIGHT_UNIT_COOKIE } from '#lib/utils/weight-unit.js';
import WeightUnitPicker from './weight-unit-picker.svelte';

afterEach(() => {
  document.cookie = `${WEIGHT_UNIT_COOKIE}=; Path=/; Max-Age=0`;
});

it('updates the shared preference immediately and saves it across languages and pages', async () => {
  const weights = new WeightPreference();
  const context = new Map<symbol, unknown>([
    [I18N_CONTEXT, new I18n(() => 'fr')],
    [WEIGHT_UNIT_CONTEXT, weights],
  ]);
  const screen = await render(WeightUnitPicker, { context });
  const picker = screen.getByRole('combobox', { name: 'Unité de poids' });
  await picker.selectOptions('lb');
  expect(weights.unit).toBe('lb');
  expect(document.cookie).toContain(`${WEIGHT_UNIT_COOKIE}=lb`);
  await picker.selectOptions('kg');
  expect(weights.unit).toBe('kg');
  expect(document.cookie).toContain(`${WEIGHT_UNIT_COOKIE}=kg`);
});
