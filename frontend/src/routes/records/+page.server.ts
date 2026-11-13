import { logger } from '#lib/server/logger.js';
import { error } from '@sveltejs/kit';
import { rankingsService, recordsService } from '#lib/server/api/index.js';
import type { RecordGender } from '#lib/types/records.js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url }) => {
  const params = url.searchParams;
  const gender: RecordGender = params.get('gender') === 'F' ? 'F' : 'M';
  const country = params.get('country')?.toUpperCase() || null;
  const federation = params.get('federation') || null;
  const year = Number(params.get('year')) || null;

  try {
    const [groups, countries, federations, years] = await Promise.all([
      recordsService.getRecords({ gender, country, federation, year }),
      rankingsService.getRankingCountries(),
      rankingsService.getRankingFederations(),
      recordsService.getYears(),
    ]);
    return {
      groups,
      countries,
      federations,
      years,
      gender,
      country,
      federation,
      year,
    };
  } catch (err) {
    logger.error({ err }, 'Records load failed');
    error(503, 'Records are unavailable right now');
  }
};
