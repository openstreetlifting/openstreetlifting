export interface paths {
  '/api/v1/athletes': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['list_athletes'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/athletes/{slug}': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['get_athlete'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/competitions': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['list_competitions'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/competitions/countries': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['list_competition_countries'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/competitions/federations': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['list_competition_federations'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/competitions/formats': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['list_competition_formats'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/competitions/years': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['list_competition_years'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/competitions/{slug}': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['get_competition'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/rankings': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['get_global_ranking'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/rankings/classes': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['list_ranking_classes'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/rankings/countries': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['list_ranking_countries'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/rankings/federations': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['list_ranking_federations'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/rankings/years': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['list_ranking_years'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/ris/calculations': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    post: operations['calculate_ris'];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/ris/distribution': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['get_ris_distribution'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/ris/formulas': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['list_ris_formulas'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/ris/formulas/current': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['get_current_formula'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
}
export type webhooks = Record<string, never>;
export interface components {
  schemas: {
    AthleteCompetitionSummary: {
      category_name: string;
      competition_date: string | null;
      competition_id: string;
      competition_name: string;
      competition_slug: string;
      division: string | null;
      event: string | null;
      lifts: components['schemas']['AthleteLift'][];
      rank: number | null;
      ris_score: string | null;
      ris_source: components['schemas']['RisSource'] | null;
      status: components['schemas']['AthleteStatus'];
      total: string | null;
    };
    AthleteLift: {
      attempts: components['schemas']['AttemptInfo'][];
      best_weight: string | null;
      movement_name: components['schemas']['Movement'];
    };
    AthleteResponse: {
      athlete_id: string;
      competitions?: components['schemas']['AthleteCompetitionSummary'][];
      country: string | null;
      created_at: string;
      first_name: string;
      gender: components['schemas']['Gender'];
      instagram_handle: string | null;
      last_name: string;
      native_name: string | null;
      personal_records?: components['schemas']['PersonalRecord'][];
      profile_picture_url: string | null;
      slug: string;
      standing?: components['schemas']['AthleteStanding'] | null;
      strength_profile?: components['schemas']['StrengthProfile'] | null;
      total_competitions?: number;
    };
    AthleteStanding: {
      dips: components['schemas']['MetricStanding'] | null;
      muscleup: components['schemas']['MetricStanding'] | null;
      pullup: components['schemas']['MetricStanding'] | null;
      ris: components['schemas']['MetricStanding'] | null;
      squat: components['schemas']['MetricStanding'] | null;
      total: components['schemas']['MetricStanding'] | null;
    };
    AthleteStatus: 'competed' | 'disqualified' | 'no_show';
    AttemptInfo: {
      attempt_number: number;
      is_successful: boolean;
      weight: string;
    };
    CategoryDetail: {
      category: components['schemas']['CategoryInfo'];
      participants: components['schemas']['ParticipantDetail'][];
    };
    CategoryInfo: {
      division: string | null;
      gender: components['schemas']['Gender'];
      name: string;
      weight_class: string;
    };
    CompetitionAthlete: {
      athlete_id: string;
      country: string | null;
      first_name: string;
      gender: components['schemas']['Gender'];
      last_name: string;
      slug: string;
    };
    CompetitionEdition: {
      name: string;
      slug: string;
      start_date: string;
    };
    CompetitionFederation: {
      abbreviation: string | null;
      country: string | null;
      federation_id: string;
      name: string;
    };
    CompetitionInfo: {
      competition_id: string;
      date: string | null;
      name: string;
      slug: string;
    };
    CompetitionResponse: {
      categories?: components['schemas']['CategoryDetail'][];
      city: string | null;
      competition_id: string;
      country: string | null;
      created_at: string;
      editions?: components['schemas']['CompetitionEdition'][];
      end_date: string | null;
      event_code: string | null;
      federation?: components['schemas']['CompetitionFederation'];
      federation_id: string;
      lifter_count: number | null;
      movements?: components['schemas']['MovementInfo'][];
      name: string;
      region: string | null;
      scoring: components['schemas']['Scoring'] | null;
      slug: string;
      start_date: string | null;
      status: components['schemas']['CompetitionStatus'];
      venue: string | null;
    };
    CompetitionStatus: 'draft' | 'upcoming' | 'live' | 'completed' | 'cancelled';
    ComputeRisRequest: {
      bodyweight: string;
      formula_year?: number | null;
      gender: components['schemas']['Gender'];
      total: string;
    };
    ComputeRisResponse: {
      formula_year: number;
      ris_score: string;
    };
    CountryStanding: {
      code: string;
      field: number;
      place: number;
    };
    Direction: 'desc' | 'asc';
    Gender: 'M' | 'F' | 'MX';
    GenderConstants: {
      a: number;
      b: number;
      k: number;
      q: number;
      v: number;
    };
    GlobalRankingEntry: {
      athlete: components['schemas']['RankingAthlete'];
      category: string;
      competition: components['schemas']['CompetitionInfo'];
      dips: number | null;
      division: string | null;
      event: string | null;
      federation: components['schemas']['RankingFederation'];
      muscleup: number | null;
      participant_id: string;
      pullup: number | null;
      rank: number;
      ris: number | null;
      ris_source: components['schemas']['RisSource'] | null;
      squat: number | null;
      total: number | null;
    };
    Include: string;
    LiftDetail: {
      attempts: components['schemas']['AttemptInfo'][];
      best_weight: string | null;
      movement_name: components['schemas']['Movement'];
    };
    MetricStanding: {
      class: string | null;
      country: components['schemas']['CountryStanding'] | null;
      global: components['schemas']['StandingPlace'];
      value: string;
    };
    Movement: 'Muscle-up' | 'Pull-up' | 'Dips' | 'Squat';
    MovementInfo: {
      code: string;
      display_order: number | null;
      movement_name: components['schemas']['Movement'];
    };
    PaginatedResponse_AthleteResponse: {
      data: {
        athlete_id: string;
        competitions?: components['schemas']['AthleteCompetitionSummary'][];
        country: string | null;
        created_at: string;
        first_name: string;
        gender: components['schemas']['Gender'];
        instagram_handle: string | null;
        last_name: string;
        native_name: string | null;
        personal_records?: components['schemas']['PersonalRecord'][];
        profile_picture_url: string | null;
        slug: string;
        standing?: components['schemas']['AthleteStanding'] | null;
        strength_profile?: components['schemas']['StrengthProfile'] | null;
        total_competitions?: number;
      }[];
      pagination: components['schemas']['PaginationMeta'];
    };
    PaginatedResponse_CompetitionResponse: {
      data: {
        categories?: components['schemas']['CategoryDetail'][];
        city: string | null;
        competition_id: string;
        country: string | null;
        created_at: string;
        editions?: components['schemas']['CompetitionEdition'][];
        end_date: string | null;
        event_code: string | null;
        federation?: components['schemas']['CompetitionFederation'];
        federation_id: string;
        lifter_count: number | null;
        movements?: components['schemas']['MovementInfo'][];
        name: string;
        region: string | null;
        scoring: components['schemas']['Scoring'] | null;
        slug: string;
        start_date: string | null;
        status: components['schemas']['CompetitionStatus'];
        venue: string | null;
      }[];
      pagination: components['schemas']['PaginationMeta'];
    };
    PaginatedResponse_GlobalRankingEntry: {
      data: {
        athlete: components['schemas']['RankingAthlete'];
        category: string;
        competition: components['schemas']['CompetitionInfo'];
        dips: number | null;
        division: string | null;
        event: string | null;
        federation: components['schemas']['RankingFederation'];
        muscleup: number | null;
        participant_id: string;
        pullup: number | null;
        rank: number;
        ris: number | null;
        ris_source: components['schemas']['RisSource'] | null;
        squat: number | null;
        total: number | null;
      }[];
      pagination: components['schemas']['PaginationMeta'];
    };
    PaginationMeta: {
      page: number;
      page_size: number;
      total_items: number;
      total_pages: number;
    };
    PaginationParams: {
      page?: number;
      page_size?: number;
    };
    ParticipantDetail: {
      athlete: components['schemas']['CompetitionAthlete'];
      bodyweight: string | null;
      lifts: components['schemas']['LiftDetail'][];
      participant_id: string;
      rank: number | null;
      ris_score: string | null;
      ris_source: components['schemas']['RisSource'] | null;
      status: components['schemas']['AthleteStatus'];
      status_reason: string | null;
      total: string | null;
    };
    PersonalRecord: {
      competition_name: string;
      competition_slug: string;
      date: string | null;
      max_weight: string;
      movement_name: components['schemas']['Movement'];
    };
    RankedGender: 'M' | 'F' | 'MX';
    RankingAthlete: {
      athlete_id: string;
      bodyweight: number | null;
      country: string | null;
      first_name: string;
      gender: components['schemas']['Gender'];
      instagram_handle: string | null;
      last_name: string;
      slug: string;
    };
    RankingFederation: {
      abbreviation: string | null;
      name: string;
    };
    RankingMetric: 'muscleup' | 'pullup' | 'dips' | 'squat' | 'total' | 'ris';
    RisConstants: {
      men: components['schemas']['GenderConstants'];
      women: components['schemas']['GenderConstants'];
    };
    RisDistributionResponse: {
      men: components['schemas']['RisPerformanceResponse'][];
      women: components['schemas']['RisPerformanceResponse'][];
    };
    RisFormulaResponse: {
      constants: components['schemas']['RisConstants'];
      is_current: boolean;
      year: number;
    };
    RisPerformanceResponse: {
      athlete_name: string;
      athlete_slug: string;
      bodyweight: string;
      competition_date: string;
      competition_name: string;
      competition_slug: string;
      participant_id: string;
      total: string;
    };
    RisSource: 'computed' | 'reported';
    Scoring: 'total' | 'ris';
    StandingPlace: {
      field: number;
      place: number;
    };
    StrengthComparison: {
      field: number;
      movement_name: components['schemas']['Movement'];
      percentile: number | null;
      value: string | null;
    };
    StrengthProfile: {
      category: string;
      lifts: components['schemas']['StrengthComparison'][];
    };
  };
  responses: never;
  parameters: never;
  requestBodies: never;
  headers: never;
  pathItems: never;
}
export type $defs = Record<string, never>;
export interface operations {
  list_athletes: {
    parameters: {
      query?: {
        page?: number;
        page_size?: number;
      };
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': components['schemas']['PaginatedResponse_AthleteResponse'];
        };
      };
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  get_athlete: {
    parameters: {
      query?: {
        include?: components['schemas']['Include'];
      };
      header?: never;
      path: {
        slug: string;
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': components['schemas']['AthleteResponse'];
        };
      };
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  list_competitions: {
    parameters: {
      query?: {
        page?: number;
        page_size?: number;
        include?: components['schemas']['Include'];
        status?: components['schemas']['CompetitionStatus'] | null;
        event?: string | null;
        federation?: string | null;
        country?: string | null;
        year?: number | null;
        q?: string | null;
        direction?: components['schemas']['Direction'];
      };
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': components['schemas']['PaginatedResponse_CompetitionResponse'];
        };
      };
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  list_competition_countries: {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': string[];
        };
      };
    };
  };
  list_competition_federations: {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': string[];
        };
      };
    };
  };
  list_competition_formats: {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': string[];
        };
      };
    };
  };
  list_competition_years: {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': number[];
        };
      };
    };
  };
  get_competition: {
    parameters: {
      query?: {
        include?: components['schemas']['Include'];
      };
      header?: never;
      path: {
        slug: string;
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': components['schemas']['CompetitionResponse'];
        };
      };
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  get_global_ranking: {
    parameters: {
      query?: {
        page?: number;
        page_size?: number;
        gender?: components['schemas']['RankedGender'] | null;
        country?: string | null;
        federation?: string | null;
        q?: string | null;
        movement?: components['schemas']['RankingMetric'];
        direction?: components['schemas']['Direction'];
        event?: string | null;
        category?: string | null;
        year?: number | null;
        competition_id?: string | null;
      };
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': components['schemas']['PaginatedResponse_GlobalRankingEntry'];
        };
      };
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  list_ranking_classes: {
    parameters: {
      query?: {
        gender?: components['schemas']['Gender'] | null;
        competition_id?: string | null;
      };
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': string[];
        };
      };
    };
  };
  list_ranking_countries: {
    parameters: {
      query?: {
        competition_id?: string | null;
      };
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': string[];
        };
      };
    };
  };
  list_ranking_federations: {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': string[];
        };
      };
    };
  };
  list_ranking_years: {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': number[];
        };
      };
    };
  };
  calculate_ris: {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody: {
      content: {
        'application/json': components['schemas']['ComputeRisRequest'];
      };
    };
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': components['schemas']['ComputeRisResponse'];
        };
      };
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  get_ris_distribution: {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': components['schemas']['RisDistributionResponse'];
        };
      };
    };
  };
  list_ris_formulas: {
    parameters: {
      query?: {
        year?: number | null;
      };
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': components['schemas']['RisFormulaResponse'][];
        };
      };
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  get_current_formula: {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': components['schemas']['RisFormulaResponse'];
        };
      };
    };
  };
}
type FlattenedDeepRequired<T> = {
  [K in keyof T]-?: FlattenedDeepRequired<
    T[K] extends unknown[] | undefined | null ? Extract<T[K], unknown[]>[number] : T[K]
  >;
};
type ReadonlyArray<T> = [Exclude<T, undefined>] extends [unknown[]]
  ? Readonly<Exclude<T, undefined>>
  : Readonly<Exclude<T, undefined>[]>;
