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
  '/api/v1/competitions/facets': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['get_competition_facets'];
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
  '/api/v1/rankings/facets': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['get_ranking_facets'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/records': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['list_records'];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  '/api/v1/records/facets': {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get: operations['get_record_facets'];
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
      competition_date: string;
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
      total_record: components['schemas']['RecordMark'] | null;
    };
    AthleteInclude: 'competitions' | 'records' | 'standing' | 'lift_balance' | 'neighbors';
    AthleteLift: {
      attempts: components['schemas']['AttemptInfo'][];
      best_weight: string | null;
      movement_name: components['schemas']['Movement'];
      record: components['schemas']['RecordMark'] | null;
    };
    AthleteNeighbor: {
      athlete_id: string;
      competition_date: string | null;
      competition_name: string;
      competition_slug: string;
      country: string | null;
      field: number;
      first_name: string;
      gap: string;
      last_name: string;
      place: number;
      slug: string;
      value: string;
    };
    AthleteNeighborhood: {
      category: string;
      global: components['schemas']['AthleteNeighbor'][];
      national: components['schemas']['AthleteNeighbor'][] | null;
    };
    AthleteResponse: {
      athlete_id: string;
      best_ris?: components['schemas']['BestRis'] | null;
      best_total?: components['schemas']['BestTotal'] | null;
      competitions?: components['schemas']['AthleteCompetitionSummary'][];
      country: string | null;
      created_at: string;
      first_name: string;
      gender: components['schemas']['Gender'];
      instagram_handle: string | null;
      last_name: string;
      lift_balance?: components['schemas']['LiftBalance'] | null;
      native_name: string | null;
      neighbors?: components['schemas']['AthleteNeighborhood'] | null;
      personal_records?: components['schemas']['PersonalRecord'][];
      profile_picture_url: string | null;
      slug: string;
      standing?: components['schemas']['AthleteStanding'] | null;
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
    BestRis: {
      competition_name: string;
      competition_slug: string;
      date: string;
      ris_score: string;
      ris_source: components['schemas']['RisSource'];
    };
    BestTotal: {
      competition_name: string;
      competition_slug: string;
      date: string;
      record: components['schemas']['RecordMark'] | null;
      total: string;
    };
    CategoryDetail: {
      category: components['schemas']['CategoryInfo'];
      participants: components['schemas']['ParticipantDetail'][];
    };
    CategoryInfo: {
      division: string | null;
      gender: components['schemas']['Gender'];
      name: string;
      scoring: components['schemas']['Scoring'];
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
    CompetitionFacets: {
      countries: string[];
      federations: string[];
      formats: string[];
      years: number[];
    };
    CompetitionFederation: {
      abbreviation: string | null;
      country: string | null;
      federation_id: string;
      name: string;
    };
    CompetitionInclude: 'federation' | 'results' | 'movements' | 'editions';
    CompetitionInfo: {
      competition_id: string;
      date: string | null;
      name: string;
      slug: string;
    };
    CompetitionListInclude: 'federation' | 'movements';
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
    ErrorResponse: {
      error: string;
    };
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
      records: components['schemas']['LiftRecord'][];
      ris: number | null;
      ris_source: components['schemas']['RisSource'] | null;
      squat: number | null;
      total: number | null;
      total_record: components['schemas']['RecordMark'] | null;
    };
    LiftBalance: {
      category: string;
      competition_date: string;
      competition_name: string;
      competition_slug: string;
      field: number;
      lifts: components['schemas']['LiftShare'][];
      total: string;
    };
    LiftDetail: {
      attempts: components['schemas']['AttemptInfo'][];
      best_weight: string | null;
      movement_name: components['schemas']['Movement'];
      record: components['schemas']['RecordMark'] | null;
    };
    LiftRecord: {
      mark: components['schemas']['RecordMark'];
      movement: components['schemas']['Movement'];
    };
    LiftShare: {
      class_share: number | null;
      class_spread: number | null;
      movement_name: components['schemas']['Movement'];
      share: number;
      value: string;
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
        best_ris?: components['schemas']['BestRis'] | null;
        best_total?: components['schemas']['BestTotal'] | null;
        competitions?: components['schemas']['AthleteCompetitionSummary'][];
        country: string | null;
        created_at: string;
        first_name: string;
        gender: components['schemas']['Gender'];
        instagram_handle: string | null;
        last_name: string;
        lift_balance?: components['schemas']['LiftBalance'] | null;
        native_name: string | null;
        neighbors?: components['schemas']['AthleteNeighborhood'] | null;
        personal_records?: components['schemas']['PersonalRecord'][];
        profile_picture_url: string | null;
        slug: string;
        standing?: components['schemas']['AthleteStanding'] | null;
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
        records: components['schemas']['LiftRecord'][];
        ris: number | null;
        ris_source: components['schemas']['RisSource'] | null;
        squat: number | null;
        total: number | null;
        total_record: components['schemas']['RecordMark'] | null;
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
      age: number | null;
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
      total_record: components['schemas']['RecordMark'] | null;
    };
    PersonalRecord: {
      competition_name: string;
      competition_slug: string;
      date: string;
      max_weight: string;
      movement_name: components['schemas']['Movement'];
      record: components['schemas']['RecordMark'] | null;
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
    RankingFacets: {
      classes: string[];
      countries: string[];
      federations: string[];
      series: components['schemas']['RankingSeries'][];
      years: number[];
    };
    RankingFederation: {
      abbreviation: string | null;
      name: string;
    };
    RankingMetric: 'muscleup' | 'pullup' | 'dips' | 'squat' | 'total' | 'ris';
    RankingSeries: {
      federation: string;
      name: string;
      series: string;
    };
    RecordAthlete: {
      country: string | null;
      first_name: string;
      last_name: string;
      slug: string;
    };
    RecordCompetition: {
      name: string;
      slug: string;
    };
    RecordEntry: {
      athlete: components['schemas']['RecordAthlete'];
      competition: components['schemas']['RecordCompetition'];
      date: string;
      rank: number;
      weight: string;
    };
    RecordFacets: {
      countries: string[];
      custom_weight_classes: boolean;
      federations: string[];
      series: components['schemas']['RecordSeries'][];
      years: number[];
    };
    RecordGender: 'M' | 'F';
    RecordGroup: {
      entries: components['schemas']['RecordEntry'][];
      gender: components['schemas']['RecordGender'];
      movement: components['schemas']['RecordMovement'];
      weight_class: string | null;
    };
    RecordMark: 'WR' | 'NR';
    RecordMovement: 'muscleup' | 'pullup' | 'dips' | 'squat' | 'total';
    RecordSeries: {
      federation: string;
      name: string;
      series: string;
    };
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
        content: {
          'application/json': components['schemas']['ErrorResponse'];
        };
      };
    };
  };
  get_athlete: {
    parameters: {
      query?: {
        include?: components['schemas']['AthleteInclude'][];
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
        content: {
          'application/json': components['schemas']['ErrorResponse'];
        };
      };
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': components['schemas']['ErrorResponse'];
        };
      };
    };
  };
  list_competitions: {
    parameters: {
      query?: {
        page?: number;
        page_size?: number;
        status?: components['schemas']['CompetitionStatus'] | null;
        event?: string | null;
        federation?: string | null;
        country?: string | null;
        year?: number | null;
        include?: components['schemas']['CompetitionListInclude'][];
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
        content: {
          'application/json': components['schemas']['ErrorResponse'];
        };
      };
    };
  };
  get_competition_facets: {
    parameters: {
      query?: {
        status?: components['schemas']['CompetitionStatus'] | null;
        event?: string | null;
        federation?: string | null;
        country?: string | null;
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
          'application/json': components['schemas']['CompetitionFacets'];
        };
      };
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': components['schemas']['ErrorResponse'];
        };
      };
    };
  };
  get_competition: {
    parameters: {
      query?: {
        include?: components['schemas']['CompetitionInclude'][];
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
        content: {
          'application/json': components['schemas']['ErrorResponse'];
        };
      };
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': components['schemas']['ErrorResponse'];
        };
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
        series?: string | null;
        q?: string | null;
        movement?: components['schemas']['RankingMetric'];
        direction?: components['schemas']['Direction'];
        event?: string | null;
        category?: string | null;
        year?: number | null;
        competition_id?: string | null;
        division?: string | null;
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
        content: {
          'application/json': components['schemas']['ErrorResponse'];
        };
      };
    };
  };
  list_ranking_countries: {
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
  get_ranking_facets: {
    parameters: {
      query?: {
        gender?: components['schemas']['RankedGender'] | null;
        country?: string | null;
        federation?: string | null;
        series?: string | null;
        q?: string | null;
        movement?: components['schemas']['RankingMetric'];
        direction?: components['schemas']['Direction'];
        event?: string | null;
        category?: string | null;
        year?: number | null;
        competition_id?: string | null;
        division?: string | null;
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
          'application/json': components['schemas']['RankingFacets'];
        };
      };
    };
  };
  list_records: {
    parameters: {
      query?: {
        movement?: components['schemas']['RecordMovement'] | null;
        gender?: components['schemas']['RecordGender'] | null;
        weight_class?: string | null;
        country?: string | null;
        federation?: string | null;
        series?: string | null;
        year?: number | null;
        limit?: number | null;
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
          'application/json': components['schemas']['RecordGroup'][];
        };
      };
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': components['schemas']['ErrorResponse'];
        };
      };
    };
  };
  get_record_facets: {
    parameters: {
      query?: {
        movement?: components['schemas']['RecordMovement'] | null;
        gender?: components['schemas']['RecordGender'] | null;
        weight_class?: string | null;
        country?: string | null;
        federation?: string | null;
        series?: string | null;
        year?: number | null;
        limit?: number | null;
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
          'application/json': components['schemas']['RecordFacets'];
        };
      };
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': components['schemas']['ErrorResponse'];
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
        content: {
          'application/json': components['schemas']['ErrorResponse'];
        };
      };
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          'application/json': components['schemas']['ErrorResponse'];
        };
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
        content: {
          'application/json': components['schemas']['ErrorResponse'];
        };
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
export const athleteIncludeValues: ReadonlyArray<
  FlattenedDeepRequired<components>['schemas']['AthleteInclude']
