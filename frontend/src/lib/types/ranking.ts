import type { AthleteInfo, CompetitionInfo } from './competition';
import type { Paginated } from './pagination';

export interface RankingEntry {
  rank: number;
  athlete: AthleteInfo;
  ris: number | null;
  ris_source: 'computed' | 'reported' | null;
  total: number;
  muscleup: number;
  pullup: number;
  dips: number;
  squat: number;
  competition: CompetitionInfo;
}

export type { PaginationMeta } from './pagination';

export type RankingsResponse = Paginated<RankingEntry>;

export interface RankingFilters {
  page: number;
  gender?: string | null;
  country?: string | null;
  movement?: string;
}
