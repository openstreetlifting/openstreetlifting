import { SITEMAP_SECTIONS } from '$lib/server/archive';
import { sitemapIndex, XML_HEADERS } from '$lib/server/sitemap';
import type { RequestHandler } from './$types';

const BODY = sitemapIndex(SITEMAP_SECTIONS);

export const GET: RequestHandler = () => new Response(BODY, { headers: XML_HEADERS });
