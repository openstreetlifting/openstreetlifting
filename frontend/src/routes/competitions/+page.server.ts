import { competitionsService } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	try {
		const competitions = await competitionsService.getAll();

		return {
			competitions
		};
	} catch (error) {
		console.error('Failed to fetch competitions:', error);
		return {
			competitions: [],
			error: 'Failed to load competitions'
		};
	}
};
