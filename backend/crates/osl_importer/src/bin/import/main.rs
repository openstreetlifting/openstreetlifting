use anyhow::{Context, Result, bail};
use clap::Parser;
use osl_importer::canonical::{
    competition, entries, format as canonical_format, store, transformer::CanonicalTransformer,
    validator::CanonicalValidator,
};
use osl_importer::identity::AthleteQuery;
use osl_importer::privacy::{self, PrivacyList};
use osl_importer::redact;
use osl_importer::sync::CompetitionSync;
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
        Commands::Fmt {
            paths,
            check,
            dry_run,
        } => {
            handle_fmt(&paths, check, dry_run).await?;
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
            "{} handle(s) in {}. Set DATABASE_URL to also check that each name still names one \
             athlete",
            count,
            file.display()
        );
        return Ok(());
    };

    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("connecting to the database")?;

    if dry_run {
        let report = osl_importer::social::check_instagram_handles(&file, &pool).await?;
        tracing::info!("{} handle(s) name one athlete", report.matched);
        return Ok(());
    }

    let report = osl_importer::social::load_instagram_handles(&file, &pool).await?;
    tracing::info!("Attached {} handle(s)", report.matched);

    Ok(())
}

fn import_privacy(path: &Path, skip: bool) -> Result<Option<PrivacyList>> {
    if skip {
        tracing::warn!(
            "Privacy checks explicitly skipped; this validation does not approve publication"
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
        "'{}' ({}) becomes {} across {} competition(s), {} entry(ies)",
        plan.label,
        plan.identity.describe(),
        plan.redacted,
        plan.competitions.len(),
        plan.entries
    );

    for competition in &plan.competitions {
        tracing::info!("  {}", competition.display());
    }

    if plan.handles > 0 {
        tracing::info!("  {} Instagram handle(s) removed", plan.handles);
    }

    if dry_run {
        tracing::warn!("Nothing was written. Drop --dry-run to carry it out");
        return Ok(());
    }

    redact::apply(instagram_file, &mut list, &query, &plan)?;

    tracing::info!(
        "Redacted. Review the diff and commit it, then the next import takes the name off the site"
    );

    Ok(())
}

fn handle_privacy(privacy_file: &Path, directory: &Path) -> Result<()> {
    let list = PrivacyList::load(privacy_file)?;

    let directories = competition_directories(&[directory.to_path_buf()])?;

    if list.is_empty() {
        tracing::info!("{} lists nobody", list.path().display());
        return Ok(());
    }

    list.require_key()?;

    let mut failures = 0;

    for competition in &directories {
        let canonical = store::read(competition)?;

        if let Err(problem) = privacy::check_competition(&canonical, &list) {
            tracing::error!("{}: {problem}", competition.display());
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
        "{} redaction(s) hold across {} competition(s)",
        list.len(),
        directories.len()
    );

    Ok(())
}

async fn handle_recompute_ris(database_url: &str, dry_run: bool) -> Result<()> {
    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("connecting to the database")?;

    if dry_run {
        let count = osl_db::services::ris_computation::scorable_participants(&pool, None)
            .await?
            .len();
        tracing::info!("Would recompute RIS for {count} participant(s)");
        return Ok(());
    }
    let count = osl_db::services::ris_computation::recompute_all_ris(&pool).await?;
    tracing::info!("Recomputed RIS for {} participant(s)", count);

    Ok(())
}

async fn handle_fmt(paths: &[PathBuf], check: bool, dry_run: bool) -> Result<()> {
    let directories = competition_directories(paths)?;

    let mut changed = Vec::new();
    for directory in &directories {
        if is_formatted(directory)? {
            continue;
        }

        changed.push(directory.clone());
        if !check && !dry_run {
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
            "{} competition(s) are not formatted. Run `osl-import fmt` to fix",
            changed.len()
        );
    }

    if dry_run {
        tracing::info!("Would format {} competition(s)", changed.len());
    } else {
        tracing::info!("Formatted {} competition(s)", changed.len());
    }
    Ok(())
}

fn is_formatted(directory: &Path) -> Result<bool> {
    let mut canonical = store::read(directory)?;
    canonical_format::normalize(&mut canonical);
    let (competition_text, entries_text) = store::render(&canonical)?;

    if std::fs::read_to_string(directory.join(competition::FILE_NAME))? != competition_text {
        return Ok(false);
    }

    let entries_path = directory.join(entries::FILE_NAME);

    match entries_text {
        Some(entries_text) => Ok(std::fs::read_to_string(&entries_path)? == entries_text),
        None => Ok(!entries_path.exists()),
    }
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

async fn handle_competitions(
    directories: &[PathBuf],
    dry_run: bool,
    prune: bool,
    database_url: Option<&str>,
    privacy: Option<&PrivacyList>,
) -> Result<()> {
    let competitions = competition_directories(directories)?;
    let claimed_slugs = claimed_competition_slugs(&competitions)?;
    let mut prepared = Vec::new();
    let mut failures = 0;
    for directory in &competitions {
        match read_competition(directory, privacy) {
            Ok(canonical) => prepared.push(canonical),
            Err(error) => {
                failures += 1;
                tracing::error!("{}: {error}", directory.display());
            }
        }
    }
    anyhow::ensure!(
        failures == 0,
        "{failures} competition(s) failed validation; nothing was imported"
    );
    if dry_run {
        tracing::info!(
            "Validated {} competition(s); no database changes",
            prepared.len()
        );
        if prune {
            tracing::info!("Pruning would run after import; no deletion preview was calculated");
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
    for (index, canonical) in prepared.into_iter().enumerate() {
        tracing::info!(
            "[{}/{}] Importing {}",
            index + 1,
            total,
            canonical.competition.name
        );
        transformer.import_to_database(canonical).await?;
    }
    if prune {
        handle_prune(&pool, &claimed_slugs).await?;
    }
    tracing::info!("Imported {total} competition(s)");
    Ok(())
}

async fn handle_prune(pool: &sqlx::PgPool, claimed_slugs: &[String]) -> Result<()> {
    let sync = CompetitionSync::new(pool);

    let plan = sync.apply(claimed_slugs).await?;

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

    if !plan.federations.is_empty() {
        tracing::info!("Federations left without a competition:");
        for federation in &plan.federations {
            tracing::info!("  {}", federation);
        }
    }

    tracing::info!(
        "Deleted {} competition(s), {} athlete(s) and {} federation(s)",
        plan.competitions.len(),
        plan.athletes.len(),
        plan.federations.len()
    );

    Ok(())
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
