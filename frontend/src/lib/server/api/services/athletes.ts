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

    return {
      ...athlete,
      competitions: athlete.competitions ?? [],
      personal_records: athlete.personal_records ?? [],
      total_competitions: athlete.total_competitions ?? 0,
    };
  },
};
