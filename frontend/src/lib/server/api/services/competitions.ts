import { apiClient } from '../client';
import type { Competition, CompetitionDetail, CompetitionFilters } from '$lib/types/competition';
import type { Paginated } from '$lib/types/pagination';

export const competitionsService = {
  async getAll(filters?: CompetitionFilters): Promise<Paginated<Competition>> {
    return apiClient.get<Paginated<Competition>>('/api/v1/competitions', {
      params: {
        include: 'federation,movements',
        ...filters,
      },
    });
  },

  async getById(slug: string): Promise<CompetitionDetail> {
    return apiClient.get<CompetitionDetail>(`/api/v1/competitions/${slug}`, {
      params: { include: 'federation,results' },
    });
  },
};
