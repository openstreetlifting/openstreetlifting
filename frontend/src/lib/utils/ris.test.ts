import { describe, expect, it } from 'vitest';
import { benchmarkTotal, bodyweightForRis, calculateRis } from './ris';

describe('RIS reference performance', () => {
  const constants = { a: 100, k: 300, b: 0.1, v: 80, q: 1 };

  it('places the reference at 100 and preserves proportional scores', () => {
    expect(benchmarkTotal(constants, 80)).toBe(200);
    expect(calculateRis(constants, 80, 200)).toBe(100);
    expect(calculateRis(constants, 80, 220)).toBe(110);
    expect(calculateRis(constants, 80, 180)).toBe(90);
  });

  it('reproduces the 2026 reference example without rounding intermediate values', () => {
    const men2026 = {
      a: 335.5625,
      k: 556.1103380806655,
      b: 0.10289374204365953,
      v: 76.74125992622565,
      q: 0.4973075488457353,
    };
    expect(calculateRis(men2026, 80, 500).toFixed(2)).toBe('100.35');
    const reference = benchmarkTotal(men2026, 80);
    expect(calculateRis(men2026, 80, reference)).toBeCloseTo(100, 12);
  });

  it('solves a target at fixed total using the same reference curve', () => {
    expect(bodyweightForRis(constants, 200, 100)).toEqual({ kind: 'bodyweight', value: 80 });
    const target = calculateRis(constants, 72.5, 210);
    const result = bodyweightForRis(constants, 210, target);
    expect(result.kind).toBe('bodyweight');
    if (result.kind === 'bodyweight') expect(result.value).toBeCloseTo(72.5, 10);
  });

  it('does not invent a bodyweight beyond the curve’s asymptotes', () => {
    expect(bodyweightForRis(constants, 0, 110)).toEqual({ kind: 'unreachable' });
    expect(bodyweightForRis(constants, 100, 100)).toEqual({ kind: 'unreachable' });
    expect(bodyweightForRis(constants, 300, 100)).toEqual({ kind: 'already-met' });
  });
});
