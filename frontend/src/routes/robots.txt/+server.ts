import { INDEXABLE, absolute } from '$lib/seo';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async () => {
  const body = INDEXABLE
    ? `User-agent: *\nAllow: /\n\nSitemap: ${absolute('/sitemap.xml')}\n`
    : `User-agent: *\nDisallow: /\n`;

  return new Response(body, {
    headers: {
      'content-type': 'text/plain',
      'cache-control': 'public, max-age=3600',
    },
  });
};
