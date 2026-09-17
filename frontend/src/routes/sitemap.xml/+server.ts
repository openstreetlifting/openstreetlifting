import { error } from '@sveltejs/kit';
import { rankingsService } from '$lib/server/api';
import { allAthletes, countryCodes } from '$lib/server/countries';
import { publishedCompetitions, summarizeFederations } from '$lib/server/federations';
import { STATIC_ROUTES } from '$lib/constants/routes';
import { absolute } from '$lib/seo';
import { countryPath, federationPath } from '$lib/utils';
import type { RequestHandler } from './$types';

const CACHE_TTL = 3_600_000;

const STATIC_PATHS = STATIC_ROUTES.map((route) => route.path);

let cache: { xml: string; expires: number } | null = null;

function escapeXml(value: string): string {
  return value.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}

async function buildSitemap(): Promise<string> {
  const [competitions, athletes, rankedCountries] = await Promise.all([
    publishedCompetitions(),
    allAthletes(),
    rankingsService.getRankingCountries(),
  ]);

  const paths = [
    ...STATIC_PATHS,
    ...summarizeFederations(competitions).map((federation) => federationPath(federation.name)),
    ...countryCodes(competitions, rankedCountries).map(countryPath),
    ...competitions.map((competition) => `/competitions/${competition.slug}`),
    ...athletes.map((athlete) => `/athletes/${athlete.slug}`),
  ];

  const urls = paths
    .map((path) => `  <url><loc>${escapeXml(absolute(path))}</loc></url>`)
    .join('\n');

  return `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urls}
</urlset>
`;
}

export const GET: RequestHandler = async () => {
  if (!cache || cache.expires < Date.now()) {
    try {
      cache = { xml: await buildSitemap(), expires: Date.now() + CACHE_TTL };
    } catch (err) {
      console.error('Failed to build sitemap', err);
      // A partial sitemap reads as a shrunken site, so nothing is served rather
      // than a list missing whatever the API could not answer for.
      error(503, 'Sitemap unavailable');
    }
  }

  return new Response(cache.xml, {
    headers: {
      'content-type': 'application/xml',
      'cache-control': 'public, max-age=3600',
    },
  });
};
