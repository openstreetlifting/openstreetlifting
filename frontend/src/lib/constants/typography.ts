export const TEXT = {
  fieldLabel: 'font-mono text-[0.6875rem] leading-4 text-muted',
  panelTitle: 'font-mono text-sm font-medium leading-5 text-balance text-ink',
  caption: 'text-xs leading-5 text-muted',
  cardLabel: 'font-mono text-xs font-medium leading-5 text-balance text-secondary sm:text-sm',
  cardFigure: 'font-mono text-2xl font-semibold leading-8 text-ink sm:text-3xl sm:leading-9',
  title: 'font-mono text-2xl font-semibold tracking-tight text-balance sm:text-3xl',
  heading: 'font-mono text-lg font-medium text-balance sm:text-xl',
  subheading: 'font-mono text-base font-medium text-balance',
  figure: 'font-mono text-2xl font-semibold',
  body: 'text-sm',
  meta: 'text-xs',
  micro: 'text-[0.65rem]',
} as const;

export const CONTROL = 'font-mono text-xs font-medium leading-4';

export const NAV_LINK =
  'inline-flex min-h-11 items-center whitespace-nowrap font-mono text-xs text-secondary transition-colors hover:text-ink focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-focus';

export const FIELD =
  'min-h-11 rounded-control border border-stroke bg-canvas font-mono text-xs leading-4 text-ink transition-colors hover:border-stroke-strong focus:border-focus focus:ring-2 focus:ring-focus focus:ring-offset-2 focus:ring-offset-canvas focus:outline-none sm:min-h-8';

export const CHART = {
  tooltip:
    'max-w-64 rounded-panel border border-stroke-strong bg-surface p-3 text-ink shadow-lg shadow-overlay/30',
  tick: 'fill-muted stroke-none font-mono text-[11px]',
  label: 'fill-secondary stroke-none font-mono text-[11px]',
} as const;

export const OVERLAY_SURFACE =
  'rounded-panel border border-stroke-strong bg-surface text-ink shadow-lg shadow-overlay/30';
