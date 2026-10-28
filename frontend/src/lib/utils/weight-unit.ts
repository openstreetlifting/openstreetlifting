export const WEIGHT_UNIT_COOKIE = 'osl-weight-unit';
export type WeightUnit = 'kg' | 'lb';

// The international avoirdupois pound is exactly 0.45359237 kilograms.
const KILOGRAMS_PER_POUND = 0.45359237;

export function weightUnit(value: unknown): WeightUnit {
  return value === 'lb' ? 'lb' : 'kg';
}

export function displayWeight(kilograms: number, unit: WeightUnit): number {
  return unit === 'lb' ? kilograms / KILOGRAMS_PER_POUND : kilograms;
}

export function kilograms(value: number, unit: WeightUnit): number {
  return unit === 'lb' ? value * KILOGRAMS_PER_POUND : value;
}
