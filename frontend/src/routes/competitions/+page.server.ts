import { logger } from '#lib/server/logger.js';
import { TABLE_PAGE_SIZE } from '#lib/constants/pagination.js';
import { competitionsService } from '#lib/server/api/index.js';
import type { PageServerLoad } from './$types';
import type { CompetitionStatus } from '#lib/types/competition.js';
import { COMPETITION_STATUS_FILTERS } from '#lib/constants/competition.js';
import { COMPETITION_FILTERS } from '#lib/constants/filters.js';
import { keepSelected, readFilters, text } from '#lib/utils/filters.js';

const RESULTS_STATUS: CompetitionStatus = 'completed';
const UPCOMING_STATUS: CompetitionStatus = 'upcoming';

function readStatus(raw: string | null): CompetitionStatus | undefined {
  return COMPETITION_STATUS_FILTERS.find((status) => status.value === raw)?.value;
}

export const load: PageServerLoad = async ({ url }) => {
  const status = readStatus(url.searchParams.get('status')) ?? RESULTS_STATUS;
  const filters = readFilters(url.searchParams, COMPETITION_FILTERS);
  const q = text(url.searchParams.get('q'));
  const page = Number(url.searchParams.get('page') ?? 1) || 1;

  const otherStatuses = COMPETITION_STATUS_FILTERS.filter((option) => option.value !== status);

  try {
    const [{ data: competitions, pagination }, facets, otherResults] = await Promise.all([
      competitionsService.getAll({
        status,
        ...filters,
        q,
        direction: status === UPCOMING_STATUS ? 'asc' : 'desc',
        page,
        page_size: TABLE_PAGE_SIZE,
      }),
      competitionsService.getFacets({ status, ...filters }),
      Promise.all(
        otherStatuses.map(({ value }) =>
          competitionsService
            .getAll({
              status: value,
              ...filters,
              q,
              page_size: value === 'live' ? 5 : 1,
            })
            .then((response) => ({ status: value, ...response }))
            .catch(() => null)
        )
      ),
    ]);

    const counts = {
      [status]: pagination.total_items,
      ...Object.fromEntries(
        otherResults
          .filter((result) => result !== null)
          .map((result) => [result.status, result.pagination.total_items])
      ),
    } as Partial<Record<CompetitionStatus, number>>;

    return {
      competitions,
      runningCompetitions: otherResults.find((result) => result?.status === 'live')?.data ?? [],
      pagination,
      facets: keepSelected(facets, {
        federations: filters.federation,
        countries: filters.country,
        years: filters.year,
        formats: filters.event,
      }),
      counts,
      status,
      filters,
      q,
    };
  } catch (error) {
    logger.error({ status, page, err: error }, 'Competition directory load failed');
    return {
      competitions: [],
      runningCompetitions: [],
      pagination: { page: 1, page_size: TABLE_PAGE_SIZE, total_items: 0, total_pages: 0 },
      facets: { federations: [], years: [], countries: [], formats: [] },
      counts: {} as Partial<Record<CompetitionStatus, number>>,
      status,
      filters,
      q,
      error: 'Failed to load competitions',
    };
  }
};
