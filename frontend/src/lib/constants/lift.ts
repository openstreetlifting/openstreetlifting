import type { Movement } from '#lib/types/enums.js';

export const LIFTS = [
  {
    key: 'muscleup',
    code: 'M',
    movement: 'Muscle-up',
    label: 'Muscle Up',
    tone: 'var(--color-lift-muscleup)',
  },
  {
    key: 'pullup',
    code: 'P',
    movement: 'Pull-up',
    label: 'Pull Up',
    tone: 'var(--color-lift-pullup)',
  },
  { key: 'dips', code: 'D', movement: 'Dips', label: 'Dips', tone: 'var(--color-lift-dips)' },
  { key: 'squat', code: 'S', movement: 'Squat', label: 'Squat', tone: 'var(--color-lift-squat)' },
] as const satisfies readonly {
  key: string;
  code: string;
  movement: Movement;
  label: string;
  tone: string;
}[];

export type Lift = (typeof LIFTS)[number];

export function liftTone(name: string): string | undefined {
  return LIFTS.find((lift) => lift.movement === name || lift.key === name || lift.code === name)
    ?.tone;
}
