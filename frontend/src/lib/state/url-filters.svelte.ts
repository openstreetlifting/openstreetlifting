import { readFilters, type FilterSpec, type FilterValues } from '#lib/utils/filters.js';

export class UrlFilters<S extends FilterSpec> {
  values: FilterValues<S>;
  readonly spec: S;

  constructor(spec: S, initial: FilterValues<S>) {
    this.spec = spec;
    this.values = $state({ ...initial });
  }

  sync(values: FilterValues<S>) {
    this.values = { ...values };
  }

  get activeCount(): number {
    return Object.values(this.values).filter(Boolean).length;
  }

  writeTo(params: URLSearchParams) {
    for (const [name, value] of Object.entries(this.values)) {
      if (value) params.set(name, String(value));
    }
  }

  clear() {
    this.values = readFilters({ get: () => null }, this.spec);
  }
}
