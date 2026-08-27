/**
 * Brightness carries one meaning across every table, how much a number counts,
 * and nothing else may use it. Hue is reserved for notability, a lift that set
 * a record, which is the one thing the archive can say that a column header
 * cannot.
 *
 * Amber is the single exception, and it always means a status rather than a
 * value: a lifter who did not place, and a score that was reported rather than
 * computed. See STATUS_FLAG and REPORTED_MARK below.
 */
export const CELL = {
  /** Who the row is, rather than a value it carries, so it sits outside the scale. */
  identity: 'text-white',
  counted: 'font-medium text-white',
  data: 'text-zinc-400',
  discounted: 'text-zinc-600 line-through decoration-zinc-600',
  /** Contested and never made, which is a result. Unlike `absent`, which is no data. */
  nothing: 'text-zinc-600',
  absent: 'text-zinc-700',
} as const;

/** A status rather than a value, the one hue spent outside notability. */
export const STATUS_FLAG = 'text-amber-500/90';

/**
 * A score the federation reported rather than one worked out from a bodyweight.
 * Provenance is a status, so it takes the amber above rather than a colour of
 * its own, and it takes it on a mark beside the number rather than on the
 * number. A reported score is not worth less than a computed one, so putting it
 * on the brightness scale would say something untrue about it.
 */
export const REPORTED_MARK = `${STATUS_FLAG} ml-px align-super text-[0.75em] leading-none font-semibold`;
/** R for reported. A letter rather than a footnote glyph, which reads as a cross. */
export const REPORTED_GLYPH = 'R';
export const REPORTED_TITLE = 'RIS reported by federation, missing bodyweight we need to recompute';

export const NO_VALUE = '-';
export const NO_RESULT = '—';

/** Which column the table is sorted on. A tint, so it stays off the brightness scale. */
export const SORTED_COLUMN = 'rounded-t bg-zinc-800/60 text-zinc-200';

/**
 * Below sm the data is worth more than the margin, so the table runs to the
 * screen edge. Anything stacked with it takes the same treatment, or the page
 * ends up with two different left edges.
 */
export const EDGE_TO_EDGE = '-mx-4 sm:mx-0';
