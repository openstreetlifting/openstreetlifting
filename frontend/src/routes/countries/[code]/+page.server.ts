import { error, redirect } from '@sveltejs/kit';
import { rankingsService } from '$lib/server/api';
import { countryCodes, federationsBasedIn } from '$lib/server/countries';
import { isHeld, publishedCompetitions } from '$lib/server/federations';
import type { PageServerLoad } from './$types';

const byDate = (a: { start_date: string | null }, b: { start_date: string | null }) =>
  (a.start_date ?? '').localeCompare(b.start_date ?? '');

export const load: PageServerLoad = async ({ params, url }) => {
  if (params.code !== params.code.toLowerCase()) {
    redirect(308, `/countries/${params.code.toLowerCase()}${url.search}`);
  }

  const code = params.code.toUpperCase();
  const page = Number(url.searchParams.get('page') ?? 1) || 1;

  let loaded;
  try {
    loaded = await Promise.all([
      publishedCompetitions(),
      rankingsService.getRankingCountries(),
      rankingsService.getGlobalRankings({ page, movement: 'ris', country: code }),
    ]);
  } catch (err) {
    console.error('Failed to load country', { code, error: err });
    error(503, 'Country details are unavailable right now');
  }

  const [competitions, rankedCountries, rankings] = loaded;
  if (!countryCodes(competitions, rankedCountries).includes(code)) {
    error(404, 'Country not found');
  }

  const held = competitions.filter((competition) => competition.country === code);
  return {
    code,
    federations: federationsBasedIn(competitions, code),
    results: held.filter(isHeld).sort((a, b) => byDate(b, a)),
    upcoming: held.filter((competition) => competition.status === 'upcoming').sort(byDate),
    rankings: rankings.data,
    pagination: rankings.pagination,
  };
};
