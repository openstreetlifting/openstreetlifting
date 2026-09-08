export interface RisConstants {
  a: number;
  k: number;
  b: number;
  v: number;
  q: number;
}

export interface RisFormula {
  year: number;
  is_current: boolean;
  credit: string;
  constants: {
    men: RisConstants;
    women: RisConstants;
  };
}

/** One recorded performance, with its athlete and competition. */
export interface RisPerformance {
  participant_id: string;
  athlete_name: string;
  athlete_slug: string;
  competition_name: string;
  competition_slug: string;
  competition_date: string;
  bodyweight: number;
  total: number;
}

export interface RisDistribution {
  men: RisPerformance[];
  women: RisPerformance[];
}
