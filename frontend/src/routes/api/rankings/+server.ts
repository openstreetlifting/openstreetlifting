import type { RequestHandler } from './$types';
import { rankingsService } from '$lib/server/api';

export const GET: RequestHandler = async ({ url }) => {
  const data = await rankingsService.getGlobalRankings({
    page: Number(url.searchParams.get('page') ?? 1),
    movement: url.searchParams.get('movement') ?? 'ris',
    direction: url.searchParams.get('direction') === 'asc' ? 'asc' : 'desc',
    gender: url.searchParams.get('gender') ?? null,
    country: url.searchParams.get('country') ?? null,
    category: url.searchParams.get('category') ?? null,
    year: Number(url.searchParams.get('year')) || null,
    competition_id: url.searchParams.get('competition_id') ?? null,
  });

  return Response.json(data);
};