> = ['competitions', 'records', 'standing', 'lift_balance', 'neighbors'];
export const athleteStatusValues: ReadonlyArray<
  FlattenedDeepRequired<components>['schemas']['AthleteStatus']
> = ['competed', 'disqualified', 'no_show'];
export const competitionIncludeValues: ReadonlyArray<
  FlattenedDeepRequired<components>['schemas']['CompetitionInclude']
> = ['federation', 'results', 'movements', 'editions'];
export const competitionListIncludeValues: ReadonlyArray<
  FlattenedDeepRequired<components>['schemas']['CompetitionListInclude']
> = ['federation', 'movements'];
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
export const recordGenderValues: ReadonlyArray<
  FlattenedDeepRequired<components>['schemas']['RecordGender']
> = ['M', 'F'];
export const recordMarkValues: ReadonlyArray<
  FlattenedDeepRequired<components>['schemas']['RecordMark']
> = ['WR', 'NR'];
export const recordMovementValues: ReadonlyArray<
  FlattenedDeepRequired<components>['schemas']['RecordMovement']
> = ['muscleup', 'pullup', 'dips', 'squat', 'total'];
export const risSourceValues: ReadonlyArray<
  FlattenedDeepRequired<components>['schemas']['RisSource']
> = ['computed', 'reported'];
export const scoringValues: ReadonlyArray<FlattenedDeepRequired<components>['schemas']['Scoring']> =
  ['total', 'ris'];
