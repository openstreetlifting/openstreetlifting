import { competitionsService } from '$lib/server/api';
import type { PageServerLoad } from './$types';
import type { CompetitionStatus } from '$lib/types/competition';
import { COMPETITION_STATUS_FILTERS } from '$lib/constants/competition';

const RESULTS_STATUS: CompetitionStatus = 'completed';
const UPCOMING_STATUS: CompetitionStatus = 'upcoming';

function readStatus(raw: string | null): CompetitionStatus | undefined {
  return COMPETITION_STATUS_FILTERS.find((status) => status.value === raw)?.value;
}

function readText(raw: string | null): string | undefined {
  const value = raw?.trim();
  return value ? value : undefined;
}

export const load: PageServerLoad = async ({ url }) => {
  const status = readStatus(url.searchParams.get('status')) ?? RESULTS_STATUS;
  const federation = readText(url.searchParams.get('federation'));
  const country = readText(url.searchParams.get('country'));
  const q = readText(url.searchParams.get('q'));
  const year = Number(url.searchParams.get('year')) || undefined;
  const page = Number(url.searchParams.get('page') ?? 1) || 1;

  const other = status === UPCOMING_STATUS ? RESULTS_STATUS : UPCOMING_STATUS;

  try {
    const [{ data: competitions, pagination }, facets, otherCount] = await Promise.all([
      competitionsService.getAll({
        status,
        federation,
        country,
        year,
        q,
        direction: status === UPCOMING_STATUS ? 'asc' : 'desc',
        page,
      }),
      competitionsService.getFacets(),
      competitionsService
        .getAll({ status: other, federation, country, year, q, page_size: 1 })
        .then((response) => response.pagination.total_items)
        .catch(() => null),
    ]);

    const counts = {
      [status]: pagination.total_items,
      ...(otherCount === null ? {} : { [other]: otherCount }),
    } as Partial<Record<CompetitionStatus, number>>;

    return { competitions, pagination, facets, counts, status, federation, country, year, q };
  } catch (error) {
    console.error('Failed to fetch competitions:', error);
    return {
      competitions: [],
      pagination: { page: 1, page_size: 50, total_items: 0, total_pages: 0 },
      facets: { federations: [], years: [], countries: [] },
      counts: {} as Partial<Record<CompetitionStatus, number>>,
      status,
      federation,
      country,
      year,
      q,
      error: 'Failed to load competitions',
    };
  }
};
