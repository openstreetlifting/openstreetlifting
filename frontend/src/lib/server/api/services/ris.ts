import { apiClient } from '../client';
import type { RisDistribution, RisFormula, RisPerformance } from '$lib/types/ris';

export const risService = {
  /** Every published edition, newest last, as the backend holds them. */
  async getFormulas(): Promise<RisFormula[]> {
    return apiClient.get<RisFormula[]>('/api/v1/ris/formulas');
  },

  /**
   * Decimals arrive as strings, so they are parsed once here rather than at
   * every point in the chart.
   */
  async getDistribution(): Promise<RisDistribution> {
    type RawPerformance = Omit<RisPerformance, 'bodyweight' | 'total'> & {
      bodyweight: string;
      total: string;
    };
    const raw = await apiClient.get<{ men: RawPerformance[]; women: RawPerformance[] }>(
      '/api/v1/ris/distribution'
    );

    const points = (rows: RawPerformance[]): RisPerformance[] =>
      rows.map((row) => ({
        ...row,
        bodyweight: Number(row.bodyweight),
        total: Number(row.total),
      }));

    return { men: points(raw.men), women: points(raw.women) };
  },
};
