use anyhow::Context;
use osl_api::{AppState, app, config::Config};
use osl_db::Database;
use utoipa::OpenApi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // cargo run -p osl_api -- --dump-openapi > openapi.json
    if std::env::args().any(|arg| arg == "--dump-openapi") {
        println!("{}", osl_api::ApiDoc::openapi().to_pretty_json()?);
        return Ok(());
    }

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

    tracing::info!("Starting OpenStreetlifting API");

    let config = Config::from_env().context("Failed to load API configuration")?;
    tracing::debug!(
        cache_enabled = config.cache_enabled,
        "API configuration loaded"
    );
    tracing::debug!("Connecting to PostgreSQL");
    let db = Database::new(&config.database_url)
        .await
        .context("Failed to initialize database")?;
    tracing::debug!("Connected to PostgreSQL");

    tracing::debug!("Running database migrations");
    db.run_migrations()
        .await
        .context("Failed to run migrations")?;
    tracing::info!("Database migrations completed");
    let state = AppState::new(db, config.cache_enabled);

    let bind_address = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    tracing::info!("API listening at http://{}", bind_address);
    tracing::debug!(
        "Swagger UI available at http://{}/swagger-ui/",
        bind_address
    );

    axum::serve(listener, app(state))
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
