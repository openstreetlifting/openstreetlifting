import { afterEach, expect, it, vi } from 'vitest';
import { render } from 'vitest-browser-svelte';
import { I18N_CONTEXT, I18n } from '#lib/i18n/index.js';
import { WEIGHT_UNIT_CONTEXT, WeightPreference } from '#lib/state/weight-unit.svelte.js';
import { WEIGHT_UNIT_STORAGE_KEY } from '#lib/utils/weight-unit.js';
import WeightUnitPicker from './weight-unit-picker.svelte';
import WeightUnitContext from './__tests__/weight-unit-context.svelte';

afterEach(() => {
  vi.restoreAllMocks();
  localStorage.removeItem(WEIGHT_UNIT_STORAGE_KEY);
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
  expect(localStorage.getItem(WEIGHT_UNIT_STORAGE_KEY)).toBe('lb');
  const restored = new WeightPreference();
  restored.restore();
  expect(restored.unit).toBe('lb');
  await picker.selectOptions('kg');
  expect(weights.unit).toBe('kg');
  expect(localStorage.getItem(WEIGHT_UNIT_STORAGE_KEY)).toBe('kg');
});

it.each(['lb', 'kg', 'invalid', null])(
  'restores %s from browser storage with kilograms as the fallback',
  async (saved) => {
    if (saved !== null) localStorage.setItem(WEIGHT_UNIT_STORAGE_KEY, saved);
    const screen = await render(WeightUnitContext);
    await expect
      .element(screen.getByRole('combobox', { name: 'Weight unit' }))
      .toHaveValue(saved === 'lb' ? 'lb' : 'kg');
  }
);

it('still switches units when browser storage cannot be read or written', async () => {
  vi.spyOn(Storage.prototype, 'getItem').mockImplementation(() => {
    throw new DOMException('Storage blocked', 'SecurityError');
  });
  vi.spyOn(Storage.prototype, 'setItem').mockImplementation(() => {
    throw new DOMException('Storage full', 'QuotaExceededError');
  });
  const weights = new WeightPreference();
  weights.restore();
  expect(weights.unit).toBe('kg');
  const screen = await render(WeightUnitPicker, {
    context: new Map([[WEIGHT_UNIT_CONTEXT, weights]]),
  });
  await screen.getByRole('combobox', { name: 'Weight unit' }).selectOptions('lb');
  expect(weights.unit).toBe('lb');
  await expect.element(screen.getByRole('combobox', { name: 'Weight unit' })).toHaveValue('lb');
});
