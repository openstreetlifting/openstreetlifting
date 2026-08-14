import { apiClient } from '../client';
import type { RankingsResponse, RankingFilters } from '$lib/types/ranking';

export const rankingsService = {
  async getGlobalRankings(filters: RankingFilters): Promise<RankingsResponse> {
    const params: Record<string, string | number> = {
      page: filters.page,
    };

    if (filters.gender) {
      params.gender = filters.gender;
    }

    if (filters.country) {
      params.country = filters.country;
    }

    if (filters.movement) {
      params.movement = filters.movement;
    }

    if (filters.direction) {
      params.direction = filters.direction;
    }

    if (filters.category) {
      params.category = filters.category;
    }

    if (filters.year) {
      params.year = filters.year;
    }

    if (filters.competition_id) {
      params.competition_id = filters.competition_id;
    }

    return apiClient.get<RankingsResponse>('/api/v1/rankings', { params });
  },

  async getRankingClasses(
    gender?: string | null,
    competitionId?: string | null
  ): Promise<string[]> {
    const params: Record<string, string> = {};
    if (gender) params.gender = gender;
    if (competitionId) params.competition_id = competitionId;
    return apiClient.get<string[]>('/api/v1/rankings/classes', { params });
  },

  async getRankingYears(): Promise<number[]> {
    return apiClient.get<number[]>('/api/v1/rankings/years');
  },
};