export const athleteStatusValues: ReadonlyArray<
  FlattenedDeepRequired<components>['schemas']['AthleteStatus']
> = ['competed', 'disqualified', 'no_show'];
export const competitionStatusValues: ReadonlyArray<
  FlattenedDeepRequired<components>['schemas']['CompetitionStatus']
> = ['draft', 'upcoming', 'live', 'completed', 'cancelled'];
export const directionValues: ReadonlyArray<
  FlattenedDeepRequired<components>['schemas']['Direction']
> = ['desc', 'asc'];
export const genderValues: ReadonlyArray<FlattenedDeepRequired<components>['schemas']['Gender']> = [
  'M',
  'F',
  'MX',
];
export const movementValues: ReadonlyArray<
  FlattenedDeepRequired<components>['schemas']['Movement']
> = ['Muscle-up', 'Pull-up', 'Dips', 'Squat'];
export const rankedGenderValues: ReadonlyArray<
  FlattenedDeepRequired<components>['schemas']['RankedGender']
> = ['M', 'F', 'MX'];
export const rankingMetricValues: ReadonlyArray<
  FlattenedDeepRequired<components>['schemas']['RankingMetric']
> = ['muscleup', 'pullup', 'dips', 'squat', 'total', 'ris'];
export const risSourceValues: ReadonlyArray<
  FlattenedDeepRequired<components>['schemas']['RisSource']
> = ['computed', 'reported'];
export const scoringValues: ReadonlyArray<FlattenedDeepRequired<components>['schemas']['Scoring']> =
  ['total', 'ris'];
