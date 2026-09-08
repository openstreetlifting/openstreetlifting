import type { RequestHandler } from './$types';
import { rankingsService } from '$lib/server/api';
import { asRankedGender, asRankingMetric } from '$lib/types/enums';

export const GET: RequestHandler = async ({ url }) => {
  const data = await rankingsService.getGlobalRankings({
    page: Number(url.searchParams.get('page') ?? 1),
    movement: asRankingMetric(url.searchParams.get('movement')) ?? 'ris',
    direction: url.searchParams.get('direction') === 'asc' ? 'asc' : 'desc',
    gender: asRankedGender(url.searchParams.get('gender')),
    country: url.searchParams.get('country') ?? null,
    q: url.searchParams.get('q') ?? null,
    category: url.searchParams.get('category') ?? null,
    year: Number(url.searchParams.get('year')) || null,
    competition_id: url.searchParams.get('competition_id') ?? null,
  });

  return Response.json(data);
};
