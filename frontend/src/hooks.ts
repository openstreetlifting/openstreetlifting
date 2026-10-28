import type { Reroute } from '@sveltejs/kit/hooks';
import { isPagePath, pathLocale, stripLocale } from '#lib/i18n/paths.js';

// The public URL keeps its locale; existing route modules handle the page itself.
export const reroute: Reroute = ({ url }) => {
  const path = stripLocale(url.pathname);
  return pathLocale(url.pathname) && (isPagePath(url.pathname) || path === '/manifest.webmanifest')
    ? path
    : undefined;
};
