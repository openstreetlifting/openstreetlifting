/**
 * Brightness carries one meaning across every table, how much a number counts,
 * and nothing else may use it. Hue is reserved for notability, a lift that set
 * a record, which is the one thing the archive can say that a column header
 * cannot.
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

export const NO_VALUE = '-';
export const NO_RESULT = '—';

/** Which column the table is sorted on. A tint, so it stays off the brightness scale. */
export const SORTED_COLUMN = 'rounded-t bg-zinc-800/60 text-zinc-200';
