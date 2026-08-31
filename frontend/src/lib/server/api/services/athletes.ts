import { apiClient } from '../client';
import type { AthleteDetail, AthleteSummary } from '$lib/types/athlete';
import type { Paginated } from '$lib/types/pagination';

export const athletesService = {
  async getAll(params: { page: number; page_size: number }): Promise<Paginated<AthleteSummary>> {
    return apiClient.get<Paginated<AthleteSummary>>('/api/v1/athletes', { params });
  },

  async getBySlug(slug: string): Promise<AthleteDetail> {
    return apiClient.get<AthleteDetail>(`/api/v1/athletes/${slug}`, {
      params: { include: 'competitions,records,standing' },
    });
  },
};
