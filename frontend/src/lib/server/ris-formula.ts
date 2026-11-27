import { risService } from '#lib/server/api/index.js';
import type { components } from '#lib/api/schema.generated.js';

type RisFormula = components['schemas']['RisFormulaResponse'];

const CACHE_TTL = 3_600_000;

let cache: { value: RisFormula | null; expires: number } | null = null;

export async function currentRisFormula(): Promise<RisFormula | null> {
  if (cache && cache.expires > Date.now()) return cache.value;
  try {
    const formulas = await risService.getFormulas();
    const value = formulas.find((formula) => formula.is_current) ?? null;
    cache = { value, expires: Date.now() + CACHE_TTL };
    return value;
  } catch {
    return null;
  }
}
