export const TEXT = {
  fieldLabel: 'font-mono text-micro text-muted',
  panelTitle: 'font-mono text-body font-medium leading-5 text-balance text-ink',
  caption: 'text-meta leading-5 text-muted',
  cardLabel: 'font-mono text-meta font-medium leading-5 text-balance text-secondary sm:text-body',
  cardFigure: 'font-mono text-title font-semibold leading-8 text-ink sm:text-display sm:leading-9',
  title: 'font-mono text-title font-semibold tracking-tight text-balance sm:text-display',
  heading: 'font-mono text-subtitle font-medium text-balance sm:text-heading',
  subheading: 'font-mono text-lead font-medium text-balance',
  figure: 'font-mono text-title font-semibold',
  body: 'text-body',
  meta: 'text-meta',
  micro: 'text-micro',
} as const;

export const CONTROL = 'font-mono text-meta font-medium leading-4';

export const NAV_LINK =
  'inline-flex min-h-11 items-center whitespace-nowrap font-mono text-meta text-secondary transition-colors hover:text-ink focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-focus';

export const FIELD =
  'min-h-11 rounded-control border border-stroke bg-canvas font-mono text-meta leading-4 text-ink transition-colors hover:border-stroke-strong focus:border-focus focus:ring-2 focus:ring-focus focus:ring-offset-2 focus:ring-offset-canvas focus:outline-none sm:min-h-8';

export const CHART = {
  tooltip:
    'max-w-64 rounded-panel border border-stroke-strong bg-surface p-3 text-ink shadow-lg shadow-overlay/30',
  tick: 'fill-muted stroke-none font-mono text-micro',
  label: 'fill-secondary stroke-none font-mono text-micro',
} as const;

export const OVERLAY_SURFACE =
  'rounded-panel border border-stroke-strong bg-surface text-ink shadow-lg shadow-overlay/30';
