import { TABLE_PAGE_SIZE } from '$lib/constants/pagination';
import { competitionsService, rankingsService } from '$lib/server/api';
import { defaultRankingSort, risProvenance } from '$lib/constants/ranking';
import { formatAthleteName } from '$lib/utils/format';
import type { PageServerLoad } from './$types';
import { error, redirect } from '@sveltejs/kit';
import { asRankedGender, asRankingMetric } from '$lib/types/enums';

const noRankings = () => ({
  data: [],
  pagination: { page: 1, page_size: TABLE_PAGE_SIZE, total_items: 0, total_pages: 0 },
});

export const load: PageServerLoad = async ({ params, url }) => {
  let competition;
  try {
    competition = await competitionsService.getById(params.slug);
  } catch (err) {
    console.error('Failed to load competition', { slug: params.slug, error: err });
    throw error(404, 'Competition not found');
  }

  const ris = risProvenance(competition);

  if (competition.categories.length === 0) {
    const empty = noRankings();
    return {
      competition,
      ris,
      classes: [],
      countries: [],
      initialRankings: empty.data,
      pagination: empty.pagination,
    };
  }

  const movement = asRankingMetric(url.searchParams.get('movement')) ?? defaultRankingSort(ris);
  const direction = url.searchParams.get('direction') === 'asc' ? 'asc' : 'desc';
  const gender = asRankedGender(url.searchParams.get('gender'));
  const category = url.searchParams.get('category') || null;
  const country = url.searchParams.get('country') || null;
  const q = url.searchParams.get('q') || null;
  const page = Number(url.searchParams.get('page') ?? 1) || 1;

  const focused = url.searchParams.get('athlete');
  if (focused && !url.searchParams.has('page') && !q) {
    const participant = competition.categories
      .flatMap((category) => category.participants)
      .find((participant) => participant.athlete.slug === focused);
    if (participant) {
      // Name search preserves the unfiltered rank, so the link opens the page containing the athlete.
      const match = await rankingsService
        .getGlobalRankings({
          page: 1,
          page_size: TABLE_PAGE_SIZE,
          movement,
          direction,
          gender,
          category,
          country,
          q: formatAthleteName(participant.athlete),
          competition_id: competition.competition_id,
        })
        .catch(() => noRankings());
      const entry = match.data.find((entry) => entry.athlete.slug === focused);
      if (entry) {
        const targetPage = Math.ceil(entry.rank / match.pagination.page_size);
        if (targetPage > 1) {
          const target = new URL(url);
          target.searchParams.set('page', String(targetPage));
          redirect(307, `${target.pathname}${target.search}`);
        }
      }
    }
  }

  const [classes, countries, rankings] = await Promise.all([
    rankingsService.getRankingClasses(gender, competition.competition_id).catch(() => []),
    rankingsService.getRankingCountries(competition.competition_id).catch(() => []),
    rankingsService
      .getGlobalRankings({
        page,
        page_size: TABLE_PAGE_SIZE,
        movement,
        direction,
        gender,
        category,
        country,
        q,
        competition_id: competition.competition_id,
      })
      .catch((err) => {
        console.error('Failed to load competition rankings', { slug: params.slug, error: err });
        error(502, 'Competition results are temporarily unavailable');
      }),
  ]);

  return {
    competition,
    ris,
    classes,
    countries,
    initialRankings: rankings.data,
    pagination: rankings.pagination,
  };
};
