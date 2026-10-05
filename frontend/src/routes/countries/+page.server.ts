import { error } from '@sveltejs/kit';
import { rankingsService } from '#lib/server/api/index.js';
import { allAthletes, summarizeCountries } from '#lib/server/countries.js';
import { publishedCompetitions } from '#lib/server/federations.js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
  try {
    const [competitions, athletes, rankedCountries] = await Promise.all([
      publishedCompetitions(),
      allAthletes(),
      rankingsService.getRankingCountries(),
    ]);
    return { countries: summarizeCountries(competitions, athletes, rankedCountries) };
  } catch (err) {
    console.error('Failed to load countries', err);
    error(503, 'Countries are unavailable right now');
  }
};
