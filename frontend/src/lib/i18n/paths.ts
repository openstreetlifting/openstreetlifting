import { DEFAULT_LOCALE, isLocale, type Locale } from './locale.js';

export function pathLocale(pathname: string): Locale | undefined {
  const first = pathname.split('/')[1];
  return isLocale(first) ? first : undefined;
}

export function stripLocale(pathname: string): string {
  return pathLocale(pathname) ? pathname.replace(/^\/[a-z]{2}(?=\/|$)/, '') || '/' : pathname;
}

/** Prefix internal links once, preserving their query string and fragment. */
export function localeHref(href: string, locale: Locale): string {
  if (!href.startsWith('/') || href.startsWith('//')) return href;
  const boundary = href.search(/[?#]/);
  const path = boundary === -1 ? href : href.slice(0, boundary);
  const suffix = boundary === -1 ? '' : href.slice(boundary);
  const unprefixed = stripLocale(path);
  return `/${locale}${unprefixed === '/' ? '' : unprefixed}${suffix}`;
}

const PAGE_ROOTS = new Set([
  '',
  'rankings',
  'competitions',
  'athletes',
  'countries',
  'federations',
  'ris',
  'contact',
  'privacy',
]);
export function isPagePath(pathname: string): boolean {
  return PAGE_ROOTS.has(stripLocale(pathname).split('/')[1] ?? '');
}

export function legacyRedirect(pathname: string, search: string): string | undefined {
  return !pathLocale(pathname) && isPagePath(pathname)
    ? localeHref(pathname + search, DEFAULT_LOCALE)
    : undefined;
}
