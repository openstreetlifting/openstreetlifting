use axum::{
    Router,
    http::{HeaderName, HeaderValue, StatusCode},
};
use osl_db::Database;
use std::{sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    timeout::TimeoutLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

pub mod athlete;
pub mod cache;
pub mod competition;
pub mod config;
pub mod error;
pub mod health;
pub mod ranking;
pub mod records;
pub mod ris;
pub mod router;
pub mod shared;
pub mod telemetry;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub caches: Arc<cache::AppCaches>,
}

impl AppState {
    #[must_use]
    pub fn new(db: Database, cache_enabled: bool) -> Self {
        Self {
            db: Arc::new(db),
            caches: Arc::new(cache::AppCaches::new(cache_enabled)),
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        competition::handlers::list_competitions,
        competition::handlers::get_competition,
        competition::handlers::get_competition_facets,
        athlete::handlers::list_athletes,
        athlete::handlers::get_athlete,
        ranking::handler::get_global_ranking,
        ranking::handler::list_ranking_countries,
        ranking::handler::get_ranking_facets,
        records::handler::list_records,
        records::handler::get_record_facets,
        ris::handlers::list_ris_formulas,
        ris::handlers::get_current_formula,
        ris::handlers::calculate_ris,
        ris::handlers::get_ris_distribution,
    ),
    components(
        schemas(
            crate::competition::dto::CompetitionResponse,
            crate::competition::dto::CompetitionEdition,
            crate::competition::dto::CategoryDetail,
            crate::competition::dto::ParticipantDetail,
            crate::competition::dto::LiftDetail,
            crate::competition::dto::AttemptInfo,
            crate::competition::dto::FederationInfo,
            crate::competition::dto::CategoryInfo,
            crate::competition::dto::AthleteInfo,
            crate::competition::dto::MovementInfo,
            crate::athlete::dto::AthleteInclude,
            crate::competition::dto::CompetitionInclude,
            crate::competition::dto::CompetitionListInclude,
            crate::athlete::dto::AthleteResponse,
            crate::athlete::dto::AthleteCompetitionSummary,
            crate::athlete::dto::AthleteLift,
            crate::athlete::dto::PersonalRecord,
            crate::athlete::dto::BestTotal,
            crate::athlete::dto::BestRis,
            crate::shared::dto::PaginationMeta,
            crate::shared::dto::PaginationParams,
            crate::shared::query::Include,
            osl_domain::AthleteStatus,
            osl_domain::CompetitionStatus,
            osl_domain::Gender,
            osl_domain::Movement,
            osl_domain::RecordMark,
            crate::shared::filters::RankedGender,
            osl_domain::RisSource,
            crate::ranking::dto::RankingMetric,
            crate::shared::dto::Direction,
            crate::ranking::dto::GlobalRankingEntry,
            crate::ranking::dto::AthleteInfo,
            crate::ranking::dto::CompetitionInfo,
            crate::ranking::dto::FederationInfo,
            crate::ranking::dto::LiftRecord,
            crate::records::dto::RecordMovement,
            crate::records::dto::RecordGender,
            crate::records::dto::RecordGroup,
            crate::records::dto::RecordEntry,
            crate::records::dto::RecordAthlete,
            crate::records::dto::RecordCompetition,
            crate::ris::dto::RisFormulaResponse,
            crate::ris::dto::RisConstants,
            crate::ris::dto::GenderConstants,
            crate::ris::dto::ComputeRisRequest,
            crate::ris::dto::ComputeRisResponse,
            crate::ris::dto::RisDistributionResponse,
            crate::ris::dto::RisPerformanceResponse,
        )
    ),
    tags(
        (name = "competitions", description = "Competition endpoints"),
        (name = "athletes", description = "Athlete endpoints"),
        (name = "rankings", description = "Ranking endpoints"),
        (name = "records", description = "Best results in the OpenStreetlifting archive"),
        (name = "ris", description = "RIS formulas and score computation"),
    ),
)]
pub struct ApiDoc;

#[derive(Clone, Default)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _: &axum::http::Request<B>) -> Option<RequestId> {
        let id = Uuid::new_v4().to_string();
        HeaderValue::from_str(&id).ok().map(RequestId::new)
    }
}

