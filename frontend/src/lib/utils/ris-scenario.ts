import type { WeightUnit } from './weight-unit.js';

export type RisWeightField = { text: string; unit: WeightUnit };
export type RisFields = Record<'bodyweight' | 'total', RisWeightField>;
export interface RisScenario {
  sex: 'men' | 'women';
  fields: RisFields;
  target: string;
}

export function readRisScenario(params: Pick<URLSearchParams, 'get'>): RisScenario {
  const field = (key: 'bodyweight' | 'total'): RisWeightField => ({
    text: params.get(key) ?? '',
    unit: params.get(`${key}_unit`) === 'lb' ? 'lb' : 'kg',
  });
  return {
    sex: params.get('sex') === 'women' ? 'women' : 'men',
    fields: { bodyweight: field('bodyweight'), total: field('total') },
    target: params.get('target') ?? '',
  };
}

export function writeRisScenario(url: Pick<URL, 'href'>, scenario: RisScenario): URL {
  const result = new URL(url.href);
  const set = (key: string, value: string) => {
    if (value) result.searchParams.set(key, value);
    else result.searchParams.delete(key);
  };
  set('sex', scenario.sex === 'women' ? 'women' : '');
  for (const key of ['bodyweight', 'total'] as const) {
    const field = scenario.fields[key];
    set(key, field.text);
    set(`${key}_unit`, field.text && field.unit === 'lb' ? 'lb' : '');
  }
  set('target', scenario.target);
  return result;
}
