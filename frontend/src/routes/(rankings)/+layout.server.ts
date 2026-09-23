import { rankingsService } from '$lib/server/api';
import { asRankedGender } from '$lib/types/enums';
import type { LayoutServerLoad } from './$types';

// Keep filter options independent of search and pagination reloads.
export const load: LayoutServerLoad = async ({ url }) => {
  const gender = asRankedGender(url.searchParams.get('gender'));
  const [classes, years, countries, federations] = await Promise.all([
    rankingsService.getRankingClasses(gender).catch(() => []),
    rankingsService.getRankingYears().catch(() => []),
    rankingsService.getRankingCountries().catch(() => []),
    rankingsService.getRankingFederations().catch(() => []),
  ]);

  return { classes, years, countries, federations };
};
