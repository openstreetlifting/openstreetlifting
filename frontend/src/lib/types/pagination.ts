import type { components } from '#lib/api/schema.generated.js';

export type PaginationMeta = components['schemas']['PaginationMeta'];
export type Paginated<T> = Omit<
  components['schemas']['PaginatedResponse_AthleteResponse'],
  'data'
> & { data: T[] };

export interface TablePagination extends Pick<
  PaginationMeta,
  'page' | 'total_items' | 'total_pages'
> {
  pageHref?: (page: number) => string;
  replaceState?: boolean;
}
