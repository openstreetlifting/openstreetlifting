import type { components } from '#lib/api/schema.generated.js';

type Schemas = components['schemas'];

export type AthleteSummary = Schemas['AthleteResponse'];
export type AthleteDetail = AthleteSummary &
  Required<
    Pick<
      AthleteSummary,
      'competitions' | 'personal_records' | 'total_competitions' | 'standing' | 'strength_profile'
    >
  >;
export type AthleteStanding = Schemas['AthleteStanding'];
export type MetricStanding = Schemas['MetricStanding'];
export type StrengthProfile = Schemas['StrengthProfile'];
export type AthleteCompetitionSummary = Schemas['AthleteCompetitionSummary'];
