import { CANVAS } from '$lib/constants/theme';
import { PRIMARY_ROUTES } from '$lib/constants/routes';
import { SITE_DESCRIPTION, SITE_NAME } from '$lib/seo';
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

// Nothing here depends on the request, so the document is serialised once.
const BODY = JSON.stringify(manifest);

export const GET: RequestHandler = () => {
  return new Response(BODY, {
    headers: {
      'content-type': 'application/manifest+json',
      'cache-control': 'public, max-age=3600',
    },
  });
};
