import { apiClient } from '../client';
import type { Competition, CompetitionDetail, CompetitionFilters } from '$lib/types/competition';

export const competitionsService = {
	async getAll(filters?: CompetitionFilters): Promise<Competition[]> {
		return apiClient.get<Competition[]>('/api/competitions/detailed', {
			params: filters as Record<string, string | number | boolean>
		});
	},

	async getById(slug: string): Promise<CompetitionDetail> {
		return apiClient.get<CompetitionDetail>(`/api/competitions/${slug}/detailed`);
	}
};