pub fn app(state: AppState) -> Router {
    let x_request_id = HeaderName::from_static("x-request-id");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .expose_headers([HeaderName::from_static("traceparent"), x_request_id.clone()])
        .max_age(Duration::from_hours(1));

    let middleware_stack = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(
            x_request_id.clone(),
            MakeRequestUuid,
        ))
        .layer(PropagateRequestIdLayer::new(x_request_id))
        .layer(axum::middleware::from_fn(telemetry::instrument_http))
        .layer(axum::middleware::from_fn(telemetry::track_http))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::GATEWAY_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(cors)
        .layer(CompressionLayer::new());

    let swagger_ui: Router<AppState> = SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
        .into();

    Router::new()
        .merge(health::routes::router())
        .merge(swagger_ui)
        .nest("/api/v1", router::api_router())
        .layer(middleware_stack)
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_committed_openapi() {
        let committed: serde_json::Value =
            serde_json::from_str(include_str!("../../../openapi.json")).unwrap();

        assert_eq!(
            serde_json::to_value(ApiDoc::openapi()).unwrap(),
            committed,
            "backend/openapi.json is stale, run from frontend: pnpm api:sync"
        );
    }

    #[test]
    fn documents_athlete_includes_as_a_comma_separated_enum() {
        let schema = serde_json::to_value(ApiDoc::openapi()).unwrap();
        let parameters = schema["paths"]["/api/v1/athletes/{slug}"]["get"]["parameters"]
            .as_array()
            .unwrap();
        let include = parameters.iter().find(|p| p["name"] == "include").unwrap();
        assert_eq!(include["style"], "form");
        assert_eq!(include["explode"], false);
        assert_eq!(
            include["schema"]["items"]["$ref"],
            "#/components/schemas/AthleteInclude"
        );
        assert_eq!(
            schema["components"]["schemas"]["AthleteInclude"]["enum"],
            serde_json::json!([
                "competitions",
                "records",
                "standing",
                "lift_balance",
                "neighbors"
            ])
        );
    }

    #[test]
    fn documents_competition_includes_and_keeps_them_off_facets() {
        let schema = serde_json::to_value(ApiDoc::openapi()).unwrap();
        for (path, name) in [
            ("/api/v1/competitions", "CompetitionListInclude"),
            ("/api/v1/competitions/{slug}", "CompetitionInclude"),
        ] {
            let parameters = schema["paths"][path]["get"]["parameters"]
                .as_array()
                .unwrap();
            let include = parameters.iter().find(|p| p["name"] == "include").unwrap();
            assert_eq!(include["style"], "form", "{path}");
            assert_eq!(include["explode"], false, "{path}");
            assert_eq!(
                include["schema"]["items"]["$ref"],
                format!("#/components/schemas/{name}"),
            );
        }
        let facets = schema["paths"]["/api/v1/competitions/facets"]["get"]["parameters"]
            .as_array()
            .unwrap();
        for name in ["include", "q", "direction"] {
            assert!(facets.iter().all(|p| p["name"] != name), "{name}");
        }
    }

    #[test]
    fn documents_flat_query_pagination() {
        let schema = serde_json::to_value(ApiDoc::openapi()).unwrap();
        for path in [
            "/api/v1/athletes",
            "/api/v1/competitions",
            "/api/v1/rankings",
        ] {
            let parameters = schema["paths"][path]["get"]["parameters"]
                .as_array()
                .unwrap();
            for name in ["page", "page_size"] {
                let parameter = parameters.iter().find(|p| p["name"] == name).unwrap();
                assert_eq!(parameter["in"], "query", "{path}: {name}");
                assert_eq!(parameter["required"], false, "{path}: {name}");
            }
            assert!(parameters.iter().all(|p| p["name"] != "pagination"));
        }
    }

    #[test]
    fn separates_competition_and_ranking_schemas() {
        let schema = serde_json::to_value(ApiDoc::openapi()).unwrap();
        let schemas = &schema["components"]["schemas"];
        for field in ["federation", "athlete"] {
            let competition = if field == "federation" {
                &schemas["CompetitionResponse"]["properties"][field]
            } else {
                &schemas["ParticipantDetail"]["properties"][field]
            };
            let ranking = &schemas["GlobalRankingEntry"]["properties"][field];
            assert_ne!(competition["$ref"], ranking["$ref"]);
        }
        assert!(schemas["CompetitionFederation"]["properties"]["federation_id"].is_object());
        assert!(schemas["CompetitionAthlete"]["properties"]["bodyweight"].is_null());
        assert!(schemas["RankingAthlete"]["properties"]["bodyweight"].is_object());
    }
}
