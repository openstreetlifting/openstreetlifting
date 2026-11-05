import { logger } from '#lib/server/logger.js';
import { TABLE_PAGE_SIZE } from '#lib/constants/pagination.js';
import { competitionsService, rankingsService, risService } from '#lib/server/api/index.js';
import { ApiError } from '#lib/server/api/client.js';
import { defaultRankingSort, risProvenance } from '#lib/constants/ranking.js';
import { formatAthleteName } from '#lib/utils/format.js';
import type { PageServerLoad } from './$types';
import { error, redirect } from '@sveltejs/kit';
import { asRankedGender, asRankingMetric } from '#lib/types/enums.js';

async function currentFormula() {
  try {
    const formulas = await risService.getFormulas();
    return formulas.find((formula) => formula.is_current) ?? null;
  } catch {
    return null;
  }
}

const noRankings = () => ({
  data: [],
  pagination: { page: 1, page_size: TABLE_PAGE_SIZE, total_items: 0, total_pages: 0 },
});

export const load: PageServerLoad = async ({ params, url }) => {
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

  const movement =
    asRankingMetric(url.searchParams.get('movement')) ??
    competition.scoring ??
    defaultRankingSort(ris);
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

  const [formula, classes, countries, rankings] = await Promise.all([
    currentFormula(),
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
        logger.error({ err }, 'Competition rankings load failed');
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
    formula,
  };
};
