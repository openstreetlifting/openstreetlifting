import { logger } from '#lib/server/logger.js';
import { MAIN_RANKING_PAGE_SIZE } from '#lib/constants/pagination.js';
import { rankingsService } from '#lib/server/api/index.js';
import { SITE_NAME, SITE_DESCRIPTION } from '#lib/seo/site.js';
import { asRankingMetric } from '#lib/types/enums.js';
import { RANKING_FILTERS } from '#lib/constants/filters.js';
import { readFilters } from '#lib/utils/filters.js';
import { isEventScoped } from '#lib/utils/ranking-scope.js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url }) => {
  try {
    const movement = asRankingMetric(url.searchParams.get('movement')) ?? 'ris';
    const direction = url.searchParams.get('direction') === 'asc' ? 'asc' : 'desc';
    const q = url.searchParams.get('q') || null;
    const page = Number(url.searchParams.get('page') ?? 1) || 1;
    const filters = readFilters(url.searchParams, RANKING_FILTERS);

    const [initialData, allEvents] = await Promise.all([
      rankingsService.getGlobalRankings({
        page,
        page_size: MAIN_RANKING_PAGE_SIZE,
        movement,
        direction,
        ...filters,
        q,
      }),
      q && isEventScoped(movement)
        ? rankingsService
            .getGlobalRankings({
              page: 1,
              page_size: 1,
              movement: 'name',
              direction,
              ...filters,
              q,
            })
            .catch(() => null)
        : null,
    ]);

    const hidden = allEvents
      ? Math.max(0, allEvents.pagination.total_items - initialData.pagination.total_items)
      : 0;

    return {
      title: SITE_NAME,
      description: SITE_DESCRIPTION,
      initialRankings: initialData.data,
      pagination: initialData.pagination,
      otherEventAthletes: hidden,
    };
  } catch (error) {
    logger.error({ err: error }, 'Global rankings load failed');
    return {
      title: SITE_NAME,
      description: SITE_DESCRIPTION,
      error: error instanceof Error ? error.message : 'Failed to load rankings',
      initialRankings: [],
      pagination: {
        page: 1,
        page_size: MAIN_RANKING_PAGE_SIZE,
        total_items: 0,
        total_pages: 0,
      },
      otherEventAthletes: 0,
    };
  }
};
