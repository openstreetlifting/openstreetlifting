import { RANKING_FILTERS } from '#lib/constants/filters.js';
import { rankingsService } from '#lib/server/api/index.js';
import { keepSelected, readFilters } from '#lib/utils/filters.js';
import type { LayoutServerLoad } from './$types';

const NO_OPTIONS = { classes: [], years: [], countries: [], federations: [], series: [] };

// Reads only the dropdown filters, so search and pagination do not reload the options.
export const load: LayoutServerLoad = async ({ url }) => {
  const filters = readFilters(url.searchParams, RANKING_FILTERS);
  const facets = await rankingsService.getFacets(filters).catch(() => NO_OPTIONS);

  return keepSelected(facets, {
    classes: filters.category,
    years: filters.year,
    countries: filters.country,
    federations: filters.federation,
  });
};
