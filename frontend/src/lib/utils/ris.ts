import type { RisConstants } from '$lib/types/ris';

/** The published reference total at this bodyweight: a RIS score of 100. */
export function benchmarkTotal(constants: RisConstants, bodyweight: number): number {
  const { a, k, b, v, q } = constants;
  return a + (k - a) / (1 + q * Math.exp(-b * (bodyweight - v)));
}

/** Keep full precision for calculations; round only when displaying a score. */
export function calculateRis(constants: RisConstants, bodyweight: number, total: number): number {
  return (total * 100) / benchmarkTotal(constants, bodyweight);
}

export type BodyweightTarget =
  { kind: 'bodyweight'; value: number } | { kind: 'unreachable' } | { kind: 'already-met' };

/** Solve the published curve at a fixed total, including its asymptotic limits. */
export function bodyweightForRis(
  constants: RisConstants,
  total: number,
  target: number
): BodyweightTarget {
  const { a, k, b, v, q } = constants;
  const reference = (total * 100) / target;
  if (reference <= a) return { kind: 'unreachable' };
  if (reference >= k) return { kind: 'already-met' };
  return { kind: 'bodyweight', value: v - Math.log((k - reference) / (q * (reference - a))) / b };
}
