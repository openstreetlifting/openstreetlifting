import { competitionsService } from '$lib/server/api';
import type { PageServerLoad } from './$types';
import type { Competition, CompetitionStatus } from '$lib/types/competition';
import { COMPETITION_STATUS_FILTERS } from '$lib/constants/competition';

// The archive is what people come for, so results are the default list and the
// calendar rides above it. Asking for a status explicitly overrides both.
const RESULTS_STATUS: CompetitionStatus = 'completed';
const UPCOMING_PREVIEW = 5;

function readStatus(raw: string | null): CompetitionStatus | 'all' | undefined {
  if (raw === 'all') return 'all';
  return COMPETITION_STATUS_FILTERS.find((status) => status.value === raw)?.value;
}

function readText(raw: string | null): string | undefined {
  const value = raw?.trim();
  return value ? value : undefined;
}

export const load: PageServerLoad = async ({ url }) => {
  const status = readStatus(url.searchParams.get('status'));
  const federation = readText(url.searchParams.get('federation'));
  const country = readText(url.searchParams.get('country'));
  const q = readText(url.searchParams.get('q'));
  const year = Number(url.searchParams.get('year')) || undefined;
  const page = Number(url.searchParams.get('page') ?? 1) || 1;

  const listed = status === 'all' ? undefined : (status ?? RESULTS_STATUS);
  const narrowed = Boolean(status || federation || country || q || year);

  try {
    const [{ data: competitions, pagination }, facets] = await Promise.all([
      competitionsService.getAll({
        status: listed,
        federation,
        country,
        year,
        q,
        direction: listed === 'upcoming' ? 'asc' : 'desc',
        page,
      }),
      competitionsService.getFacets(),
    ]);

    // The calendar is a header, not a filter result, so a narrowed page drops it
    // rather than answering a question nobody asked.
    let upcoming: Competition[] = [];
    if (!narrowed && page === 1) {
      const next = await competitionsService.getAll({
        status: 'upcoming',
        direction: 'asc',
        page_size: UPCOMING_PREVIEW,
      });
      upcoming = next.data;
    }

    return { competitions, pagination, facets, upcoming, status, federation, country, year, q };
  } catch (error) {
    console.error('Failed to fetch competitions:', error);
    return {
      competitions: [],
      pagination: { page: 1, page_size: 50, total_items: 0, total_pages: 0 },
      facets: { federations: [], years: [], countries: [] },
      upcoming: [],
      status,
      federation,
      country,
      year,
      q,
      error: 'Failed to load competitions',
    };
  }
};
