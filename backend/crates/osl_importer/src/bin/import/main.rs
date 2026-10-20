use anyhow::{Context, Result, bail};
use clap::Parser;
use osl_importer::canonical::{
    competition, entries, format as canonical_format, store, transformer::CanonicalTransformer,
    validator::CanonicalValidator,
};
use osl_importer::identity::AthleteQuery;
use osl_importer::privacy::{self, PrivacyList};
use osl_importer::redact;
use osl_importer::sync::SyncPlan;
use sqlx::postgres::PgPoolOptions;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

mod cli;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> std::process::ExitCode {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    let log_level = if cli.verbose { "debug" } else { "info" };
    let Ok(_telemetry) = osl_telemetry::init(
        "osl-importer",
        &format!("import={log_level},osl_importer={log_level},osl_telemetry=info"),
    ) else {
        osl_telemetry::report_startup_failure("osl-importer");
        return std::process::ExitCode::FAILURE;
    };
    match run(cli).await {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            // Interactive command diagnostics are not sent to JSON telemetry.
            if std::env::var("LOG_FORMAT").as_deref() != Ok("json") {
                eprintln!("{error:#}");
            }
            tracing::error!(fatal = true, error.type = "import_command", "Importer command terminated");
            std::process::ExitCode::FAILURE
        }
    }
}

async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Competitions {
            paths,
            dry_run,
            prune,
            skip_privacy_check,
        } => {
            let database_url = optional_database_url(cli.database_url.as_deref(), dry_run)?;
            let privacy = import_privacy(&cli.privacy_file, skip_privacy_check)?;
            handle_competitions(&paths, dry_run, prune, database_url, privacy.as_ref()).await?;
        }
        Commands::Redact {
            name,
            sex,
            country,
            disambiguation,
            directory,
            instagram_file,
            dry_run,
        } => {
            handle_redact(
                &cli.privacy_file,
                &directory,
                &instagram_file,
                &name,
                sex.map(|sex| sex.to_string()),
                country.map(|country| country.to_string()),
                disambiguation,
                dry_run,
            )?;
        }
        Commands::Privacy { directory } => {
            handle_privacy(&cli.privacy_file, &directory)?;
        }
        Commands::Instagram { file, dry_run } => {
            let database_url = optional_database_url(cli.database_url.as_deref(), dry_run)?;
            handle_instagram(file, dry_run, database_url).await?;
        }
        Commands::RecomputeRis { dry_run } => {
            let database_url = cli
                .database_url
                .as_deref()
                .context("DATABASE_URL is required to read stored RIS scores")?;
            handle_recompute_ris(database_url, dry_run).await?;
        }
        Commands::Prepare {
            paths,
            check,
            dry_run,
        } => {
            handle_prepare(&paths, check, dry_run)?;
        }
    }

    Ok(())
}

fn optional_database_url(database_url: Option<&str>, dry_run: bool) -> Result<Option<&str>> {
    match database_url {
        Some(url) => Ok(Some(url)),
        None if dry_run => Ok(None),
        None => bail!("DATABASE_URL is required to import. Pass --dry-run to skip it"),
    }
}

async fn handle_instagram(file: PathBuf, dry_run: bool, database_url: Option<&str>) -> Result<()> {
    let Some(database_url) = database_url else {
        let count = osl_importer::social::validate_file(&file)?;
        tracing::warn!(
            handles = count,
            osl.file.kind = "instagram",
            "Instagram CSV validation completed without database matching"
        );
        return Ok(());
    };

    tracing::debug!("Instagram database connection requested");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("connecting to the database")?;

    if dry_run {
        let report = osl_importer::social::check_instagram_handles(&file, &pool).await?;
        tracing::info!(
            handles = report.matched,
            "Instagram handles validated; no database changes"
        );
        return Ok(());
    }

    let report = osl_importer::social::load_instagram_handles(&file, &pool).await?;
    tracing::info!(handles = report.matched, "Instagram handles synchronized");

    Ok(())
}

fn import_privacy(path: &Path, skip: bool) -> Result<Option<PrivacyList>> {
    if skip {
        tracing::warn!("Removed-name validation skipped");
        return Ok(None);
    }
    let list = PrivacyList::load(path)?;
    list.require_key()?;
    Ok(Some(list))
}

