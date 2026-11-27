import { rankedGenderValues, rankingMetricValues } from '#lib/api/schema.generated.js';
import type { components } from '#lib/api/schema.generated.js';

export {
  athleteStatusValues as ATHLETE_STATUSES,
  competitionStatusValues as COMPETITION_STATUSES,
  directionValues as SORT_DIRECTIONS,
  genderValues as GENDERS,
  movementValues as MOVEMENTS,
  rankedGenderValues as RANKED_GENDERS,
  rankingMetricValues as RANKING_METRICS,
  risSourceValues as RIS_SOURCES,
  rankByValues as RANK_BY_VALUES,
} from '#lib/api/schema.generated.js';

type Schemas = components['schemas'];

export type AthleteStatus = Schemas['AthleteStatus'];
export type CompetitionStatus = Schemas['CompetitionStatus'];
export type Gender = Schemas['Gender'];
export type Movement = Schemas['Movement'];
export type RankedGender = Schemas['RankedGender'];
export type RankingMetric = Schemas['RankingMetric'];
export type RisSource = Schemas['RisSource'];
export type RankBy = Schemas['RankBy'];
export type SortDirection = Schemas['Direction'];

function narrow<T extends string>(
  vocabulary: readonly T[],
  raw: string | null | undefined
): T | null {
  return vocabulary.find((value) => value === raw) ?? null;
}

export function asRankedGender(raw: string | null | undefined): RankedGender | null {
  return narrow(rankedGenderValues, raw);
}

export function asRankingMetric(raw: string | null | undefined): RankingMetric | null {
  return narrow(rankingMetricValues, raw);
}
