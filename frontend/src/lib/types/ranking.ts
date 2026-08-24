import type { AthleteInfo, CompetitionInfo } from './competition';
import type { Paginated } from './pagination';

export interface RankingFederationInfo {
  name: string;
  abbreviation: string | null;
}

export interface RankingEntry {
  rank: number;
  athlete: AthleteInfo;
  category: string;
  division?: string | null;
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
  federation: RankingFederationInfo;
}

export type { PaginationMeta } from './pagination';

export type RankingsResponse = Paginated<RankingEntry>;

export interface RankingFilters {
  page: number;
  gender?: string | null;
  country?: string | null;
  /** Case insensitive substring of the athlete's full name. */
  q?: string | null;
  movement?: string;
  direction?: 'asc' | 'desc';
  /** Which event to rank totals within. Ignored for single-movement boards. */
  event?: string | null;
  /** Weight class suffix, e.g. `-73kg`, matched regardless of gender. */
  category?: string | null;
  year?: number | null;
  /** Narrows the ranking to one competition, e.g. for a per-meet leaderboard. */
  competition_id?: string | null;
}
