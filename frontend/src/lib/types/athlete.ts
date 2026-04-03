export interface AthleteDetail {
	athlete_id: string;
	first_name: string;
	last_name: string;
	slug: string;
	gender: string;
	nationality: string | null;
	country: string;
	profile_picture_url: string | null;
	created_at: string;
	competitions: AthleteCompetitionSummary[];
	personal_records: PersonalRecord[];
	total_competitions: number;
}

export interface AthleteCompetitionSummary {
	competition_id: string;
	competition_name: string;
	competition_slug: string;
	competition_date: string | null;
	category_name: string;
	rank: number | null;
	total: string;
	ris_score: string | null;
	is_disqualified: boolean;
}

export interface PersonalRecord {
	movement_name: string;
	max_weight: string;
	competition_name: string;
	competition_slug: string;
	date: string | null;
}
