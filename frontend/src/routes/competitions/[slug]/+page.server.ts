import { competitionsService } from '$lib/api';
import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params }) => {
	try {
		const competition = await competitionsService.getById(params.slug);

		return {
			competition
		};
	} catch (err) {
		console.error('Failed to fetch competition:', err);
		throw error(404, 'Competition not found');
	}
};
