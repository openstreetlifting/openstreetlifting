import { rankingsService } from '$lib/server/api';
import { allAthletes, countryCodes } from '$lib/server/countries';
import { publishedCompetitions, summarizeFederations } from '$lib/server/federations';
import { STATIC_ROUTES } from '$lib/constants/routes';
import { countryPath, federationPath } from '$lib/utils';

const CACHE_TTL = 3_600_000;

export const SITEMAP_SECTIONS = ['pages', 'competitions', 'athletes'] as const;
export type SitemapSection = (typeof SITEMAP_SECTIONS)[number];

export interface ArchiveIndex {
  paths: Record<SitemapSection, string[]>;
  counts: { competitions: number; athletes: number; federations: number; countries: number };
}

let cache: { value: ArchiveIndex; expires: number } | null = null;

async function build(): Promise<ArchiveIndex> {
  const [competitions, athletes, rankedCountries] = await Promise.all([
    publishedCompetitions(),
    allAthletes(),
    rankingsService.getRankingCountries(),
  ]);
  const federations = summarizeFederations(competitions);
  const countries = countryCodes(competitions, rankedCountries);

  return {
    paths: {
      pages: [
        ...STATIC_ROUTES.map((route) => route.path),
        ...federations.map((federation) => federationPath(federation.name)),
        ...countries.map(countryPath),
      ],
      competitions: competitions.map((competition) => `/competitions/${competition.slug}`),
      athletes: athletes.map((athlete) => `/athletes/${athlete.slug}`),
    },
    counts: {
      competitions: federations.reduce((sum, federation) => sum + federation.competitions, 0),
      athletes: athletes.length,
      federations: federations.length,
      countries: countries.length,
    },
  };
}

export async function archiveIndex(): Promise<ArchiveIndex> {
  if (!cache || cache.expires < Date.now()) {
    cache = { value: await build(), expires: Date.now() + CACHE_TTL };
  }
  return cache.value;
}

export function isSitemapSection(value: string): value is SitemapSection {
  return (SITEMAP_SECTIONS as readonly string[]).includes(value);
}
