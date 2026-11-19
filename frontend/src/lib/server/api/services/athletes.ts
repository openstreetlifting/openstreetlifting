import { apiClient, apiData, commaSeparatedArrays } from '../client';
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
          query: { include: ['competitions', 'records', 'standing', 'lift_balance', 'neighbors'] },
        },
        querySerializer: commaSeparatedArrays,
      })
    );

    if (
      athlete.competitions === undefined ||
      athlete.personal_records === undefined ||
      athlete.best_total === undefined ||
      athlete.best_ris === undefined ||
      athlete.total_competitions === undefined ||
      athlete.standing === undefined ||
      athlete.lift_balance === undefined ||
      athlete.neighbors === undefined
    ) {
      throw new Error('Requested athlete sections are missing');
    }

    return {
      ...athlete,
      competitions: athlete.competitions,
      personal_records: athlete.personal_records,
      best_total: athlete.best_total,
      best_ris: athlete.best_ris,
      total_competitions: athlete.total_competitions,
      standing: athlete.standing,
      lift_balance: athlete.lift_balance,
      neighbors: athlete.neighbors,
    };
  },
};
