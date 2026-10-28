import { getContext, onMount, setContext } from 'svelte';
import type { Locale } from '#lib/i18n/locale.js';
import { formatWeight } from '#lib/utils/format.js';
import {
  displayWeight,
  weightUnit,
  WEIGHT_UNIT_STORAGE_KEY,
  type WeightUnit,
} from '#lib/utils/weight-unit.js';

export class WeightPreference {
  #selected = $state<WeightUnit>();

  constructor(private readonly initial: () => WeightUnit = () => 'kg') {}

  get unit(): WeightUnit {
    return this.#selected ?? this.initial();
  }

  set unit(value: WeightUnit) {
    this.#selected = value;
  }

  restore(): void {
    try {
      this.unit = weightUnit(localStorage.getItem(WEIGHT_UNIT_STORAGE_KEY));
    } catch {
      // Keep the default when browser storage is unavailable.
    }
  }

  select(unit: WeightUnit): void {
    this.unit = unit;
    try {
      localStorage.setItem(WEIGHT_UNIT_STORAGE_KEY, unit);
    } catch {
      // Unit switching still works for this page when persistence is blocked.
    }
  }

  value(kilograms: number): number {
    return displayWeight(kilograms, this.unit);
  }

  format(value: number | string | null | undefined, locale: Locale, digits?: number): string {
    return formatWeight(value, locale, this.unit, digits);
  }

  category(label: string | null | undefined, locale: Locale): string {
    if (!label) return '';
    if (this.unit === 'kg') return label;
    return label.replace(
      /(^|\s)([+-])(\d+(?:\.\d+)?)(?:\s*kg)?$/i,
      (_, prefix, sign, value) => `${prefix}${sign}${this.format(value, locale)} lb`
    );
  }
}

export const WEIGHT_UNIT_CONTEXT = Symbol('weight-unit');

export function provideWeightUnit(): WeightPreference {
  const preference = setContext(WEIGHT_UNIT_CONTEXT, new WeightPreference());
  onMount(() => preference.restore());
  return preference;
}

export function useWeightUnit(): WeightPreference {
  return getContext<WeightPreference | undefined>(WEIGHT_UNIT_CONTEXT) ?? new WeightPreference();
}
