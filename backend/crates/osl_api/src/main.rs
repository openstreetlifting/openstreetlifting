use anyhow::Context;
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

mod athlete;
mod competition;
mod config;
mod error;
mod health;
mod ranking;
mod ris;
mod router;
mod shared;

use config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        competition::handlers::list_competitions,
        competition::handlers::get_competition,
        athlete::handlers::list_athletes,
        athlete::handlers::get_athlete,
        ranking::handler::get_global_ranking,
        ranking::handler::list_ranking_classes,
        ranking::handler::list_ranking_countries,
        ranking::handler::list_ranking_years,
        ris::handlers::list_ris_formulas,
        ris::handlers::get_current_formula,
        ris::handlers::get_formula_by_year,
        ris::handlers::calculate_ris,
        ris::handlers::get_participant_ris_scores,
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
            crate::athlete::dto::PersonalRecord,
            crate::shared::dto::PaginationMeta,
            crate::shared::dto::PaginationParams,
            crate::shared::query::Include,
            crate::ranking::dto::Movement,
            crate::ranking::dto::Direction,
            crate::ranking::dto::GlobalRankingEntry,
            crate::ranking::dto::AthleteInfo,
            crate::ranking::dto::CompetitionInfo,
            crate::ranking::dto::FederationInfo,
            crate::ris::dto::RisFormulaResponse,
            crate::ris::dto::RisConstants,
            crate::ris::dto::GenderConstants,
            crate::ris::dto::RisScoreResponse,
            crate::ris::dto::ComputeRisRequest,
            crate::ris::dto::ComputeRisResponse,
        )
    ),
    tags(
        (name = "competitions", description = "Public competition endpoints"),
        (name = "athletes", description = "Public athlete endpoints"),
        (name = "rankings", description = "Public ranking endpoints"),
        (name = "ris", description = "RIS formulas and score computation"),
    ),
)]
struct ApiDoc;

#[derive(Clone, Default)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _: &axum::http::Request<B>) -> Option<RequestId> {
        let id = Uuid::new_v4().to_string();
        HeaderValue::from_str(&id).ok().map(RequestId::new)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let log_format = std::env::var("LOG_FORMAT").unwrap_or_default();
    let filter =
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());

    match log_format.as_str() {
        "json" => {
            tracing_subscriber::fmt()
                .json()
                .with_env_filter(filter)
                .with_current_span(true)
                .init();
        }
        _ => {
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .with_target(true)
                .with_file(true)
                .with_line_number(true)
                .init();
        }
    }

    tracing::info!("Starting OpenStreetLifting API");

    let config = Config::from_env().context("Failed to load API configuration")?;
    tracing::info!("Configuration loaded successfully");

    tracing::info!(
        "Connecting to database at: {}",
        config
            .database_url
            .split('@')
            .next_back()
            .unwrap_or("unknown")
    );
    let db = Database::new(&config.database_url)
        .await
        .context("Failed to initialize database")?;
    tracing::info!("Database connection established");

    tracing::info!("Running database migrations");
    db.run_migrations()
        .await
        .context("Failed to run migrations")?;
    tracing::info!("Database migrations completed successfully");

    let state = AppState { db: Arc::new(db) };

    let bind_address = format!("{}:{}", config.host, config.port);
    tracing::info!("Starting server at http://{}", bind_address);
    tracing::info!(
        "Swagger UI available at http://{}/swagger-ui/",
        bind_address
    );

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

    let app = Router::new()
        .merge(health::routes::router())
        .merge(swagger_ui)
        .nest("/api/v1", router::api_router())
        .layer(middleware_stack)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl-C handler");
    };

    #[cfg(unix)]
    let sigterm = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let sigterm = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c  => { tracing::info!("Received Ctrl-C, shutting down"); }
        _ = sigterm => { tracing::info!("Received SIGTERM, shutting down"); }
    }
}
