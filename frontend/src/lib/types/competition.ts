export interface Federation {
  federation_id: string;
  name: string;
  abbreviation: string | null;
  country: string | null;
}

export interface Movement {
  movement_name: string;
  display_order: number | null;
}

export interface Competition {
  competition_id: string;
  name: string;
  created_at: string;
  slug: string;
  status: 'upcoming' | 'ongoing' | 'completed';
  city: string | null;
  region: string | null;
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
  instagram_handle?: string | null;
}

export interface Attempt {
  attempt_number: number;
  weight: string;
  is_successful: boolean;
}

export interface Lift {
  movement_name: string;
  best_weight: string | null;
  attempts: Attempt[];
}

export interface Participant {
  athlete: AthleteInfo;
  bodyweight: string | null;
  rank: number | null;
  total: string | null;
  ris_score: string | null;
  is_disqualified: boolean;
  disqualified_reason: string | null;
  lifts: Lift[];
}

export interface Category {
  name: string;
  division?: string | null;
  gender: string;
  weight_class: string;
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
  city: string | null;
  region: string | null;
  country: string | null;
  start_date: string | null;
  end_date: string | null;
  federation: Federation;
  categories: CategoryDetail[];
}

export interface CompetitionInfo {
  competition_id: string;
  name: string;
  slug: string;
  date: string | null;
}
