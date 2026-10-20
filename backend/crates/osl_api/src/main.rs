use anyhow::Context;
use osl_api::{AppState, app, config::Config, telemetry};
use osl_db::Database;
use utoipa::OpenApi;

#[tokio::main]
async fn main() -> std::process::ExitCode {
    if std::env::args().any(|arg| arg == "--dump-openapi") {
        return osl_api::ApiDoc::openapi().to_pretty_json().map_or(
            std::process::ExitCode::FAILURE,
            |schema| {
                println!("{schema}");
                std::process::ExitCode::SUCCESS
            },
        );
    }

    dotenvy::dotenv().ok();
    let Ok(_telemetry) = telemetry::init("osl-api", "info,sqlx=off") else {
        osl_telemetry::report_startup_failure("osl-api");
        return std::process::ExitCode::FAILURE;
    };
    if run().await.is_err() {
        tracing::error!(fatal = true, error.type = "api_runtime", "API process terminated");
        std::process::ExitCode::FAILURE
    } else {
        std::process::ExitCode::SUCCESS
    }
}

async fn run() -> anyhow::Result<()> {
    let metrics = telemetry::init_recorder()?;
    let upkeep = metrics.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            upkeep.run_upkeep();
        }
    });

    tracing::info!("API process initialized");

    let config = Config::from_env().context("Failed to load API configuration")?;
    tracing::debug!(
        cache_enabled = config.cache_enabled,
        "API configuration loaded"
    );
    tracing::debug!("API database connection requested");
    let db = Database::new(&config.database_url)
        .await
        .context("Failed to initialize database")?;
    tracing::debug!("API database connection established");

    tracing::debug!("Database migration execution requested");
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
    tracing::info!(server.address = %config.host, server.port = config.port, "API listener started");
    tracing::debug!(url.path = "/swagger-ui/", "Swagger UI route registered");

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
        () = ctrl_c  => { tracing::info!("API shutdown requested by Ctrl-C"); }
        () = sigterm => { tracing::info!("API shutdown requested by SIGTERM"); }
    }
}
