import type { RequestHandler } from './$types';
import { athletesService } from '#lib/server/api/index.js';
import { ATHLETE_SEARCH_LIMIT } from '#lib/constants/athlete-search.js';

export const GET: RequestHandler = async ({ url }) =>
  Response.json(
    await athletesService.search(url.searchParams.get('q') ?? '', ATHLETE_SEARCH_LIMIT)
  );
