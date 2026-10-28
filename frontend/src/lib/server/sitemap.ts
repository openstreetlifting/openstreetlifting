import { LOCALES } from '#lib/i18n/locale.js';
import { absolute } from '#lib/seo/index.js';
import type { SitemapSection } from '#lib/server/archive.js';

function escapeXml(value: string): string {
  return value
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

export function sitemapPath(section: SitemapSection): string {
  return `/sitemap-${section}.xml`;
}

export function urlset(paths: string[]): string {
  const urls = paths
    .flatMap((path) =>
      LOCALES.map((locale) => {
        const alternates = LOCALES.map(
          (language) =>
            `<xhtml:link rel="alternate" hreflang="${language}" href="${escapeXml(absolute(path, language))}"/>`
        ).join('');
        return `  <url><loc>${escapeXml(absolute(path, locale))}</loc>${alternates}</url>`;
      })
    )
    .join('\n');

  return `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">
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
