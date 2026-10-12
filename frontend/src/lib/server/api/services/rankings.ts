import { apiClient } from '../client';
import type { RankingsResponse, RankingFilters } from '#lib/types/ranking.js';
import type { Gender } from '#lib/types/enums.js';

export const rankingsService = {
  async getGlobalRankings(filters: RankingFilters): Promise<RankingsResponse> {
    return apiClient.get<RankingsResponse>('/api/v1/rankings', { params: { ...filters } });
  },

  async getRankingClasses(
    gender?: Gender | null,
    competitionId?: string | null
  ): Promise<string[]> {
    const params: Record<string, string> = {};
    if (gender) params.gender = gender;
    if (competitionId) params.competition_id = competitionId;
    return apiClient.get<string[]>('/api/v1/rankings/classes', { params });
  },

  async getRankingFederations(): Promise<string[]> {
    return apiClient.get<string[]>('/api/v1/rankings/federations');
  },

  async getRankingYears(): Promise<number[]> {
    return apiClient.get<number[]>('/api/v1/rankings/years');
  },

  async getRankingCountries(competitionId?: string | null): Promise<string[]> {
    const params: Record<string, string> = {};
    if (competitionId) params.competition_id = competitionId;
    return apiClient.get<string[]>('/api/v1/rankings/countries', { params });
  },
};
