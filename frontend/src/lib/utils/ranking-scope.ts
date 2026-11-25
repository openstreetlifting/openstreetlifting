import type { RankingMetric } from '#lib/types/enums.js';

export const EVENT_SCOPED_METRICS: readonly RankingMetric[] = ['ris', 'total', 'classic_total'];

export function isEventScoped(metric: RankingMetric): boolean {
  return EVENT_SCOPED_METRICS.includes(metric);
}
