use anyhow::Context;
use osl_api::{AppState, app, config::Config, telemetry};
use osl_db::Database;
use utoipa::OpenApi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::args().any(|arg| arg == "--dump-openapi") {
        println!("{}", osl_api::ApiDoc::openapi().to_pretty_json()?);
        return Ok(());
    }

    dotenvy::dotenv().ok();

    let _telemetry = telemetry::init("osl-api", "info,sqlx=off")?;
    let metrics = telemetry::init_recorder()?;

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
    let metrics_app = telemetry::metrics_router(metrics, state.db.clone());
    let metrics_listener =
        tokio::net::TcpListener::bind((config.metrics_host.as_str(), config.metrics_port)).await?;

    let bind_address = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    tracing::info!("API listening at http://{}", bind_address);
    tracing::debug!(
        "Swagger UI available at http://{}/swagger-ui/",
        bind_address
    );

    let (shutdown_tx, mut api_shutdown) = tokio::sync::watch::channel(false);
    let mut metrics_shutdown = api_shutdown.clone();
    let signal = tokio::spawn(async move {
        shutdown_signal().await;
        let _ = shutdown_tx.send(true);
    });
    let result = tokio::try_join!(
        async {
            axum::serve(listener, app(state))
                .with_graceful_shutdown(async move {
                    let _ = api_shutdown.changed().await;
                })
                .await
        },
        async {
            axum::serve(metrics_listener, metrics_app)
                .with_graceful_shutdown(async move {
                    let _ = metrics_shutdown.changed().await;
                })
                .await
        },
    );
    signal.abort();
    result?;

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
        () = ctrl_c  => { tracing::info!("Received Ctrl-C, shutting down"); }
        () = sigterm => { tracing::info!("Received SIGTERM, shutting down"); }
    }
}
