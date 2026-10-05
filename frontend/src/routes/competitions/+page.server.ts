import { TABLE_PAGE_SIZE } from '#lib/constants/pagination.js';
import { competitionsService } from '#lib/server/api/index.js';
import type { PageServerLoad } from './$types';
import type { CompetitionStatus } from '#lib/types/competition.js';
import { COMPETITION_STATUS_FILTERS } from '#lib/constants/competition.js';
import { normalizeEvent } from '#lib/utils/competition-format.js';

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
  const event = normalizeEvent(url.searchParams.get('event')) || undefined;
  const year = Number(url.searchParams.get('year')) || undefined;
  const page = Number(url.searchParams.get('page') ?? 1) || 1;

  const otherStatuses = COMPETITION_STATUS_FILTERS.filter((option) => option.value !== status);

  try {
    const [{ data: competitions, pagination }, facets, otherResults] = await Promise.all([
      competitionsService.getAll({
        status,
        federation,
        country,
        year,
        q,
        event,
        direction: status === UPCOMING_STATUS ? 'asc' : 'desc',
        page,
        page_size: TABLE_PAGE_SIZE,
      }),
      competitionsService.getFacets(),
      Promise.all(
        otherStatuses.map(({ value }) =>
          competitionsService
            .getAll({
              status: value,
              federation,
              country,
              year,
              q,
              event,
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
      facets,
      counts,
      status,
      federation,
      country,
      year,
      q,
      event,
    };
  } catch (error) {
    console.error('Failed to load competitions', { status, page, error });
    return {
      competitions: [],
      runningCompetitions: [],
      pagination: { page: 1, page_size: TABLE_PAGE_SIZE, total_items: 0, total_pages: 0 },
      facets: { federations: [], years: [], countries: [], formats: [] },
      counts: {} as Partial<Record<CompetitionStatus, number>>,
      status,
      federation,
      country,
      year,
      q,
      event,
      error: 'Failed to load competitions',
    };
  }
};
