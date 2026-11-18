import { apiClient, apiData, commaSeparatedArrays } from '../client';
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
        params: { query: { ...filters, include: ['federation', 'movements'] } },
        querySerializer: commaSeparatedArrays,
      })
    );
    return { ...response, data: response.data.map(withDetails) };
  },

  async getById(slug: string) {
    const competition = withDetails(
      await apiData(
        apiClient.GET('/api/v1/competitions/{slug}', {
          params: {
            path: { slug },
            query: { include: ['federation', 'results', 'movements', 'editions'] },
          },
          querySerializer: commaSeparatedArrays,
        })
      )
    );
    if (!competition.categories) throw new Error('Requested competition results are missing');
    if (!competition.editions) throw new Error('Requested competition editions are missing');
    return {
      ...competition,
      categories: competition.categories,
      editions: competition.editions,
    };
  },

  getFacets(query: CompetitionFilters) {
    return apiData(apiClient.GET('/api/v1/competitions/facets', { params: { query } }));
  },
};
