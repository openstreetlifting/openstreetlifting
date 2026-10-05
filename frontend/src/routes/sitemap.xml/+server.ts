import { SITEMAP_SECTIONS } from '#lib/server/archive.js';
import { sitemapIndex, XML_HEADERS } from '#lib/server/sitemap.js';
import type { RequestHandler } from './$types';

const BODY = sitemapIndex(SITEMAP_SECTIONS);

export const GET: RequestHandler = () => new Response(BODY, { headers: XML_HEADERS });
