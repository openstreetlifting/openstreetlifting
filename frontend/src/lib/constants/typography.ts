/**
 * The results tables are the point of the site, so everything around them is
 * sized to sit under them rather than compete: one step down from the usual
 * web defaults throughout.
 */
export const TEXT = {
  title: 'text-2xl font-light tracking-tight sm:text-3xl',
  heading: 'text-lg font-medium sm:text-xl',
  subheading: 'text-base font-medium',
  figure: 'text-2xl font-semibold',
  body: 'text-sm',
  meta: 'text-xs',
  micro: 'text-[0.65rem]',
} as const;

/** Buttons, pagination and counts sit with the data, not with prose. */
export const CONTROL = 'text-xs font-medium';

/** Shared chrome for every filter control. Each caller adds its own padding and width. */
export const FIELD =
  'rounded-lg border border-zinc-800 bg-zinc-900/50 text-xs text-zinc-300 transition-colors focus:border-zinc-700 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none';
