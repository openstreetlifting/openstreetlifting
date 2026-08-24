import { competitionsService } from '$lib/server/api';
import type { PageServerLoad } from './$types';
import type { CompetitionStatus } from '$lib/types/competition';
import { COMPETITION_STATUS_FILTERS } from '$lib/constants/competition';

function readStatus(raw: string | null): CompetitionStatus | undefined {
  return COMPETITION_STATUS_FILTERS.find((status) => status.value === raw)?.value;
}

export const load: PageServerLoad = async ({ url }) => {
  const status = readStatus(url.searchParams.get('status'));
  const page = Number(url.searchParams.get('page') ?? 1) || 1;

  try {
    const { data: competitions, pagination } = await competitionsService.getAll({ status, page });

    return { competitions, pagination, status };
  } catch (error) {
    console.error('Failed to fetch competitions:', error);
    return {
      competitions: [],
      pagination: { page: 1, page_size: 50, total_items: 0, total_pages: 0 },
      status,
      error: 'Failed to load competitions',
    };
  }
};
