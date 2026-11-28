import type { AthleteCompetitionSummary } from '#lib/types/athlete.js';
import type { Movement, RankingMetric } from '#lib/types/enums.js';
import { LIFTS, type Lift } from '#lib/constants/lift.js';

const MOVEMENT_METRICS = {
  'Muscle-up': 'muscleup',
  'Pull-up': 'pullup',
  Dips: 'dips',
  Squat: 'squat',
} as const satisfies Record<Movement, RankingMetric>;

export function movementMetric(movement: Movement): RankingMetric {
  return MOVEMENT_METRICS[movement];
}

export function competitionFocusQuery(metric: RankingMetric, athleteSlug?: string): string {
  const query = new URLSearchParams({ movement: metric });
  if (athleteSlug) query.set('athlete', athleteSlug);
  return query.toString();
}

export interface ProgressPoint {
  competition: AthleteCompetitionSummary;
  timestamp: number;
  value: number;
}

export interface LiftProgress {
  lift: Lift;
  points: ProgressPoint[];
}

function liftPoints(competitions: AthleteCompetitionSummary[], movement: Movement) {
  const byCompetition = new Map<string, ProgressPoint>();
  for (const competition of competitions) {
    if (competition.status !== 'competed') continue;
    const timestamp = Date.parse(competition.competition_date);
    const raw = competition.lifts.find((lift) => lift.movement_name === movement)?.best_weight;
    const value = raw == null || raw.trim() === '' ? NaN : Number(raw);
    if (!Number.isFinite(timestamp) || !Number.isFinite(value)) continue;
    const existing = byCompetition.get(competition.competition_id);
    if (!existing || value > existing.value) {
      byCompetition.set(competition.competition_id, { competition, timestamp, value });
    }
  }
  return [...byCompetition.values()].sort(
    (a, b) =>
      a.timestamp - b.timestamp ||
      a.competition.competition_id.localeCompare(b.competition.competition_id)
  );
}

export function liftProgress(competitions: AthleteCompetitionSummary[]): LiftProgress[] {
  return LIFTS.map((lift) => ({ lift, points: liftPoints(competitions, lift.movement) })).filter(
    ({ points }) => points.length > 0
  );
}

export function showsProgress(progress: LiftProgress[]): boolean {
  return progress.filter(({ points }) => points.length >= 2).length >= 2;
}
