use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use osl_importer::canonical::{
    format as canonical_format, models::CanonicalFormat, transformer::CanonicalTransformer,
    validator::CanonicalValidator,
};
use osl_importer::sync::CompetitionSync;
use sqlx::postgres::PgPoolOptions;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "osl-import")]
#[command(about = "OpenStreetLifting Competition Data Importer", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Optional because `fmt` never touches the database.
    #[arg(long, env = "DATABASE_URL")]
    database_url: Option<String>,

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

        /// Delete the competitions no file in the tree claims, and the athletes
        /// they leave without a result. Reports what it would delete unless
        /// `--yes` is passed, and needs the whole imports tree to be right.
        #[arg(long)]
        prune: bool,

        /// Carry out the prune instead of only reporting it.
        #[arg(long)]
        yes: bool,
    },
    /// Attach Instagram handles to athletes from a Name,Instagram file.
    Instagram {
        #[arg(default_value = "./athlete-data/social-instagram.csv")]
        file: PathBuf,

        #[arg(long)]
        validate_only: bool,
    },
    /// Recompute every stored RIS score against the current formula.
    RecomputeRis,
    /// Rewrite canonical files in their canonical shape.
    Fmt {
        /// Files or directories. Directories are searched for .json.
        #[arg(default_value = "./imports")]
        paths: Vec<PathBuf>,

        /// Report files that would change instead of rewriting them.
        #[arg(long)]
        check: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
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
            let database_url = require_database_url(cli.database_url.as_deref(), validate_only)?;
            handle_canonical_import(file, validate_only, database_url).await?;
        }
        Commands::BulkImport {
            directory,
            validate_only,
            prune,
            yes,
        } => {
            if prune && validate_only {
                bail!("--prune writes to the database, so it cannot run with --validate-only");
            }

            let database_url = require_database_url(cli.database_url.as_deref(), validate_only)?;
            handle_bulk_import(directory, validate_only, prune, yes, database_url).await?;
        }
        Commands::Instagram {
            file,
            validate_only,
        } => {
            let database_url = require_database_url(cli.database_url.as_deref(), validate_only)?;
            handle_instagram(file, validate_only, database_url).await?;
        }
        Commands::RecomputeRis => {
            let database_url = require_database_url(cli.database_url.as_deref(), false)?;
            handle_recompute_ris(database_url).await?;
        }
        Commands::Fmt { paths, check } => {
            handle_fmt(&paths, check).await?;
        }
    }

    Ok(())
}

fn require_database_url(database_url: Option<&str>, validate_only: bool) -> Result<&str> {
    match database_url {
        Some(url) => Ok(url),
        None if validate_only => Ok(""),
        None => bail!("DATABASE_URL is required to import. Pass --validate-only to skip it"),
    }
}

async fn handle_instagram(file: PathBuf, validate_only: bool, database_url: &str) -> Result<()> {
    if validate_only {
        let count = osl_importer::social::validate_file(&file)?;
        tracing::info!("{} handle(s) in {}", count, file.display());
        return Ok(());
    }

    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("connecting to the database")?;

    let report = osl_importer::social::load_instagram_handles(&file, &pool).await?;
    tracing::info!("Attached {} handle(s)", report.matched);

    Ok(())
}

async fn handle_recompute_ris(database_url: &str) -> Result<()> {
    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("connecting to the database")?;

    let count = osl_db::services::ris_computation::recompute_all_ris(&pool, None).await?;
    tracing::info!("✓ Recomputed RIS for {} participant(s)", count);

    Ok(())
}

async fn handle_fmt(paths: &[PathBuf], check: bool) -> Result<()> {
    let mut files = Vec::new();
    for path in paths {
        collect_json_files(path, &mut files).await?;
    }
    files.sort();

    if files.is_empty() {
        tracing::warn!("No JSON files found");
        return Ok(());
    }

    let mut changed = Vec::new();
    for file in &files {
        let original = tokio::fs::read_to_string(file).await?;
        let mut canonical: CanonicalFormat = serde_json::from_str(&original)?;
        canonical_format::normalize(&mut canonical);
        let formatted = canonical_format::to_string(&canonical)?;

        if formatted == original {
            continue;
        }

        changed.push(file.clone());
        if !check {
            tokio::fs::write(file, formatted).await?;
        }
    }

    if changed.is_empty() {
        tracing::info!("{} file(s) already formatted", files.len());
        return Ok(());
    }

    for file in &changed {
        tracing::info!("{}", file.display());
    }

    if check {
        bail!(
            "{} file(s) are not formatted. Run `import fmt` to fix",
            changed.len()
        );
    }

    tracing::info!("Formatted {} file(s)", changed.len());
    Ok(())
}

