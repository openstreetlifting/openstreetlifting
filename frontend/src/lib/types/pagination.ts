export interface PaginationMeta {
  page: number;
  page_size: number;
  total_items: number;
  total_pages: number;
}

export interface Paginated<T> {
  data: T[];
  pagination: PaginationMeta;
}

export interface TablePagination extends Pick<
  PaginationMeta,
  'page' | 'total_items' | 'total_pages'
> {
  pageHref?: (page: number) => string;
  replaceState?: boolean;
}
