import type { AthleteCompetitionSummary } from '#lib/types/athlete.js';
import type { Movement } from '#lib/types/enums.js';
import { RANKING_SORTS } from '#lib/constants/ranking.js';

export type ProgressMetric = (typeof RANKING_SORTS)[number]['value'];
const MOVEMENTS = {
  muscleup: 'Muscle-up',
  pullup: 'Pull-up',
  dips: 'Dips',
  squat: 'Squat',
} as const;

const MOVEMENT_METRICS = {
  'Muscle-up': 'muscleup',
  'Pull-up': 'pullup',
  Dips: 'dips',
  Squat: 'squat',
} as const satisfies Record<Movement, ProgressMetric>;

export function movementMetric(movement: Movement): ProgressMetric {
  return MOVEMENT_METRICS[movement];
}

export function competitionFocusQuery(metric: ProgressMetric, athleteSlug?: string): string {
  const query = new URLSearchParams({ movement: metric });
  if (athleteSlug) query.set('athlete', athleteSlug);
  return query.toString();
}

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

export function progressPoints(
  competitions: AthleteCompetitionSummary[],
  metric: ProgressMetric,
  event = 'MPDS'
): ProgressPoint[] {
  const byCompetition = new Map<string, ProgressPoint>();
  for (const competition of competitions) {
    if (competition.status !== 'competed') continue;
    const timestamp = Date.parse(competition.competition_date);
    if (!Number.isFinite(timestamp)) continue;
    // Keep totals, scores and individual lifts within the selected event format.
    if (competition.event !== event) continue;
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

export function chartableMetrics(
  competitions: AthleteCompetitionSummary[],
  event = 'MPDS'
): ProgressMetric[] {
  return RANKING_SORTS.map((sort) => sort.value).filter(
    (metric) => progressPoints(competitions, metric, event).length >= 2
  );
}
