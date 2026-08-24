import { competitionsService, rankingsService } from '$lib/server/api';
import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';

// A fresh object per call: one shared literal would be handed to every request
// this server process serves.
const noRankings = () => ({
  data: [],
  pagination: { page: 1, page_size: 50, total_items: 0, total_pages: 0 },
});

export const load: PageServerLoad = async ({ params, url }) => {
  let competition;
  try {
    competition = await competitionsService.getById(params.slug);
  } catch (err) {
    console.error('Failed to fetch competition:', err);
    throw error(404, 'Competition not found');
  }

  if (competition.categories.length === 0) {
    const empty = noRankings();
    return {
      competition,
      classes: [],
      countries: [],
      initialRankings: empty.data,
      pagination: empty.pagination,
    };
  }

  const movement = url.searchParams.get('movement') || 'ris';
  const direction = url.searchParams.get('direction') === 'asc' ? 'asc' : 'desc';
  const gender = url.searchParams.get('gender') || null;
  const category = url.searchParams.get('category') || null;
  const country = url.searchParams.get('country') || null;
  const q = url.searchParams.get('q') || null;
  const page = Number(url.searchParams.get('page') ?? 1) || 1;

  const [classes, countries, rankings] = await Promise.all([
    rankingsService.getRankingClasses(gender, competition.competition_id).catch(() => []),
    rankingsService.getRankingCountries(competition.competition_id).catch(() => []),
    rankingsService
      .getGlobalRankings({
        page,
        movement,
        direction,
        gender,
        category,
        country,
        q,
        competition_id: competition.competition_id,
      })
      .catch(() => noRankings()),
  ]);

  return {
    competition,
    classes,
    countries,
    initialRankings: rankings.data,
    pagination: rankings.pagination,
  };
};
