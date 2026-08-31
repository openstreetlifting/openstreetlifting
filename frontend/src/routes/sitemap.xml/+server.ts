import { error } from '@sveltejs/kit';
import { athletesService, competitionsService } from '$lib/server/api';
import { absolute } from '$lib/seo';
import type { Paginated } from '$lib/types/pagination';
import type { RequestHandler } from './$types';

const PAGE_SIZE = 100;
const CACHE_TTL = 3_600_000;

const STATIC_PATHS = ['/', '/competitions', '/contact', '/privacy'];

let cache: { xml: string; expires: number } | null = null;

async function collect<T>(fetchPage: (page: number) => Promise<Paginated<T>>): Promise<T[]> {
  const first = await fetchPage(1);
  const remaining = Math.max(first.pagination.total_pages - 1, 0);
  const rest = await Promise.all(
    Array.from({ length: remaining }, (_, index) => fetchPage(index + 2))
  );

  return [first, ...rest].flatMap((response) => response.data);
}

function escapeXml(value: string): string {
  return value.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}

async function buildSitemap(): Promise<string> {
  const [competitions, athletes] = await Promise.all([
    collect((page) => competitionsService.getAll({ page, page_size: PAGE_SIZE })),
    collect((page) => athletesService.getAll({ page, page_size: PAGE_SIZE })),
  ]);

  const paths = [
    ...STATIC_PATHS,
    // A draft competition is a page nobody has finished checking, so it is left
    // for a crawler to find once it is published.
    ...competitions
      .filter((competition) => competition.status !== 'draft')
      .map((competition) => `/competitions/${competition.slug}`),
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
      console.error('Failed to build the sitemap:', err);
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
