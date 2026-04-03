import { apiClient } from '../client';
import type { RankingsResponse, RankingFilters } from '$lib/types/ranking';

export const rankingsService = {
	async getGlobalRankings(filters: RankingFilters): Promise<RankingsResponse> {
		const params: Record<string, string | number> = {
			pagination: filters.pagination
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

		return apiClient.get<RankingsResponse>('/api/rankings/global', { params });
	}
};
