import { RANKING_SORTS } from '$lib/constants/ranking';
import type { ProgressMetric } from './athlete-progress';

export function rankingMetric(value: string | null, fallback: ProgressMetric): ProgressMetric {
  return RANKING_SORTS.find((metric) => metric.value === value)?.value ?? fallback;
}

export function athleteFilters(params: URLSearchParams, formats: string[]) {
  const event = params.get('event');
  return {
    ranking: rankingMetric(params.get('ranking'), 'ris'),
    performance: rankingMetric(params.get('performance'), 'total'),
    event: event && formats.includes(event) ? event : (formats[0] ?? 'MPDS'),
  };
}
