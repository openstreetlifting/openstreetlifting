import type { AthleteCompetitionSummary } from '$lib/types/athlete';
import type { RANKING_SORTS } from '$lib/constants/ranking';

export type ProgressMetric = (typeof RANKING_SORTS)[number]['value'];
const MOVEMENTS = {
  muscleup: 'Muscle-up',
  pullup: 'Pull-up',
  dips: 'Dips',
  squat: 'Squat',
} as const;

export interface ProgressPoint {
  competition: AthleteCompetitionSummary;
  timestamp: number;
  value: number;
}

function numeric(value: string | null | undefined): number | null {
  if (value == null || value.trim() === '') return null;
  const number = Number(value);
  return Number.isFinite(number) ? number : null;
}

export function totalFormats(competitions: AthleteCompetitionSummary[]): string[] {
  return [
    ...new Set(
      competitions
        .filter((c) => c.status === 'competed' && c.event && numeric(c.total) !== null)
        .map((c) => c.event!)
    ),
  ].sort((a, b) => (a === 'MPDS' ? -1 : b === 'MPDS' ? 1 : a.localeCompare(b)));
}

export function progressPoints(
  competitions: AthleteCompetitionSummary[],
  metric: ProgressMetric,
  event = 'MPDS'
): ProgressPoint[] {
  const byCompetition = new Map<string, ProgressPoint>();
  for (const competition of competitions) {
    if (competition.status !== 'competed' || !competition.competition_date) continue;
    const timestamp = Date.parse(competition.competition_date);
    if (!Number.isFinite(timestamp)) continue;
    // Totals only describe the same performance when the contested movements match.
    if (metric === 'total' && competition.event !== event) continue;
    const value = numeric(
      metric === 'total'
        ? competition.total
        : metric === 'ris'
          ? competition.ris_score
          : competition.lifts.find((lift) => lift.movement_name === MOVEMENTS[metric])?.best_weight
    );
    if (value === null) continue;
    // A meet can list one athlete in multiple divisions; plot their best result once.
    const existing = byCompetition.get(competition.competition_id);
    if (!existing || value > existing.value)
      byCompetition.set(competition.competition_id, { competition, timestamp, value });
  }
  return [...byCompetition.values()].sort(
    (a, b) =>
      a.timestamp - b.timestamp ||
      a.competition.competition_id.localeCompare(b.competition.competition_id)
  );
}
