import { I18n } from '#lib/i18n/index.js';
import { CANVAS } from '#lib/constants/theme.js';
import { PRIMARY_ROUTES } from '#lib/constants/routes.js';
import { SITE_DESCRIPTION, SITE_NAME } from '#lib/seo/index.js';
import type { RequestHandler } from './$types';

function icon(path: string, size: number, purpose: 'any' | 'maskable') {
  return { src: path, sizes: `${size}x${size}`, type: 'image/png', purpose };
}

const manifest = {
  id: '/',
  name: SITE_NAME,
  short_name: 'OSL',
  description: SITE_DESCRIPTION,
  start_url: '/',
  scope: '/',
  display: 'standalone',
  background_color: CANVAS,
  theme_color: CANVAS,
  lang: 'en',
  categories: ['sports'],
  icons: [
    icon('/icons/icon-192.png', 192, 'any'),
    icon('/icons/icon-512.png', 512, 'any'),
    icon('/icons/icon-maskable-192.png', 192, 'maskable'),
    icon('/icons/icon-maskable-512.png', 512, 'maskable'),
  ],
  shortcuts: PRIMARY_ROUTES.map(({ name, path }) => ({ name, url: path })),
};

export const GET: RequestHandler = ({ locals }) => {
  const i18n = new I18n(() => locals.locale);
  return new Response(
    JSON.stringify({
      ...manifest,
      lang: i18n.locale,
      start_url: i18n.href('/'),
      description: i18n.t(SITE_DESCRIPTION),
      shortcuts: PRIMARY_ROUTES.map(({ name, path }) => ({
        name: name === 'RIS' ? name : i18n.t(name),
        url: i18n.href(path),
      })),
    }),
    {
      headers: {
        'content-type': 'application/manifest+json',
        'cache-control': 'public, max-age=3600',
      },
    }
  );
};
