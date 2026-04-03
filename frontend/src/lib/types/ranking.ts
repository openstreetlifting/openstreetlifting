import type { AthleteInfo, CompetitionInfo } from './competition';

export interface RankingEntry {
	rank: number;
	athlete: AthleteInfo;
	ris: number;
	total: number;
	muscleup: number;
	pullup: number;
	dips: number;
	squat: number;
	competition: CompetitionInfo;
}

export interface PaginationMeta {
	page: number;
	page_size: number;
	total_items: number;
	total_pages: number;
}

export interface RankingsResponse {
	data: RankingEntry[];
	pagination: PaginationMeta;
}

export interface RankingFilters {
	pagination: number;
	gender?: string | null;
	country?: string | null;
	movement?: string;
}
