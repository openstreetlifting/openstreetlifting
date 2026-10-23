import type { components } from '#lib/api/schema.generated.js';

type Schemas = components['schemas'];

export type RisConstants = Schemas['GenderConstants'];
export type RisFormula = Schemas['RisFormulaResponse'];
export type RisPerformance = Omit<Schemas['RisPerformanceResponse'], 'bodyweight' | 'total'> & {
  bodyweight: number;
  total: number;
};
