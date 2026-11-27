import { CHART } from '#lib/constants/typography.js';

export const TICK_LABEL = { fontSize: 11, class: CHART.tick } as const;

export const AXIS = {
  tickMarks: true,
  tickLength: 6,
  rule: true,
  stroke: 'var(--color-stroke-strong)',
  tickLabelProps: TICK_LABEL,
} as const;

export const BAND_AXIS = {
  ...AXIS,
  tickMarks: false,
  rule: false,
  tickLabelProps: { ...TICK_LABEL, class: CHART.label },
} as const;

export const PLOT_PADDING = { top: 24, right: 12, bottom: 44, left: 48 } as const;

export function yTitle(label: string) {
  return {
    label,
    labelPlacement: 'start',
    labelProps: { class: CHART.label, x: 0, dy: 4 },
  } as const;
}

export function xTitle(label: string) {
  return {
    label,
    labelPlacement: 'end',
    labelProps: { class: CHART.label, dx: -PLOT_PADDING.right, dy: -6 },
  } as const;
}

export function withUnit(label: string, unit: string): string {
  return `${label} (${unit})`;
}

export function expand([low, high]: [number, number], ratio = 0.04): [number, number] {
  const margin = (high - low) * ratio;
  return [low - margin, high + margin];
}
