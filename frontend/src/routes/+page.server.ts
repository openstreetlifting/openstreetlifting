import { rankingsService } from '$lib/server/api';
import type { PageServerLoad } from './$types';

const title = 'OpenStreetlifting';
const description =
  'OpenStreetlifting is an open, collaborative project building a permanent and traceable archive of all Streetlifting data, freely accessible to everyone.';

export const load: PageServerLoad = async ({ url }) => {
  const gender = url.searchParams.get('gender') || null;

  const [classes, years, countries] = await Promise.all([
    rankingsService.getRankingClasses(gender).catch(() => []),
    rankingsService.getRankingYears().catch(() => []),
    rankingsService.getRankingCountries().catch(() => []),
  ]);

  try {
    const movement = url.searchParams.get('movement') || 'ris';
    const direction = url.searchParams.get('direction') === 'asc' ? 'asc' : 'desc';
    const country = url.searchParams.get('country') || null;
    const q = url.searchParams.get('q') || null;
    const category = url.searchParams.get('category') || null;
    const year = Number(url.searchParams.get('year')) || null;
    const page = Number(url.searchParams.get('page') ?? 1) || 1;

    const initialData = await rankingsService.getGlobalRankings({
      page,
      movement,
      direction,
      gender,
      country,
      category,
      year,
      q,
    });

    return {
      title,
      description,
      initialRankings: initialData.data,
      pagination: initialData.pagination,
      classes,
      years,
      countries,
    };
  } catch (error) {
    console.error('Error loading rankings:', error);
    return {
      title,
      description,
      error: error instanceof Error ? error.message : 'Failed to load rankings',
      initialRankings: [],
      pagination: {
        page: 1,
        page_size: 50,
        total_items: 0,
        total_pages: 0,
      },
      classes,
      years,
      countries,
    };
  }
};
