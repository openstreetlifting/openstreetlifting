import { error } from '@sveltejs/kit';
import { publishedCompetitions, summarizeFederations } from '$lib/server/federations';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
  try {
    const federations = summarizeFederations(await publishedCompetitions());
    return { federations };
  } catch (err) {
    console.error('Failed to load federations', err);
    error(503, 'Federations are unavailable right now');
  }
};
