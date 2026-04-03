export interface Federation {
	federation_id: string;
	name: string;
	abbreviation: string | null;
	country: string | null;
}

export interface Movement {
	movement_name: string;
	is_required: boolean;
	display_order: number | null;
}

export interface Competition {
	competition_id: string;
	name: string;
	created_at: string;
	slug: string;
	status: 'upcoming' | 'ongoing' | 'completed';
	venue: string | null;
	city: string | null;
	country: string | null;
	start_date: string | null;
	end_date: string | null;
	federation: Federation;
	movements: Movement[];
}

export interface CompetitionFilters {
	status?: 'upcoming' | 'ongoing' | 'completed';
	country?: string;
	search?: string;
}

export interface AthleteInfo {
	athlete_id: string;
	first_name: string;
	last_name: string;
	gender: string;
	country: string;
	slug: string;
	bodyweight?: number | null;
}

export interface Attempt {
	attempt_number: number;
	weight: string;
	is_successful: boolean;
	passing_judges: number | null;
	no_rep_reason: string | null;
}

export interface Lift {
	movement_name: string;
	best_weight: string;
	attempts: Attempt[];
}

export interface Participant {
	athlete: AthleteInfo;
	bodyweight: string | null;
	rank: number | null;
	total: string;
	ris_score: string | null;
	is_disqualified: boolean;
	disqualified_reason: string | null;
	lifts: Lift[];
}

export interface Category {
	category_id: string;
	name: string;
	gender: string;
	weight_class_min: string | null;
	weight_class_max: string | null;
}

export interface CategoryDetail {
	category: Category;
	participants: Participant[];
}

export interface CompetitionDetail {
	competition_id: string;
	name: string;
	slug: string;
	status: string;
	venue: string | null;
	city: string | null;
	country: string | null;
	start_date: string | null;
	end_date: string | null;
	federation: Federation;
	categories: CategoryDetail[];
}

export interface CompetitionInfo {
	competition_id: string;
	name: string;
	date: string | null;
}
