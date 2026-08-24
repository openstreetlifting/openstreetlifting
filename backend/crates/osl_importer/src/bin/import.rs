use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use osl_importer::canonical::{
    entries, format as canonical_format, meet, store, transformer::CanonicalTransformer,
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
        directory: PathBuf,

        #[arg(long)]
        validate_only: bool,
    },
    BulkImport {
        #[arg(long, default_value = "./data/competitions")]
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
        #[arg(default_value = "./data/athletes/instagram.csv")]
        file: PathBuf,

        #[arg(long)]
        validate_only: bool,
    },
    /// Recompute every stored RIS score against the current formula.
    RecomputeRis,
    /// Rewrite canonical files in their canonical shape.
    Fmt {
        /// Competition directories, or a tree to search for them.
        #[arg(default_value = "./data/competitions")]
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
            directory,
            validate_only,
        } => {
            let database_url = require_database_url(cli.database_url.as_deref(), validate_only)?;
            handle_canonical_import(directory, validate_only, database_url).await?;
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
    let mut directories = Vec::new();
    for path in paths {
        collect_competitions(path, &mut directories)?;
    }
    directories.sort();

    if directories.is_empty() {
        tracing::warn!("No competition directories found");
        return Ok(());
    }

    let mut changed = Vec::new();
    for directory in &directories {
        if is_formatted(directory)? {
            continue;
        }

        changed.push(directory.clone());
        if !check {
            let mut canonical = store::read(directory)?;
            canonical_format::normalize(&mut canonical);
            store::write(directory, &canonical)?;
        }
    }

    if changed.is_empty() {
        tracing::info!("{} competition(s) already formatted", directories.len());
        return Ok(());
    }

    for directory in &changed {
        tracing::info!("{}", directory.display());
    }

    if check {
        bail!(
            "{} competition(s) are not formatted. Run `import fmt` to fix",
            changed.len()
        );
    }

    tracing::info!("Formatted {} competition(s)", changed.len());
    Ok(())
}

fn is_formatted(directory: &Path) -> Result<bool> {
    let mut canonical = store::read(directory)?;
    canonical_format::normalize(&mut canonical);
    let (meet_text, entries_text) = store::render(&canonical)?;

    if std::fs::read_to_string(directory.join(meet::FILE_NAME))? != meet_text {
        return Ok(false);
    }

    let entries_path = directory.join(entries::FILE_NAME);

    match entries_text {
        Some(entries_text) => Ok(std::fs::read_to_string(&entries_path)? == entries_text),
        None => Ok(!entries_path.exists()),
    }
}

async fn handle_canonical_import(
    directory: PathBuf,
    validate_only: bool,
    database_url: &str,
) -> Result<()> {
    tracing::info!("Loading competition from: {}", directory.display());

    let canonical = store::read(&directory)?;

    tracing::info!("Loaded competition: {}", canonical.competition.name);

    tracing::info!("Validating canonical format...");
    let validation_report = CanonicalValidator::validate(&canonical)?;
    validation_report.log_warnings();
    tracing::info!("\u{2713} Validation successful!");

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

    tracing::info!("\u{2713} Import completed successfully!");

    Ok(())
}

/// Gathers every competition directory under `path`, or `path` itself when it
/// is one.
fn collect_competitions(path: &Path, found: &mut Vec<PathBuf>) -> Result<()> {
    let mut pending = vec![path.to_path_buf()];

    while let Some(current) = pending.pop() {
        if !current.is_dir() {
            continue;
        }

        if store::is_competition_directory(&current) {
            found.push(current);
            continue;
        }

        for entry in std::fs::read_dir(&current)? {
            pending.push(entry?.path());
        }
    }

    Ok(())
}

/// The competitions this tree claims, which is also what a prune keeps.
///
/// An import owns its whole competition and removes the rows its files do not
/// list, so two directories claiming one slug would each delete the other's
/// results. Sessions of the same meet belong in one directory, and that has to
/// hold before anything is written.
fn claimed_competition_slugs(directories: &[PathBuf]) -> Result<Vec<String>> {
    let mut by_slug: BTreeMap<String, Vec<&PathBuf>> = BTreeMap::new();

    for directory in directories {
        let slug = store::slug_of(directory)?;
        by_slug.entry(slug).or_default().push(directory);
    }

    let clashes: Vec<_> = by_slug
        .iter()
        .filter(|(_, directories)| directories.len() > 1)
        .collect();

    if clashes.is_empty() {
        return Ok(by_slug.into_keys().collect());
    }

    for (slug, directories) in &clashes {
        tracing::error!(
            "Competition '{}' is claimed by {} directories:",
            slug,
            directories.len()
        );
        for directory in directories.iter() {
            tracing::error!("  {}", directory.display());
        }
    }

    bail!(
        "{} competition slug(s) claimed by more than one directory. Merge them into one directory per competition",
        clashes.len()
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
        "Scanning directory for competitions: {}",
        directory.display()
    );

    let mut competitions = Vec::new();
    collect_competitions(&directory, &mut competitions)?;

    if competitions.is_empty() {
        tracing::warn!("No competitions found in {}", directory.display());
        return Ok(());
    }

    competitions.sort();
    tracing::info!("Found {} competition(s)", competitions.len());

    let claimed_slugs = claimed_competition_slugs(&competitions)?;

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

    for (idx, competition) in competitions.iter().enumerate() {
        tracing::info!(
            "[{}/{}] Processing: {}",
            idx + 1,
            competitions.len(),
            competition.display()
        );

        match process_competition(competition, validate_only, pool.as_ref()).await {
            Ok(_) => {
                success_count += 1;
                tracing::info!("  \u{2713} Success");
            }
            Err(e) => {
                error_count += 1;
                tracing::error!("  \u{2717} Error: {}", e);
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
            tracing::warn!(
                "Skipping the prune: a competition that failed to import claims nothing"
            );
        }
        bail!("{} competition(s) failed to import", error_count);
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

async fn process_competition(
    directory: &Path,
    validate_only: bool,
    pool: Option<&sqlx::PgPool>,
) -> Result<()> {
    let canonical = store::read(directory)?;
    store::check_location(directory, &canonical)?;

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
