import { getContext, onMount, setContext } from 'svelte';
import type { Locale } from '#lib/i18n/locale.js';
import { formatWeight } from '#lib/utils/format.js';
import {
  displayWeight,
  weightUnit,
  WEIGHT_UNIT_COOKIE,
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

export function provideWeightUnit(initial: () => WeightUnit): WeightPreference {
  const preference = setContext(WEIGHT_UNIT_CONTEXT, new WeightPreference(initial));
  onMount(() => {
    // Cached offline pages can have been rendered before the preference changed.
    const saved = document.cookie
      .split(';')
      .find((entry) => entry.trim().startsWith(`${WEIGHT_UNIT_COOKIE}=`));
    preference.unit = weightUnit(saved?.split('=')[1]);
  });
  return preference;
}

export function useWeightUnit(): WeightPreference {
  return getContext<WeightPreference | undefined>(WEIGHT_UNIT_CONTEXT) ?? new WeightPreference();
}
