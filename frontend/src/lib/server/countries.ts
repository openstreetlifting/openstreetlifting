import { athletesService } from '$lib/server/api';
import { collect } from '$lib/server/api/collect';
import { isHeld } from '$lib/server/federations';
import type { AthleteSummary } from '$lib/types/athlete';
import type { Competition } from '$lib/types/competition';

const PAGE_SIZE = 100;

export interface CountrySummary {
  code: string;
  athletes: number;
  competitions: number;
  upcoming: number;
  federations: string[];
}

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

export function federationsBasedIn(competitions: Competition[], code: string): string[] {
  const names = new Set(
    competitions
      .filter((competition) => competition.federation.country === code)
      .map((competition) => competition.federation.name)
  );
  return [...names].sort((a, b) => a.localeCompare(b));
}

export function summarizeCountries(
  competitions: Competition[],
  athletes: AthleteSummary[],
  rankedCountries: string[]
): CountrySummary[] {
  return countryCodes(competitions, rankedCountries)
    .map((code) => {
      const held = competitions.filter((competition) => competition.country === code);

      return {
        code,
        athletes: athletes.filter((athlete) => athlete.country === code).length,
        competitions: held.filter(isHeld).length,
        upcoming: held.filter((competition) => competition.status === 'upcoming').length,
        federations: federationsBasedIn(competitions, code),
      };
    })
    .sort(
      (a, b) =>
        b.athletes - a.athletes || b.competitions - a.competitions || a.code.localeCompare(b.code)
    );
}
