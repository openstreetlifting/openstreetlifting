export type FilterSpec = Record<string, (raw: string | null) => unknown>;
export type FilterValues<S extends FilterSpec> = { [K in keyof S]: ReturnType<S[K]> };

export const text = (raw: string | null) => raw?.trim() || null;
export const year = (raw: string | null) => Number(raw) || null;

export function readFilters<S extends FilterSpec>(
  params: Pick<URLSearchParams, 'get'>,
  spec: S
): FilterValues<S> {
  return Object.fromEntries(
    Object.entries(spec).map(([name, parse]) => [name, parse(params.get(name))])
  ) as FilterValues<S>;
}

type Selected<F> = { [K in keyof F]?: F[K] extends (infer T)[] ? T | null : never };

export function keepSelected<F extends Record<string, unknown[]>>(
  facets: F,
  selected: Selected<F>
): F {
  const kept: Record<string, unknown[]> = { ...facets };
  for (const [name, value] of Object.entries(selected)) {
    if (value != null && !kept[name].includes(value)) kept[name] = [...kept[name], value];
  }
  return kept as F;
}
