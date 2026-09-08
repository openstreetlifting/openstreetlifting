import { describe, expect, it } from 'vitest';
import type { AthleteCompetitionSummary } from '$lib/types/athlete';
import { progressPoints, totalFormats } from './athlete-progress';

function result(overrides: Partial<AthleteCompetitionSummary> = {}): AthleteCompetitionSummary {
  return {
    competition_id: 'meet-1',
    competition_name: 'Worlds',
    competition_slug: 'worlds',
    competition_date: '2025-10-04',
    category_name: 'Women -63 kg',
    rank: 1,
    total: '300',
    ris_score: '100',
    ris_source: 'computed',
    status: 'competed',
    event: 'MPDS',
    lifts: [{ movement_name: 'Muscle-up', best_weight: '0', attempts: [] }],
    ...overrides,
  };
}

describe('athlete progress', () => {
  it('separates total event formats and excludes unknown formats', () => {
    const competitions = [
      result(),
      result({ competition_id: 'meet-2', event: 'PD', total: '200' }),
      result({ competition_id: 'meet-3', event: null, total: '400' }),
    ];
    expect(totalFormats(competitions)).toEqual(['MPDS', 'PD']);
    expect(progressPoints(competitions, 'total').map((p) => p.value)).toEqual([300]);
    expect(progressPoints(competitions, 'total', 'PD').map((p) => p.value)).toEqual([200]);
  });

  it('keeps real zeroes while rejecting absent and invalid values', () => {
    const competitions = [
      result(),
      result({ competition_id: 'meet-2', lifts: [] }),
      result({
        competition_id: 'meet-3',
        lifts: [{ movement_name: 'Muscle-up', best_weight: null, attempts: [] }],
      }),
      result({ competition_id: 'meet-4', total: 'NaN' }),
    ];
    expect(progressPoints(competitions.slice(0, 3), 'muscleup').map((p) => p.value)).toEqual([0]);
    expect(progressPoints(competitions.slice(3), 'total')).toEqual([]);
  });

  it('excludes disqualified, no-show and undated results', () => {
    const competitions = [
      result({ status: 'disqualified' }),
      result({ status: 'no_show' }),
      result({ competition_date: null }),
      result({ competition_date: 'invalid' }),
    ];
    expect(progressPoints(competitions, 'ris')).toEqual([]);
  });

  it('takes the best per meet across divisions and sorts without mutating history', () => {
    const competitions = [
      result({ division: 'Open', total: '300' }),
      result({ division: 'Junior', total: '310' }),
      result({ competition_id: 'older', competition_date: '2024-01-01', total: '250' }),
    ];
    const points = progressPoints(competitions, 'total');
    expect(points.map((p) => p.value)).toEqual([250, 310]);
    expect(points[1].competition.division).toBe('Junior');
    expect(competitions[0].total).toBe('300');
  });
});
