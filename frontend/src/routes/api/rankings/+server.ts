import type { RequestHandler } from './$types';
import { rankingsService } from '#lib/server/api/index.js';
import { RANKING_FILTERS } from '#lib/constants/filters.js';
import { asRankingMetric } from '#lib/types/enums.js';
import { readFilters } from '#lib/utils/filters.js';

export const GET: RequestHandler = async ({ url }) => {
  const data = await rankingsService.getGlobalRankings({
    page: Number(url.searchParams.get('page') ?? 1),
    movement: asRankingMetric(url.searchParams.get('movement')) ?? 'ris',
    direction: url.searchParams.get('direction') === 'asc' ? 'asc' : 'desc',
    ...readFilters(url.searchParams, RANKING_FILTERS),
    q: url.searchParams.get('q') ?? null,
    competition_id: url.searchParams.get('competition_id') ?? null,
  });

  return Response.json(data);
};
