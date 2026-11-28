import { athletesService } from '#lib/server/api/index.js';
import { collect } from '#lib/server/api/collect.js';
import type { AthleteSummary } from '#lib/types/athlete.js';
import type { Competition } from '#lib/types/competition.js';

const PAGE_SIZE = 100;

export function allAthletes(): Promise<AthleteSummary[]> {
  return collect((page) => athletesService.getAll({ page, page_size: PAGE_SIZE }));
}

export function countryCodes(competitions: Competition[], rankedCountries: string[]): string[] {
  const codes = new Set(rankedCountries);
  for (const competition of competitions) {
    if (competition.country) codes.add(competition.country);
  }
  return [...codes].sort();
}
