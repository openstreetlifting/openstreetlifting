import { error } from '@sveltejs/kit';
import { rankingsService } from '$lib/server/api';
import { isHeld, publishedCompetitions, summarizeFederations } from '$lib/server/federations';
import type { PageServerLoad } from './$types';

const byDate = (a: { start_date: string | null }, b: { start_date: string | null }) =>
  (a.start_date ?? '').localeCompare(b.start_date ?? '');

export const load: PageServerLoad = async ({ params, url }) => {
  const page = Number(url.searchParams.get('page') ?? 1) || 1;

  let competitions;
  try {
    competitions = await publishedCompetitions();
  } catch (err) {
    console.error('Failed to load federation', { slug: params.slug, error: err });
    error(503, 'Federation details are unavailable right now');
  }

  const federation = summarizeFederations(competitions).find(
    (candidate) => candidate.slug === params.slug
  );
  if (!federation) {
    error(404, 'Federation not found');
  }

  const own = competitions.filter((competition) => competition.federation.name === federation.name);

  try {
    const rankings = await rankingsService.getGlobalRankings({
      page,
      movement: 'ris',
      federation: federation.name,
    });

    return {
      federation,
      results: own.filter(isHeld).sort((a, b) => byDate(b, a)),
      upcoming: own.filter((competition) => competition.status === 'upcoming').sort(byDate),
      rankings: rankings.data,
      pagination: rankings.pagination,
    };
  } catch (err) {
    console.error('Failed to load federation rankings', { slug: params.slug, error: err });
    error(503, 'Federation details are unavailable right now');
  }
};
