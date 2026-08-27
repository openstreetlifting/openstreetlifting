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

export type CompetitionStatus = 'draft' | 'upcoming' | 'live' | 'completed' | 'cancelled';

export interface Competition {
  competition_id: string;
  name: string;
  created_at: string;
  slug: string;
  status: CompetitionStatus;
  city: string | null;
  region: string | null;
  country: string | null;
  start_date: string | null;
  end_date: string | null;
  federation: Federation;
  movements: Movement[];
  lifter_count?: number;
}

export interface CompetitionFilters {
  status?: CompetitionStatus;
  federation?: string;
  country?: string;
  year?: number;
  q?: string;
  direction?: 'asc' | 'desc';
  page?: number;
  page_size?: number;
}

export interface CompetitionFacets {
  federations: string[];
  years: number[];
  countries: string[];
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

/**
 * Where a score came from. `computed` was worked out from the athlete's
 * bodyweight and total. `reported` was stated by the source, which gave no
 * bodyweight, so it cannot be restated on the formula everything else uses.
 */
export type RisSource = 'computed' | 'reported';

export interface Participant {
  athlete: AthleteInfo;
  bodyweight: string | null;
  rank: number | null;
  total: string | null;
  ris_score: string | null;
  ris_source: RisSource | null;
  status: 'competed' | 'disqualified' | 'no_show';
  status_reason: string | null;
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
  status: CompetitionStatus;
  city: string | null;
  region: string | null;
  country: string | null;
  start_date: string | null;
  end_date: string | null;
  federation: Federation;
  movements: Movement[];
  categories: CategoryDetail[];
}

export interface CompetitionInfo {
  competition_id: string;
  name: string;
  slug: string;
  date: string | null;
}
