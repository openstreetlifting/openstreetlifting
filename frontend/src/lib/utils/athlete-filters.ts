import { RANKING_SORTS } from '#lib/constants/ranking.js';
import type { RankingMetric } from '#lib/types/enums.js';

export function athleteRanking(params: Pick<URLSearchParams, 'get'>): RankingMetric {
  return RANKING_SORTS.find((metric) => metric.value === params.get('ranking'))?.value ?? 'ris';
}
