import { apiClient, apiData } from '../client';
import type { paths } from '#lib/api/schema.generated.js';

type AthleteQuery = paths['/api/v1/athletes']['get']['parameters']['query'];

export const athletesService = {
  getAll(query: AthleteQuery) {
    return apiData(apiClient.GET('/api/v1/athletes', { params: { query } }));
  },

  async getBySlug(slug: string) {
    const athlete = await apiData(
      apiClient.GET('/api/v1/athletes/{slug}', {
        params: {
          path: { slug },
          query: { include: 'competitions,records,standing,strength_profile' },
        },
      })
    );

    if (
      athlete.competitions === undefined ||
      athlete.personal_records === undefined ||
      athlete.total_competitions === undefined ||
      athlete.standing === undefined ||
      athlete.strength_profile === undefined
    ) {
      throw new Error('Requested athlete sections are missing');
    }

    return {
      ...athlete,
      competitions: athlete.competitions,
      personal_records: athlete.personal_records,
      total_competitions: athlete.total_competitions,
      standing: athlete.standing,
      strength_profile: athlete.strength_profile,
    };
  },
};
