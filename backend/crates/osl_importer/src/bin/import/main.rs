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
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
use cli::{Cli, Commands};

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
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .init();

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
            handle_prepare(&paths, check, dry_run).await?;
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
            file = %file.display(),
            "CSV validated without checking athlete matches. Set DATABASE_URL to check matches"
        );
        return Ok(());
    };

    tracing::debug!("Connecting to PostgreSQL");
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
        tracing::warn!(
            "Skipped checks for removed names. Run validation with OSL_PRIVACY_KEY before publication"
        );
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
        athlete = %plan.label,
        identity = %plan.identity.describe(),
        replacement = %plan.redacted,
        competitions = plan.competitions.len(),
        entries = plan.entries,
        handles_to_remove = plan.handles,
        "Redaction planned"
    );

    for competition in &plan.competitions {
        tracing::info!(directory = %competition.display(), "Redaction target");
    }

    if dry_run {
        tracing::info!("Dry run complete; no files changed. Rerun without --dry-run to apply");
        return Ok(());
    }

    redact::apply(instagram_file, &mut list, &query, &plan)?;

    tracing::info!(
        "Redaction saved. Review and commit the changes, then deploy and import the complete dataset with competitions --prune"
    );

    Ok(())
}

fn handle_privacy(privacy_file: &Path, directory: &Path) -> Result<()> {
    let list = PrivacyList::load(privacy_file)?;

    let directories = competition_directories(&[directory.to_path_buf()])?;

    if list.is_empty() {
        tracing::info!(file = %list.path().display(), "No name-removal records");
        return Ok(());
    }

    list.require_key()?;

    let mut failures = 0;

    for competition in &directories {
        let canonical = store::read(competition)?;

        if let Err(problem) = privacy::check_competition(&canonical, &list) {
            tracing::error!(
                directory = %competition.display(),
                error = %problem,
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
    tracing::debug!("Connecting to PostgreSQL");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("connecting to the database")?;

    if dry_run {
        let count = osl_db::services::ris_computation::scorable_participants(&pool, None)
            .await?
            .len();
        tracing::info!(
            participants = count,
            "RIS recomputation preview; no database changes"
        );
        return Ok(());
    }
    let count = osl_db::services::ris_computation::recompute_all_ris(&pool).await?;
    tracing::info!(participants = count, "RIS scores recomputed");

    Ok(())
}

async fn handle_prepare(paths: &[PathBuf], check: bool, dry_run: bool) -> Result<()> {
    let directories = competition_directories(paths)?;

    // Validate every directory before writing any changes.
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
            "Competition files already prepared"
        );
        return Ok(());
    }
    for (directory, _) in &changed {
        tracing::info!(directory = %directory.display(), "Preparation target");
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
            "Preparation preview; no files changed"
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
            "Competition slug appears in multiple directories"
        );
        for directory in directories.iter() {
            tracing::error!(competition = %slug, directory = %directory.display(), "Duplicate competition directory");
        }
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
                tracing::error!(
                    directory = %directory.display(),
                    error = %format_args!("{error:#}"),
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
            tracing::info!("Pruning skipped; dry runs do not preview deletions");
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
        tracing::info!("Nothing to prune");
        return;
    }

    if !plan.competitions.is_empty() {
        for competition in &plan.competitions {
            tracing::info!(competition = %competition.slug, name = %competition.name, "Deleted competition absent from the input");
        }
    }

    if !plan.athletes.is_empty() {
        for athlete in &plan.athletes {
            tracing::info!(athlete = %athlete, "Deleted athlete with no competition entries");
        }
    }

    if !plan.federations.is_empty() {
        for federation in &plan.federations {
            tracing::info!(federation = %federation, "Deleted federation with no competitions");
        }
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
