// The site's hand-written pages. Every consumer that has to enumerate them —
// the sitemap, the manifest shortcuts — reads this list, so a new page reaches
// all of them at once.
export const PRIMARY_ROUTES = [
  { name: 'Rankings', path: '/' },
  { name: 'Competitions', path: '/competitions' },
  { name: 'RIS', path: '/ris' },
] as const;

export const SECONDARY_ROUTES = [
  { name: 'Contact', path: '/contact' },
  { name: 'Privacy', path: '/privacy' },
] as const;

export const STATIC_ROUTES = [...PRIMARY_ROUTES, ...SECONDARY_ROUTES];
