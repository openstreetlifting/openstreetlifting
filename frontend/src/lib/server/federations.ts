import { competitionsService } from '$lib/server/api';
import { collect } from '$lib/server/api/collect';
import type { Competition } from '$lib/types/competition';
import { slugify } from '$lib/utils';

const PAGE_SIZE = 100;

export interface FederationSummary {
  slug: string;
  name: string;
  abbreviation: string | null;
  country: string | null;
  competitions: number;
  upcoming: number;
  entries: number;
  countries: number;
  firstYear: number | null;
  lastYear: number | null;
}

export async function publishedCompetitions(): Promise<Competition[]> {
  const competitions = await collect((page) =>
    competitionsService.getAll({ page, page_size: PAGE_SIZE })
  );
  return competitions.filter((competition) => competition.status !== 'draft');
}

export function isHeld(competition: Competition): boolean {
  return competition.status === 'completed' || competition.status === 'live';
}

function year(date: string | null): number | null {
  return date ? Number(date.slice(0, 4)) : null;
}

export function summarizeFederations(competitions: Competition[]): FederationSummary[] {
  const groups = new Map<string, Competition[]>();
  for (const competition of competitions) {
    const group = groups.get(competition.federation.name) ?? [];
    group.push(competition);
    groups.set(competition.federation.name, group);
  }

  return [...groups.values()]
    .map((group) => {
      const { name, abbreviation, country } = group[0].federation;
      const held = group.filter(isHeld);
      const years = held
        .map((competition) => year(competition.start_date))
        .filter((y) => y !== null);

      return {
        slug: slugify(name),
        name,
        abbreviation,
        country,
        competitions: held.length,
        upcoming: group.filter((competition) => competition.status === 'upcoming').length,
        entries: held.reduce((sum, competition) => sum + (competition.lifter_count ?? 0), 0),
        countries: new Set(held.map((competition) => competition.country).filter(Boolean)).size,
        firstYear: years.length ? Math.min(...years) : null,
        lastYear: years.length ? Math.max(...years) : null,
      };
    })
    .sort((a, b) => b.competitions - a.competitions || a.name.localeCompare(b.name));
}
