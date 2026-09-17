import { absolute } from '$lib/seo';
import type { SitemapSection } from '$lib/server/archive';

function escapeXml(value: string): string {
  return value.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}

export function sitemapPath(section: SitemapSection): string {
  return `/sitemap-${section}.xml`;
}

export function urlset(paths: string[]): string {
  const urls = paths
    .map((path) => `  <url><loc>${escapeXml(absolute(path))}</loc></url>`)
    .join('\n');

  return `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urls}
</urlset>
`;
}

export function sitemapIndex(sections: readonly SitemapSection[]): string {
  const entries = sections
    .map(
      (section) => `  <sitemap><loc>${escapeXml(absolute(sitemapPath(section)))}</loc></sitemap>`
    )
    .join('\n');

  return `<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${entries}
</sitemapindex>
`;
}

export const XML_HEADERS = {
  'content-type': 'application/xml',
  'cache-control': 'public, max-age=3600',
};
