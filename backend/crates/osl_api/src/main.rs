use anyhow::Context;
use axum::Router;
use std::sync::Arc;
use osl_db::Database;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod error;
mod handlers;
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
        handlers::competitions::list_competitions,
        handlers::competitions::list_competitions_detailed,
        handlers::competitions::get_competition,
        handlers::competitions::get_competition_detailed,
        handlers::competitions::create_competition,
        handlers::competitions::update_competition,
        handlers::competitions::delete_competition,
        handlers::athletes::list_athletes,
        handlers::athletes::get_athlete,
        handlers::athletes::get_athlete_detailed,
        handlers::athletes::create_athlete,
        handlers::athletes::update_athlete,
        handlers::athletes::delete_athlete,
        handlers::ranking::get_global_ranking,
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .init();

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

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .max_age(std::time::Duration::from_secs(3600));

    let swagger_ui: Router<AppState> =
        SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()).into();

    let app = Router::new()
        .merge(swagger_ui)
        .nest("/api", routes::api_router(state.clone()))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
