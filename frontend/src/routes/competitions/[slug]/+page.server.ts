import { logger } from '#lib/server/logger.js';
import { competitionsService } from '#lib/server/api/index.js';
import { currentRisFormula } from '#lib/server/ris-formula.js';
import { ApiError } from '#lib/server/api/client.js';
import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params }) => {
  let competition;
  try {
    competition = await competitionsService.getById(params.slug);
  } catch (err) {
    if (err instanceof ApiError && err.status === 404) {
      error(404, 'Competition not found');
    }
    logger.error({ err }, 'Competition page load failed');
    error(503, 'Competition details are unavailable right now');
  }

  const formula = competition.categories.length > 0 ? await currentRisFormula() : null;
  return { competition, formula };
};
