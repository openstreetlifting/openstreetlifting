import { apiClient, apiData } from '../client';
import type { RankingFilters } from '#lib/types/ranking.js';
import type { Gender } from '#lib/types/enums.js';

export const rankingsService = {
  getGlobalRankings(query: RankingFilters) {
    return apiData(apiClient.GET('/api/v1/rankings', { params: { query } }));
  },

  getRankingClasses(gender?: Gender | null, competitionId?: string | null) {
    return apiData(
      apiClient.GET('/api/v1/rankings/classes', {
        params: { query: { gender, competition_id: competitionId } },
      })
    );
  },

  getRankingFederations() {
    return apiData(apiClient.GET('/api/v1/rankings/federations'));
  },

  getRankingYears() {
    return apiData(apiClient.GET('/api/v1/rankings/years'));
  },

  getRankingCountries(competitionId?: string | null) {
    return apiData(
      apiClient.GET('/api/v1/rankings/countries', {
        params: { query: { competition_id: competitionId } },
      })
    );
  },
};
