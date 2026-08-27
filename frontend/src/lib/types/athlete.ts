import type { RisSource } from './competition';

export interface AthleteDetail {
  athlete_id: string;
  first_name: string;
  last_name: string;
  slug: string;
  gender: string;
  country: string;
  profile_picture_url: string | null;
  instagram_handle: string | null;
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
