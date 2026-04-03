import { athletesService } from '$lib/api';
import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params }) => {
	try {
		const athlete = await athletesService.getBySlug(params.slug);

		return {
			athlete
		};
	} catch (err) {
		console.error('Failed to fetch athlete:', err);
		throw error(404, 'Athlete not found');
	}
};
