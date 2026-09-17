import type { Paginated } from '$lib/types/pagination';

export async function collect<T>(fetchPage: (page: number) => Promise<Paginated<T>>): Promise<T[]> {
  const first = await fetchPage(1);
  const remaining = Math.max(first.pagination.total_pages - 1, 0);
  const rest = await Promise.all(
    Array.from({ length: remaining }, (_, index) => fetchPage(index + 2))
  );

  return [first, ...rest].flatMap((response) => response.data);
}
