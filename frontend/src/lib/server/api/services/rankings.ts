import { apiClient, apiData } from '../client';
import type { RankingFilters } from '#lib/types/ranking.js';

export const rankingsService = {
  getGlobalRankings(query: RankingFilters) {
    return apiData(apiClient.GET('/api/v1/rankings', { params: { query } }));
  },

  getFacets(query: RankingFilters) {
    return apiData(apiClient.GET('/api/v1/rankings/facets', { params: { query } }));
  },

  getRankingCountries() {
    return apiData(apiClient.GET('/api/v1/rankings/countries'));
  },
};
