use anyhow::Context;
use axum::{
    Router,
    http::{HeaderName, HeaderValue},
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

mod config;
mod error;
mod middleware;
mod routes;

use config::Config;
use middleware::auth::ApiKeys;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub api_keys: ApiKeys,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::competitions::list_competitions,
        routes::competitions::list_competitions_detailed,
        routes::competitions::get_competition,
        routes::competitions::get_competition_detailed,
        routes::competitions::create_competition,
        routes::competitions::update_competition,
        routes::competitions::delete_competition,
        routes::athletes::list_athletes,
        routes::athletes::get_athlete,
        routes::athletes::get_athlete_detailed,
        routes::athletes::create_athlete,
        routes::athletes::update_athlete,
        routes::athletes::delete_athlete,
        routes::ranking::get_global_ranking,
    ),
    components(
        schemas(
            osl_db::dto::competition::CreateCompetitionRequest,
            osl_db::dto::competition::UpdateCompetitionRequest,
            osl_db::dto::competition::CompetitionResponse,
            osl_db::dto::competition::CompetitionListResponse,
            osl_db::dto::competition::CompetitionDetailResponse,
            osl_db::dto::competition::CategoryDetail,
            osl_db::dto::competition::ParticipantDetail,
            osl_db::dto::competition::LiftDetail,
            osl_db::dto::competition::AttemptInfo,
            osl_db::dto::competition::FederationInfo,
            osl_db::dto::competition::CategoryInfo,
            osl_db::dto::competition::AthleteInfo,
            osl_db::dto::competition::MovementInfo,
            osl_db::dto::athlete::CreateAthleteRequest,
            osl_db::dto::athlete::UpdateAthleteRequest,
            osl_db::dto::athlete::AthleteResponse,
            osl_db::dto::athlete::AthleteDetailResponse,
            osl_db::dto::athlete::AthleteCompetitionSummary,
            osl_db::dto::athlete::PersonalRecord,
            osl_db::dto::common::PaginationMeta,
            osl_db::dto::ranking::GlobalRankingEntry,
            osl_db::dto::ranking::AthleteInfo,
            osl_db::dto::ranking::CompetitionInfo,
            osl_db::models::Competition,
            osl_db::models::Athlete,
            osl_db::models::Category,
            osl_db::models::Federation,
            osl_db::models::Movement,
            osl_db::models::Lift,
            osl_db::models::Attempt,
            osl_db::models::CompetitionParticipant,
            osl_db::models::Record,
            osl_db::models::Social,
            osl_db::models::Rulebook,
            osl_db::models::AthleteSocial,
        )
    ),
    tags(
        (name = "competitions", description = "Public competition endpoints"),
        (name = "athletes", description = "Public athlete endpoints"),
        (name = "rankings", description = "Public ranking endpoints"),
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("API Key")
                        .build(),
                ),
            )
        }
    }
}

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

    let state = AppState {
        db: Arc::new(db),
        api_keys: ApiKeys::from_comma_separated(&config.api_keys),
    };

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
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(cors)
        .layer(CompressionLayer::new());

    let swagger_ui: Router<AppState> = SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
        .into();

    let app = Router::new()
        .merge(routes::health::router())
        .merge(swagger_ui)
        .nest("/api", routes::api_router(state.clone()))
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
