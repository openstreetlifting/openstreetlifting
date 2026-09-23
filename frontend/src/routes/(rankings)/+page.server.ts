import { MAIN_RANKING_PAGE_SIZE } from '$lib/constants/pagination';
import { rankingsService } from '$lib/server/api';
import { asRankedGender, asRankingMetric } from '$lib/types/enums';
import type { PageServerLoad } from './$types';

const title = 'Streetlifting rankings and records';
const description =
  'Global Streetlifting rankings from every competition in the archive: muscle up, pull up, dips and squat results, RIS scores and athlete records.';

export const load: PageServerLoad = async ({ url }) => {
  const gender = asRankedGender(url.searchParams.get('gender'));

  try {
    const movement = asRankingMetric(url.searchParams.get('movement')) ?? 'ris';
    const direction = url.searchParams.get('direction') === 'asc' ? 'asc' : 'desc';
    const country = url.searchParams.get('country') || null;
    const federation = url.searchParams.get('federation') || null;
    const q = url.searchParams.get('q') || null;
    const category = url.searchParams.get('category') || null;
    const year = Number(url.searchParams.get('year')) || null;
    const page = Number(url.searchParams.get('page') ?? 1) || 1;

    const initialData = await rankingsService.getGlobalRankings({
      page,
      page_size: MAIN_RANKING_PAGE_SIZE,
      movement,
      direction,
      gender,
      country,
      federation,
      category,
      year,
      q,
    });

    return {
      title,
      description,
      initialRankings: initialData.data,
      pagination: initialData.pagination,
    };
  } catch (error) {
    console.error('Failed to load rankings', error);
    return {
      title,
      description,
      error: error instanceof Error ? error.message : 'Failed to load rankings',
      initialRankings: [],
      pagination: {
        page: 1,
        page_size: MAIN_RANKING_PAGE_SIZE,
        total_items: 0,
        total_pages: 0,
      },
    };
  }
};
