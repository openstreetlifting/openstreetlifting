import type { components } from '#lib/api/schema.generated.js';

type Schemas = components['schemas'];

export type AthleteSummary = Schemas['AthleteResponse'];
export type AthleteDetail = Omit<
  AthleteSummary,
  'competitions' | 'personal_records' | 'total_competitions'
> & {
  competitions: NonNullable<AthleteSummary['competitions']>;
  personal_records: NonNullable<AthleteSummary['personal_records']>;
  total_competitions: NonNullable<AthleteSummary['total_competitions']>;
};
export type AthleteStanding = Schemas['AthleteStanding'];
export type MetricStanding = Schemas['MetricStanding'];
export type StrengthProfile = Schemas['StrengthProfile'];
export type AthleteCompetitionSummary = Schemas['AthleteCompetitionSummary'];
