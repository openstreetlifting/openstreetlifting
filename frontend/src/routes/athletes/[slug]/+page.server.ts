import { athletesService } from '$lib/server/api';
import { ApiError } from '$lib/server/api/client';
import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params }) => {
  try {
    const athlete = await athletesService.getBySlug(params.slug);

    return { athlete };
  } catch (err) {
    if (err instanceof ApiError && err.status === 404) {
      error(404, 'Athlete not found');
    }
    console.error('Failed to fetch athlete:', err);
    error(503, 'Athlete details are unavailable right now');
  }
};
