import { logger } from '#lib/server/logger.js';
import { MAIN_RANKING_PAGE_SIZE } from '#lib/constants/pagination.js';
import { athletesService, rankingsService } from '#lib/server/api/index.js';
import { OTHER_ATHLETES_FETCHED, OTHER_ATHLETES_SHOWN } from '#lib/constants/athlete-search.js';
import { notInRanking } from '#lib/utils/athlete-search.js';
import type { AthleteSearchResponse } from '#lib/types/athlete.js';
import { SITE_NAME, SITE_DESCRIPTION } from '#lib/seo/site.js';
import { asRankingMetric } from '#lib/types/enums.js';
import { RANKING_FILTERS } from '#lib/constants/filters.js';
import { readFilters } from '#lib/utils/filters.js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url }) => {
  try {
    const movement = asRankingMetric(url.searchParams.get('movement')) ?? 'ris';
    const direction = url.searchParams.get('direction') === 'asc' ? 'asc' : 'desc';
    const q = url.searchParams.get('q') || null;
    const page = Number(url.searchParams.get('page') ?? 1) || 1;

    const [initialData, matches] = await Promise.all([
      rankingsService.getGlobalRankings({
        page,
        page_size: MAIN_RANKING_PAGE_SIZE,
        movement,
        direction,
        ...readFilters(url.searchParams, RANKING_FILTERS),
        q,
      }),
      q ? searchAthletes(q) : null,
    ]);

    return {
      title: SITE_NAME,
      description: SITE_DESCRIPTION,
      initialRankings: initialData.data,
      pagination: initialData.pagination,
      otherAthletes: matches
        ? otherAthletes(
            matches,
            initialData.data.map((entry) => entry.athlete.slug),
            initialData.pagination.total_items
          )
        : null,
    };
  } catch (error) {
    logger.error({ err: error }, 'Global rankings load failed');
    return {
      title: SITE_NAME,
      description: SITE_DESCRIPTION,
      error: error instanceof Error ? error.message : 'Failed to load rankings',
      initialRankings: [],
      otherAthletes: null,
      pagination: {
        page: 1,
        page_size: MAIN_RANKING_PAGE_SIZE,
        total_items: 0,
        total_pages: 0,
      },
    };
  }
};

async function searchAthletes(q: string): Promise<AthleteSearchResponse | null> {
  try {
    return await athletesService.search(q, OTHER_ATHLETES_FETCHED);
  } catch (error) {
    logger.warn({ err: error }, 'Athlete search failed');
    return null;
  }
}

function otherAthletes(matches: AthleteSearchResponse, rankedSlugs: string[], rankedTotal: number) {
  const hits = notInRanking(matches.data, rankedSlugs);
  const total = Math.max(hits.length, matches.total_items - rankedTotal);
  return total > 0 ? { hits: hits.slice(0, OTHER_ATHLETES_SHOWN), total } : null;
}
