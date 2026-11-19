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

const API_DESCRIPTION: &str = "Streetlifting competitions, athletes, rankings, records and RIS \
scores from the OpenStreetlifting archive.

- Weights are in kilograms. Decimals are strings, such as `\"315.5\"`; ranking values are numbers.
- Dates use `YYYY-MM-DD`. Countries use ISO 3166-1 alpha-2 codes.
- `MPDS` is muscle-up, pull-up, dips and squat. Shorter events use their letters, such as `PD`.
- Lists return `data`. Paged lists add `pagination`.
- `include` adds optional sections, comma-separated.
- Errors return `{\"error\": \"...\"}`. A query value of the wrong type returns plain text.

[Licensing guide](https://docs.openstreetlifting.org/LICENSING.html)";

#[derive(OpenApi)]
#[openapi(
    info(
        title = "OpenStreetlifting API",
        description = API_DESCRIPTION,
        contact(name = "OpenStreetlifting", url = "https://openstreetlifting.org/contact"),
        license(
            name = "Licensing guide",
            url = "https://docs.openstreetlifting.org/LICENSING.html"
        ),
    ),
    servers(
        (url = "https://api.openstreetlifting.org", description = "Production"),
        (url = "/", description = "This server"),
    ),
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
            crate::error::ErrorResponse,
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
        (name = "competitions", description = "Competitions and their results"),
        (name = "athletes", description = "Athletes and their history"),
        (name = "rankings", description = "Rankings by lift, total or RIS"),
        (name = "records", description = "Best results in the archive"),
        (name = "ris", description = "RIS formulas and scores"),
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
    fn documents_every_endpoint_parameter_and_field() {
        let schema = serde_json::to_value(ApiDoc::openapi()).unwrap();
        assert!(schema["info"]["description"].is_string());
        assert_eq!(
            schema["servers"][0]["url"],
            "https://api.openstreetlifting.org"
        );

        for (path, operations) in schema["paths"].as_object().unwrap() {
            for (method, operation) in operations.as_object().unwrap() {
                assert!(operation["summary"].is_string(), "{method} {path}");
                assert!(operation["description"].is_string(), "{method} {path}");
                for parameter in operation["parameters"].as_array().into_iter().flatten() {
                    let name = &parameter["name"];
                    assert!(parameter["description"].is_string(), "{path}: {name}");
                }
                for (status, response) in operation["responses"].as_object().unwrap() {
                    if status.starts_with('4') {
                        assert_eq!(
                            response["content"]["application/json"]["schema"]["$ref"],
                            "#/components/schemas/ErrorResponse",
                            "{method} {path}: {status}"
                        );
                    }
                }
            }
        }

        for (name, component) in schema["components"]["schemas"].as_object().unwrap() {
            for (field, property) in component["properties"].as_object().into_iter().flatten() {
                let description = property
                    .get("description")
                    .or_else(|| property["oneOf"][0].get("description"));
                assert!(
                    description.is_some_and(serde_json::Value::is_string),
                    "{name}.{field}"
                );
            }
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
