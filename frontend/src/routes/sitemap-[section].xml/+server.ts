import { error } from '@sveltejs/kit';
import { archiveIndex, isSitemapSection } from '$lib/server/archive';
import { urlset, XML_HEADERS } from '$lib/server/sitemap';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ params }) => {
  if (!isSitemapSection(params.section)) {
    error(404, 'Sitemap not found');
  }

  let paths;
  try {
    paths = (await archiveIndex()).paths[params.section];
  } catch (err) {
    console.error('Failed to build sitemap', { section: params.section, error: err });
    error(503, 'Sitemap unavailable');
  }

  return new Response(urlset(paths), { headers: XML_HEADERS });
};
