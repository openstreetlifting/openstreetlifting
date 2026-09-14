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
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing::Level;
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
pub mod ris;
pub mod router;
pub mod shared;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub caches: Arc<cache::AppCaches>,
}

impl AppState {
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
        competition::handlers::list_competition_federations,
        competition::handlers::list_competition_years,
        competition::handlers::list_competition_countries,
        athlete::handlers::list_athletes,
        athlete::handlers::get_athlete,
        ranking::handler::get_global_ranking,
        ranking::handler::list_ranking_classes,
        ranking::handler::list_ranking_countries,
        ranking::handler::list_ranking_federations,
        ranking::handler::list_ranking_years,
        ris::handlers::list_ris_formulas,
        ris::handlers::get_current_formula,
        ris::handlers::get_formula_by_year,
        ris::handlers::calculate_ris,
        ris::handlers::get_ris_distribution,
    ),
    components(
        schemas(
            crate::competition::dto::CompetitionResponse,
            crate::competition::dto::CategoryDetail,
            crate::competition::dto::ParticipantDetail,
            crate::competition::dto::LiftDetail,
            crate::competition::dto::AttemptInfo,
            crate::competition::dto::FederationInfo,
            crate::competition::dto::CategoryInfo,
            crate::competition::dto::AthleteInfo,
            crate::competition::dto::MovementInfo,
            crate::athlete::dto::AthleteResponse,
            crate::athlete::dto::AthleteCompetitionSummary,
            crate::athlete::dto::AthleteLift,
            crate::athlete::dto::PersonalRecord,
            crate::shared::dto::PaginationMeta,
            crate::shared::dto::PaginationParams,
            crate::shared::query::Include,
            osl_domain::AthleteStatus,
            osl_domain::CompetitionStatus,
            osl_domain::Gender,
            osl_domain::Movement,
            crate::shared::filters::RankedGender,
            osl_domain::RisSource,
            crate::ranking::dto::RankingMetric,
            crate::shared::dto::Direction,
            crate::ranking::dto::GlobalRankingEntry,
            crate::ranking::dto::AthleteInfo,
            crate::ranking::dto::CompetitionInfo,
            crate::ranking::dto::FederationInfo,
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
        (name = "competitions", description = "Public competition endpoints"),
        (name = "athletes", description = "Public athlete endpoints"),
        (name = "rankings", description = "Public ranking endpoints"),
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
        .max_age(Duration::from_secs(3600));

    let middleware_stack = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(
            x_request_id.clone(),
            MakeRequestUuid,
        ))
        .layer(PropagateRequestIdLayer::new(x_request_id))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &axum::http::Request<_>| {
                    let rid = req
                        .headers()
                        .get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("unknown");
                    tracing::info_span!(
                        "http_request",
                        method = %req.method(),
                        uri = %req.uri().path(),
                        request_id = %rid,
                    )
                })
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
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
    fn the_committed_schema_is_current() {
        let committed = include_str!("../../../openapi.json");

        assert_eq!(
            ApiDoc::openapi().to_pretty_json().unwrap(),
            committed.trim_end(),
            "backend/openapi.json is stale, run: \
             cargo run -p osl_api -- --dump-openapi > openapi.json"
        );
    }
}
