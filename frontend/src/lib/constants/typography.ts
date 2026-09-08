export const TEXT = {
  title: 'text-2xl font-semibold tracking-tight sm:text-3xl',
  heading: 'text-lg font-medium sm:text-xl',
  subheading: 'text-base font-medium',
  figure: 'font-mono text-2xl font-semibold',
  body: 'text-sm',
  meta: 'text-xs',
  micro: 'text-[0.65rem]',
} as const;

export const CONTROL = 'text-xs font-medium';

export const NAV_LINK = 'text-sm text-secondary transition-colors hover:text-ink';

export const FIELD =
  'rounded-lg border border-stroke bg-surface text-xs text-secondary transition-colors focus:border-focus focus:ring-2 focus:ring-focus focus:ring-offset-2 focus:ring-offset-canvas focus:outline-none';

export const CHART = {
  tick: 'fill-muted stroke-none font-mono text-[11px]',
  label: 'fill-secondary stroke-none text-[11px]',
} as const;
