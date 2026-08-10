use clap::{Parser, Subcommand};
use osl_importer::canonical::{
    models::CanonicalFormat, transformer::CanonicalTransformer, validator::CanonicalValidator,
};
use sqlx::postgres::PgPoolOptions;
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "osl-import")]
#[command(about = "OpenStreetLifting Competition Data Importer", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, env = "DATABASE_URL")]
    database_url: String,

    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    Canonical {
        file: PathBuf,

        #[arg(long)]
        validate_only: bool,
    },
    BulkImport {
        #[arg(long, default_value = "./imports")]
        directory: PathBuf,

        #[arg(long)]
        validate_only: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("import={},osl_importer={}", log_level, log_level).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    match cli.command {
        Commands::Canonical {
            file,
            validate_only,
        } => {
            handle_canonical_import(file, validate_only, &cli.database_url).await?;
        }
        Commands::BulkImport {
            directory,
            validate_only,
        } => {
            handle_bulk_import(directory, validate_only, &cli.database_url).await?;
        }
    }

    Ok(())
}

async fn handle_canonical_import(
    file: PathBuf,
    validate_only: bool,
    database_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Loading canonical JSON from: {}", file.display());

    let json_content = tokio::fs::read_to_string(&file).await?;
    let canonical: CanonicalFormat = serde_json::from_str(&json_content)?;

    tracing::info!(
        "Loaded competition: {} (v{})",
        canonical.competition.name,
        canonical.source.format_version
    );

    tracing::info!("Validating canonical format...");
    let validation_report = CanonicalValidator::validate(&canonical)?;
    validation_report.log_warnings();
    tracing::info!("✓ Validation successful!");

    if validate_only {
        return Ok(());
    }

    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    tracing::info!(
        "Importing {} categories to database...",
        canonical.categories.len()
    );
    let transformer = CanonicalTransformer::new(&pool);
    transformer.import_to_database(canonical).await?;

    tracing::info!("✓ Import completed successfully!");

    Ok(())
}

async fn handle_bulk_import(
    directory: PathBuf,
    validate_only: bool,
    database_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!(
        "Scanning directory for canonical JSON files: {}",
        directory.display()
    );

    let mut json_files = Vec::new();
    let mut entries = tokio::fs::read_dir(&directory).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            let mut sub_entries = tokio::fs::read_dir(&path).await?;
            while let Some(sub_entry) = sub_entries.next_entry().await? {
                let sub_path = sub_entry.path();
                if sub_path.extension().is_some_and(|ext| ext == "json") {
                    json_files.push(sub_path);
                }
            }
        } else if path.extension().is_some_and(|ext| ext == "json") {
            json_files.push(path);
        }
    }

    if json_files.is_empty() {
        tracing::warn!("No JSON files found in {}", directory.display());
        return Ok(());
    }

    json_files.sort();
    tracing::info!("Found {} canonical JSON file(s)", json_files.len());

    let pool = if !validate_only {
        tracing::info!("Connecting to database...");
        Some(
            PgPoolOptions::new()
                .max_connections(5)
                .connect(database_url)
                .await?,
        )
    } else {
        None
    };

    let mut success_count = 0;
    let mut error_count = 0;

    for (idx, file_path) in json_files.iter().enumerate() {
        tracing::info!(
            "[{}/{}] Processing: {}",
            idx + 1,
            json_files.len(),
            file_path.display()
        );

        match process_canonical_file(file_path, validate_only, pool.as_ref()).await {
            Ok(_) => {
                success_count += 1;
                tracing::info!("  ✓ Success");
            }
            Err(e) => {
                error_count += 1;
                tracing::error!("  ✗ Error: {}", e);
            }
        }
    }

    tracing::info!(
        "Summary: {} succeeded, {} failed",
        success_count,
        error_count
    );

    if error_count > 0 {
        return Err(format!("{} file(s) failed to import", error_count).into());
    }

    Ok(())
}

async fn process_canonical_file(
    file_path: &PathBuf,
    validate_only: bool,
    pool: Option<&sqlx::PgPool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let json_content = tokio::fs::read_to_string(file_path).await?;
    let canonical: CanonicalFormat = serde_json::from_str(&json_content)?;

    let validation_report = CanonicalValidator::validate(&canonical)?;

    if !validation_report.warnings.is_empty() {
        for warning in &validation_report.warnings {
            tracing::warn!("  {}", warning);
        }
    }

    if !validate_only && let Some(pool) = pool {
        let transformer = CanonicalTransformer::new(pool);
        transformer.import_to_database(canonical).await?;
    }

    Ok(())
}
