import { logger } from '#lib/server/logger.js';
import { error } from '@sveltejs/kit';
import { publishedCompetitions, summarizeFederations } from '#lib/server/federations.js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
  try {
    const federations = summarizeFederations(await publishedCompetitions());
    return { federations };
  } catch (err) {
    logger.error({ err }, 'Federation directory load failed');
    error(503, 'Federations are unavailable right now');
  }
};
