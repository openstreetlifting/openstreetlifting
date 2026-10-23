import { apiClient, apiData } from '../client';
import type { components } from '#lib/api/schema.generated.js';
import type { CompetitionFilters } from '#lib/types/competition.js';

function withDetails(competition: components['schemas']['CompetitionResponse']) {
  if (!competition.federation || !competition.movements) {
    throw new Error('Requested competition sections are missing');
  }
  return {
    ...competition,
    federation: competition.federation,
    movements: competition.movements,
  };
}

export const competitionsService = {
  async getAll(filters?: CompetitionFilters) {
    const response = await apiData(
      apiClient.GET('/api/v1/competitions', {
        params: { query: { ...filters, include: 'federation,movements' } },
      })
    );
    return { ...response, data: response.data.map(withDetails) };
  },

  async getById(slug: string) {
    const competition = withDetails(
      await apiData(
        apiClient.GET('/api/v1/competitions/{slug}', {
          params: { path: { slug }, query: { include: 'federation,results,movements' } },
        })
      )
    );
    if (!competition.categories) throw new Error('Requested competition results are missing');
    return { ...competition, categories: competition.categories };
  },

  async getFacets() {
    const [federations, years, countries, formats] = await Promise.all([
      apiData(apiClient.GET('/api/v1/competitions/federations')),
      apiData(apiClient.GET('/api/v1/competitions/years')),
      apiData(apiClient.GET('/api/v1/competitions/countries')),
      apiData(apiClient.GET('/api/v1/competitions/formats')),
    ]);

    return { federations, years, countries, formats };
  },
};
