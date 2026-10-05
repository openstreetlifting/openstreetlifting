import { RANKING_SORTS } from '#lib/constants/ranking.js';
import type { ProgressMetric } from './athlete-progress';

export function rankingMetric(value: string | null, fallback: ProgressMetric): ProgressMetric {
  return RANKING_SORTS.find((metric) => metric.value === value)?.value ?? fallback;
}

export function athleteFilters(params: Pick<URLSearchParams, 'get'>, formats: string[]) {
  const event = params.get('event');
  return {
    ranking: rankingMetric(params.get('ranking'), 'ris'),
    performance: rankingMetric(params.get('performance'), 'total'),
    event: event && formats.includes(event) ? event : (formats[0] ?? 'MPDS'),
  };
}
