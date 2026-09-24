export const FORMAT_MOVEMENTS = [
  { code: 'M', label: 'Muscle-up' },
  { code: 'P', label: 'Pull-up' },
  { code: 'D', label: 'Dips' },
  { code: 'S', label: 'Squat' },
] as const;

/** A selection is a set: its click order must not change the requested format. */
export function normalizeEvent(raw: string | null | undefined): string {
  if (!raw || !/^[MPDS]+$/.test(raw)) return '';
  return FORMAT_MOVEMENTS.filter(({ code }) => raw.includes(code))
    .map(({ code }) => code)
    .join('');
}

export function formatMovements(event: string): string[] {
  return FORMAT_MOVEMENTS.filter(({ code }) => event.includes(code)).map(({ label }) => label);
}
