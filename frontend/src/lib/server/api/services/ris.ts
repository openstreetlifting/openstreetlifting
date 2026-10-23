import { apiClient, apiData } from '../client';
import type { components } from '#lib/api/schema.generated.js';

export const risService = {
  getFormulas(year?: number) {
    return apiData(apiClient.GET('/api/v1/ris/formulas', { params: { query: { year } } }));
  },

  async getDistribution() {
    const raw = await apiData(apiClient.GET('/api/v1/ris/distribution'));
    const points = (rows: components['schemas']['RisPerformanceResponse'][]) =>
      rows.map((row) => ({
        ...row,
        bodyweight: Number(row.bodyweight),
        total: Number(row.total),
      }));

    return { men: points(raw.men), women: points(raw.women) };
  },
};
