import { rankingsService } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url }) => {
	try {
		const movement = url.searchParams.get('movement') || 'total';
		const gender = url.searchParams.get('gender') || null;
		const country = url.searchParams.get('country') || null;

		const initialData = await rankingsService.getGlobalRankings({
			pagination: 1,
			movement,
			gender,
			country
		});

		return {
			initialRankings: initialData.data,
			pagination: initialData.pagination
		};
	} catch (error) {
		console.error('Error loading rankings:', error);
		return {
			error: error instanceof Error ? error.message : 'Failed to load rankings',
			initialRankings: [],
			pagination: {
				page: 1,
				page_size: 50,
				total_items: 0,
				total_pages: 0
			}
		};
	}
};
