import type { components } from '#lib/api/schema.generated.js';

type Schemas = components['schemas'];

export type AthleteSummary = Schemas['AthleteResponse'];
export type AthleteDetail = AthleteSummary &
  Required<
    Pick<
      AthleteSummary,
      | 'competitions'
      | 'personal_records'
      | 'total_competitions'
      | 'standing'
      | 'lift_balance'
      | 'neighbors'
    >
  >;
export type AthleteStanding = Schemas['AthleteStanding'];
export type MetricStanding = Schemas['MetricStanding'];
export type LiftBalance = Schemas['LiftBalance'];
export type AthleteCompetitionSummary = Schemas['AthleteCompetitionSummary'];

export type AthleteNeighborhood = Schemas['AthleteNeighborhood'];
export type AthleteNeighbor = Schemas['AthleteNeighbor'];