async fn handle_canonical_import(
    file: PathBuf,
    validate_only: bool,
    database_url: &str,
) -> Result<()> {
    tracing::info!("Loading canonical JSON from: {}", file.display());

    let json_content = tokio::fs::read_to_string(&file).await?;
    let canonical: CanonicalFormat = serde_json::from_str(&json_content)?;

    tracing::info!(
        "Loaded competition: {} (v{})",
        canonical.competition.name,
        canonical.format_version
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
        .await
        .context("connecting to the database")?;

    tracing::info!(
        "Importing {} categories to database...",
        canonical.categories.len()
    );
    let transformer = CanonicalTransformer::new(&pool);
    transformer.import_to_database(canonical).await?;

    tracing::info!("✓ Import completed successfully!");

    Ok(())
}

/// Gathers every .json under `path`, or `path` itself when it is a file.
async fn collect_json_files(path: &Path, found: &mut Vec<PathBuf>) -> Result<()> {
    let mut pending = vec![path.to_path_buf()];

    while let Some(current) = pending.pop() {
        if !current.is_dir() {
            if current.extension().is_some_and(|ext| ext == "json") {
                found.push(current);
            }
            continue;
        }

        let mut entries = tokio::fs::read_dir(&current).await?;
        while let Some(entry) = entries.next_entry().await? {
            pending.push(entry.path());
        }
    }

    Ok(())
}

/// The competitions this tree claims, which is also what a prune keeps.
///
/// An import owns its whole competition and removes the rows its file does not
/// list, so two files claiming one slug would each delete the other's results.
/// Sessions of the same meet belong in one file, and that has to hold before
/// anything is written.
async fn claimed_competition_slugs(files: &[PathBuf]) -> Result<Vec<String>> {
    let mut by_slug: BTreeMap<String, Vec<&PathBuf>> = BTreeMap::new();

    for file in files {
        // Unreadable and malformed files are the import loop's to report.
        let Ok(content) = tokio::fs::read_to_string(file).await else {
            continue;
        };
        let Ok(canonical) = serde_json::from_str::<CanonicalFormat>(&content) else {
            continue;
        };

        by_slug
            .entry(canonical.competition.slug)
            .or_default()
            .push(file);
    }

    misplaced_files(&by_slug)?;

    let clashes: Vec<_> = by_slug
        .iter()
        .filter(|(_, files)| files.len() > 1)
        .collect();

    if clashes.is_empty() {
        return Ok(by_slug.into_keys().collect());
    }

    for (slug, files) in &clashes {
        tracing::error!(
            "Competition '{}' is claimed by {} files:",
            slug,
            files.len()
        );
        for file in files.iter() {
            tracing::error!("  {}", file.display());
        }
    }

    bail!(
        "{} competition slug(s) claimed by more than one file. Merge them into one file per competition",
        clashes.len()
    )
}

/// The competition page links to its canonical file at
/// `imports/{slug}/{slug}.json`, so a file stored anywhere else would publish a
/// dead link. The path is part of the contract, not a habit.
fn misplaced_files(by_slug: &BTreeMap<String, Vec<&PathBuf>>) -> Result<()> {
    let misplaced: Vec<_> = by_slug
        .iter()
        .flat_map(|(slug, files)| files.iter().map(move |file| (slug, *file)))
        .filter(|(slug, file)| {
            let stem = file.file_stem().and_then(|name| name.to_str());
            let directory = file
                .parent()
                .and_then(|parent| parent.file_name())
                .and_then(|name| name.to_str());
            stem != Some(slug.as_str()) || directory != Some(slug.as_str())
        })
        .collect();

    if misplaced.is_empty() {
        return Ok(());
    }

    for (slug, file) in &misplaced {
        tracing::error!(
            "Competition '{}' should live at {}/{}.json, found {}",
            slug,
            slug,
            slug,
            file.display()
        );
    }

    bail!(
        "{} canonical file(s) are not stored as {{slug}}/{{slug}}.json",
        misplaced.len()
    )
}

async fn handle_bulk_import(
    directory: PathBuf,
    validate_only: bool,
    prune: bool,
    yes: bool,
    database_url: &str,
) -> Result<()> {
    tracing::info!(
        "Scanning directory for canonical JSON files: {}",
        directory.display()
    );

    let mut json_files = Vec::new();
    collect_json_files(&directory, &mut json_files).await?;

    if json_files.is_empty() {
        tracing::warn!("No JSON files found in {}", directory.display());
        return Ok(());
    }

    json_files.sort();
    tracing::info!("Found {} canonical JSON file(s)", json_files.len());

    let claimed_slugs = claimed_competition_slugs(&json_files).await?;

    let pool = if !validate_only {
        tracing::info!("Connecting to database...");
        Some(
            PgPoolOptions::new()
                .max_connections(5)
                .connect(database_url)
                .await
                .context("connecting to the database")?,
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
        if prune {
            tracing::warn!("Skipping the prune: a file that failed to import claims nothing");
        }
        bail!("{} file(s) failed to import", error_count);
    }

    if prune && let Some(pool) = pool.as_ref() {
        handle_prune(pool, &claimed_slugs, yes).await?;
    }

    Ok(())
}

async fn handle_prune(pool: &sqlx::PgPool, claimed_slugs: &[String], yes: bool) -> Result<()> {
    let sync = CompetitionSync::new(pool);

    let plan = if yes {
        sync.apply(claimed_slugs).await?
    } else {
        sync.dry_run(claimed_slugs).await?
    };

    if plan.is_empty() {
        tracing::info!("Nothing to prune: every stored competition is claimed by a file");
        return Ok(());
    }

    if !plan.competitions.is_empty() {
        tracing::info!("Competitions no file claims:");
        for competition in &plan.competitions {
            tracing::info!("  {} ({})", competition.slug, competition.name);
        }
    }

    if !plan.athletes.is_empty() {
        tracing::info!("Athletes left without a result:");
        for athlete in &plan.athletes {
            tracing::info!("  {}", athlete);
        }
    }

    if yes {
        tracing::info!(
            "Deleted {} competition(s) and {} athlete(s)",
            plan.competitions.len(),
            plan.athletes.len()
        );
    } else {
        tracing::warn!(
            "Would delete {} competition(s) and {} athlete(s). Pass --yes to carry it out",
            plan.competitions.len(),
            plan.athletes.len()
        );
    }

    Ok(())
}

async fn process_canonical_file(
    file_path: &PathBuf,
    validate_only: bool,
    pool: Option<&sqlx::PgPool>,
) -> Result<()> {
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
