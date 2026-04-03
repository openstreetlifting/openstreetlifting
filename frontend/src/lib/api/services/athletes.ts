import { apiClient } from '../client';
import type { AthleteDetail } from '$lib/types/athlete';

export const athletesService = {
	async getBySlug(slug: string): Promise<AthleteDetail> {
		return apiClient.get<AthleteDetail>(`/api/athletes/${slug}/detailed`);
	}
};