#[allow(clippy::too_many_arguments)]
fn handle_redact(
    privacy_file: &Path,
    directory: &Path,
    instagram_file: &Path,
    name: &str,
    sex: Option<String>,
    country: Option<String>,
    disambiguation: Option<i16>,
    dry_run: bool,
) -> Result<()> {
    let mut list = PrivacyList::load(privacy_file)?;
    let query = AthleteQuery::new(name, sex, country, disambiguation);
    let plan = redact::plan(directory, instagram_file, &list, &query, name)?;

    tracing::info!(
        competitions = plan.competitions.len(),
        entries = plan.entries,
        handles_to_remove = plan.handles,
        "Redaction planned"
    );

    for competition in &plan.competitions {
        tracing::info!(
            competition = competition
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown"),
            "Competition redaction target selected"
        );
    }

    if dry_run {
        tracing::info!("Redaction dry run completed");
        return Ok(());
    }

    redact::apply(instagram_file, &mut list, &query, &plan)?;

    tracing::info!("Name-removal changes saved");

    Ok(())
}

fn handle_privacy(privacy_file: &Path, directory: &Path) -> Result<()> {
    let list = PrivacyList::load(privacy_file)?;

    let directories = competition_directories(&[directory.to_path_buf()])?;

    if list.is_empty() {
        tracing::info!(osl.file.kind = "privacy", "Name-removal list was empty");
        return Ok(());
    }

    list.require_key()?;

    let mut failures = 0;

    for competition in &directories {
        let canonical = store::read(competition)?;

        if privacy::check_competition(&canonical, &list).is_err() {
            tracing::error!(
                competition = competition.file_name().and_then(|name| name.to_str()).unwrap_or("unknown"),
                error.type = "privacy_validation",
                "Privacy check failed"
            );
            failures += 1;
        }
    }

    if failures > 0 {
        bail!(
            "{failures} competition(s) name someone on the privacy list. Run `osl-import redact` \
             for each, or write the stand-in the list already holds"
        );
    }

    tracing::info!(
        records = list.len(),
        competitions = directories.len(),
        "Privacy check passed"
    );

    Ok(())
}

async fn handle_recompute_ris(database_url: &str, dry_run: bool) -> Result<()> {
    tracing::debug!("RIS database connection requested");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("connecting to the database")?;

    if dry_run {
        let count = osl_db::services::ris_computation::scorable_participants(&pool, None)
            .await?
            .len();
        tracing::info!(participants = count, "RIS recomputation preview completed");
        return Ok(());
    }
    let count = osl_db::services::ris_computation::recompute_all_ris(&pool).await?;
    tracing::info!(participants = count, "RIS scores recomputed");

    Ok(())
}

fn handle_prepare(paths: &[PathBuf], check: bool, dry_run: bool) -> Result<()> {
    let directories = competition_directories(paths)?;

    let mut prepared = Vec::new();
    for directory in &directories {
        let mut canonical = store::read(directory)?;
        canonical_format::prepare(&mut canonical)
            .with_context(|| format!("Preparing {}", directory.display()))?;
        let (competition_text, entries_text) = store::render(&canonical)?;
        let metadata_matches =
            std::fs::read_to_string(directory.join(competition::FILE_NAME))? == competition_text;
        let entries_path = directory.join(entries::FILE_NAME);
        let entries_match = match entries_text {
            Some(text) => std::fs::read_to_string(&entries_path)? == text,
            None => !entries_path.exists(),
        };
        prepared.push((directory, canonical, !metadata_matches || !entries_match));
    }
    CanonicalValidator::validate_countries(prepared.iter().map(|(_, canonical, _)| canonical))?;
    let changed: Vec<_> = prepared
        .into_iter()
        .filter_map(|(directory, canonical, changed)| changed.then_some((directory, canonical)))
        .collect();
    if changed.is_empty() {
        tracing::info!(
            competitions = directories.len(),
            "Competition file preparation check passed"
        );
        return Ok(());
    }
    for (directory, _) in &changed {
        tracing::info!(
            competition = directory
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown"),
            "Competition preparation target selected"
        );
    }
    if check {
        bail!(
            "{} competition(s) need preparation. Run `osl-import prepare` to fix",
            changed.len()
        );
    }
    if dry_run {
        tracing::info!(
            competitions = changed.len(),
            "Competition preparation preview completed"
        );
    } else {
        for (directory, canonical) in &changed {
            store::write(directory, canonical)?;
        }
        tracing::info!(competitions = changed.len(), "Competition files prepared");
    }
    Ok(())
}

