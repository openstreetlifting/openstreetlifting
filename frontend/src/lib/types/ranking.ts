import type { AthleteInfo, CompetitionInfo } from './competition';
import type { Paginated } from './pagination';

export interface RankingEntry {
  rank: number;
  athlete: AthleteInfo;
  ris: number | null;
  ris_source: 'computed' | 'reported' | null;
  /** Absent outside the four-movement event, where a total does not compare. */
  total: number | null;
  /** Absent when the competition did not contest the movement. */
  muscleup: number | null;
  pullup: number | null;
  dips: number | null;
  squat: number | null;
  /** Which movements the competition contested, e.g. `MPDS` for all four. */
  event: string | null;
  competition: CompetitionInfo;
}

export type { PaginationMeta } from './pagination';

export type RankingsResponse = Paginated<RankingEntry>;

export interface RankingFilters {
  page: number;
  gender?: string | null;
  country?: string | null;
  movement?: string;
  /** Which event to rank totals within. Ignored for single-movement boards. */
  event?: string | null;
}
