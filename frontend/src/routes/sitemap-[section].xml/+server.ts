import { logger } from '#lib/server/logger.js';
import { error } from '@sveltejs/kit';
import { archiveIndex, isSitemapSection } from '#lib/server/archive.js';
import { urlset, XML_HEADERS } from '#lib/server/sitemap.js';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ params }) => {
  if (!isSitemapSection(params.section)) {
    error(404, 'Sitemap not found');
  }

  let paths;
  try {
    paths = (await archiveIndex()).paths[params.section];
  } catch (err) {
    logger.error({ 'osl.sitemap.section': params.section, err }, 'Sitemap generation failed');
    error(503, 'Sitemap unavailable');
  }

  return new Response(urlset(paths), { headers: XML_HEADERS });
};