fn competition_directories(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut directories = Vec::new();
    for path in paths {
        anyhow::ensure!(path.is_dir(), "{} is not a directory", path.display());
        let count = directories.len();
        store::collect_competitions(path, &mut directories)?;
        anyhow::ensure!(
            directories.len() > count,
            "No competitions found in {}",
            path.display()
        );
    }
    let mut directories = directories
        .into_iter()
        .map(std::fs::canonicalize)
        .collect::<std::io::Result<Vec<_>>>()?;
    directories.sort();
    directories.dedup();
    Ok(directories)
}

fn validate_competition_slugs(directories: &[PathBuf]) -> Result<()> {
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
        return Ok(());
    }

    for (slug, directories) in &clashes {
        tracing::error!(
            competition = %slug,
            directories = directories.len(),
            "Competition slug uniqueness check failed"
        );
    }

    bail!(
        "{} competition slug(s) claimed by more than one directory. Merge them into one directory per competition",
        clashes.len()
    )
}

async fn handle_competitions(
    directories: &[PathBuf],
    dry_run: bool,
    prune: bool,
    database_url: Option<&str>,
    privacy: Option<&PrivacyList>,
) -> Result<()> {
    let competitions = competition_directories(directories)?;
    validate_competition_slugs(&competitions)?;
    let mut prepared = Vec::new();
    let mut failures = 0;
    for directory in &competitions {
        match read_competition(directory, privacy) {
            Ok(canonical) => prepared.push(canonical),
            Err(error) => {
                failures += 1;
                if std::env::var("LOG_FORMAT").as_deref() != Ok("json") {
                    eprintln!("{error:#}");
                }
                tracing::error!(
                    competition = directory.file_name().and_then(|name| name.to_str()).unwrap_or("unknown"),
                    error.type = "competition_validation",
                    "Competition validation failed"
                );
            }
        }
    }
    anyhow::ensure!(
        failures == 0,
        "{failures} competition(s) failed validation; nothing was imported"
    );
    CanonicalValidator::validate_countries(&prepared)?;
    if dry_run {
        tracing::info!(
            competitions = prepared.len(),
            "Competition files validated; no database changes"
        );
        if prune {
            tracing::info!("Dry-run pruning skipped");
        }
        return Ok(());
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.context("DATABASE_URL is required to import")?)
        .await
        .context("connecting to the database")?;
    let transformer = CanonicalTransformer::new(&pool);
    let total = prepared.len();
    let plan = transformer.import_batch(prepared, prune).await?;
    if prune {
        report_prune(&plan);
    }
    tracing::info!(competitions = total, "Competition import completed");
    Ok(())
}

fn report_prune(plan: &SyncPlan) {
    if plan.is_empty() {
        tracing::info!("Pruning candidate list was empty");
        return;
    }

    for competition in &plan.competitions {
        tracing::info!(competition = %competition.slug, name = %competition.name, "Competition absent from import input deleted");
    }

    for federation in &plan.federations {
        tracing::info!(federation = %federation, "Federation without competitions deleted");
    }

    tracing::info!(
        competitions = plan.competitions.len(),
        athletes = plan.athletes.len(),
        federations = plan.federations.len(),
        "Pruning completed"
    );
}

fn read_competition(
    directory: &Path,
    privacy: Option<&PrivacyList>,
) -> Result<osl_importer::canonical::models::CanonicalFormat> {
    let canonical = store::read(directory)?;
    store::check_location(directory, &canonical)?;
    if let Some(privacy) = privacy {
        privacy::check_competition(&canonical, privacy)?;
    }
    CanonicalValidator::validate(&canonical)?.log_warnings();
    Ok(canonical)
}
