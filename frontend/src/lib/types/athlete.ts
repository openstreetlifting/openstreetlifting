import type { RisSource } from './competition';

export interface AthleteSummary {
  athlete_id: string;
  first_name: string;
  last_name: string;
  slug: string;
  gender: string;
  country: string;
  profile_picture_url: string | null;
  created_at: string;
}

export interface StandingPlace {
  place: number;
  field: number;
}

export interface CountryStanding extends StandingPlace {
  code: string;
}

/** The global board, which is ordered on RIS. */
export interface RisStanding {
  /** The score the place was taken on. */
  score?: string | null;
  global: StandingPlace;
  country: CountryStanding;
}

/** The same two places on total, among the lifters in the athlete's own class. */
export interface WeightClassStanding {
  class: string;
  /** The total the place was taken on. */
  total?: string | null;
  global: StandingPlace;
  country: CountryStanding;
}

export interface AthleteStanding {
  ris?: RisStanding | null;
  weight_class?: WeightClassStanding | null;
}

export interface AthleteDetail {
  athlete_id: string;
  first_name: string;
  last_name: string;
  native_name?: string | null;
  slug: string;
  gender: string;
  country: string;
  profile_picture_url: string | null;
  instagram_handle: string | null;
  created_at: string;
  competitions: AthleteCompetitionSummary[];
  personal_records: PersonalRecord[];
  total_competitions: number;
  /** Absent for an athlete the RIS board does not rank. */
  standing?: AthleteStanding | null;
}

export interface AthleteCompetitionSummary {
  competition_id: string;
  competition_name: string;
  competition_slug: string;
  competition_date: string | null;
  category_name: string;
  division?: string | null;
  rank: number | null;
  total: string | null;
  ris_score: string | null;
  ris_source: RisSource | null;
  status: 'competed' | 'disqualified' | 'no_show';
}

export interface PersonalRecord {
  movement_name: string;
  max_weight: string;
  competition_name: string;
  competition_slug: string;
  date: string | null;
}
