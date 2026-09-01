import { env } from '$env/dynamic/public';

export const SITE_NAME = 'OpenStreetlifting';

export const SITE_DESCRIPTION =
  'OpenStreetlifting is an open, collaborative project building a permanent and traceable archive of all Streetlifting data, freely accessible to everyone.';

// Canonical, Open Graph and sitemap URLs have to be absolute, and a request
// served behind the ingress cannot tell what the public origin is.
export const SITE_URL = (env.PUBLIC_SITE_URL ?? 'https://openstreetlifting.org').replace(
  /\/+$/,
  ''
);

// A named pre-release environment serves the same pages as production, so it
// stays out of the index rather than competing with it. An unset environment is
// local development, which no crawler reaches.
const environment = env.PUBLIC_ENVIRONMENT ?? '';
export const INDEXABLE = environment === '' || environment === 'production';

export const OG_IMAGE = {
  url: `${SITE_URL}/og-image.png`,
  width: 1200,
  height: 630,
} as const;

export function absolute(path: string): string {
  return `${SITE_URL}${path}`;
}

export function pageTitle(title: string): string {
  return title.includes(SITE_NAME) ? title : `${title} - ${SITE_NAME}`;
}

// Filters and sorts multiply a table into thousands of URLs holding the same
// rows in a different order, so only paging stays crawlable, since it is how a
// robot walks past the first page.
const CRAWLABLE_PARAMS = new Set(['page']);

export interface ListingSeo {
  canonical: string;
  noindex: boolean;
}

export function listingSeo(url: URL): ListingSeo {
  const filtered = [...url.searchParams.keys()].some((key) => !CRAWLABLE_PARAMS.has(key));
  const page = Number(url.searchParams.get('page') ?? 1) || 1;
  const query = !filtered && page > 1 ? `?page=${page}` : '';

  return { canonical: absolute(`${url.pathname}${query}`), noindex: filtered };
}
