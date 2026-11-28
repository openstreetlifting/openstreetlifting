import type { RequestHandler } from './$types';
import { athletesService } from '#lib/server/api/index.js';
import { ATHLETE_SEARCH_LIMIT } from '#lib/constants/athlete-search.js';

export const GET: RequestHandler = async ({ url }) => {
  const q = url.searchParams.get('q')?.trim() ?? '';
  if (!q) return Response.json({ data: [], total_items: 0 });

  return Response.json(await athletesService.search(q, ATHLETE_SEARCH_LIMIT));
};
