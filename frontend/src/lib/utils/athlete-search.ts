import { RANKING_LIFTS } from '#lib/constants/ranking.js';
import type { AthleteSearchHit } from '#lib/types/athlete.js';

export type RankingLabel = 'Total' | 'RIS' | (typeof RANKING_LIFTS)[number]['label'];

export function rankingLabels(hit: Pick<AthleteSearchHit, 'rankings'>): RankingLabel[] {
  if (hit.rankings.includes('total')) return ['Total', 'RIS'];
  return RANKING_LIFTS.filter((lift) => hit.rankings.includes(lift.value)).map(
    (lift) => lift.label
  );
}

export function notInRanking(
  hits: AthleteSearchHit[],
  rankedSlugs: Iterable<string>
): AthleteSearchHit[] {
  const ranked = new Set(rankedSlugs);
  return hits.filter((hit) => !ranked.has(hit.slug));
}
