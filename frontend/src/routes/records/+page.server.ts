import { logger } from '#lib/server/logger.js';
import { error } from '@sveltejs/kit';
import { RECORD_FILTERS } from '#lib/constants/filters.js';
import { recordsService } from '#lib/server/api/index.js';
import type { RecordGender } from '#lib/types/records.js';
import { keepSelected, readFilters } from '#lib/utils/filters.js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url }) => {
  const gender: RecordGender = url.searchParams.get('gender') === 'F' ? 'F' : 'M';
  const filters = readFilters(url.searchParams, RECORD_FILTERS);

  try {
    const [groups, facets] = await Promise.all([
      recordsService.getRecords({ gender, ...filters }),
      recordsService.getFacets({ gender, ...filters }),
    ]);
    return {
      groups,
      facets: {
        ...facets,
        ...keepSelected(
          { countries: facets.countries, federations: facets.federations, years: facets.years },
          { countries: filters.country, federations: filters.federation, years: filters.year }
        ),
      },
      gender,
      filters,
    };
  } catch (err) {
    logger.error({ err }, 'Records load failed');
    error(503, 'Records are unavailable right now');
  }
};
