import type { RequestHandler } from './$types';
import { rankingsService } from '$lib/server/api';

export const GET: RequestHandler = async ({ url }) => {
  const data = await rankingsService.getGlobalRankings({
    pagination: Number(url.searchParams.get('pagination') ?? 1),
    movement: url.searchParams.get('movement') ?? 'total',
    gender: url.searchParams.get('gender') ?? null,
    country: url.searchParams.get('country') ?? null,
  });

  return Response.json(data);
};
