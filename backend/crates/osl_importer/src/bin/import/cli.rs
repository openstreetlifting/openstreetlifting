use std::path::PathBuf;

use clap::{Parser, Subcommand};
use osl_domain::{CountryCode, Gender};
use osl_importer::privacy;

#[derive(Parser)]
#[command(
    name = "osl-import",
    bin_name = "osl-import",
    version,
    about = "Import and maintain OpenStreetlifting competition data"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(
        long,
        global = true,
        env = "DATABASE_URL",
        hide_env_values = true,
        help = "PostgreSQL connection URL"
    )]
    pub database_url: Option<String>,

    #[arg(long, global = true, default_value = privacy::DEFAULT_PATH,
        help = "Suppression records used to prevent republishing removed names")]
    pub privacy_file: PathBuf,

    #[arg(short, long, global = true, help = "Enable debug logging")]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "Import competition directories or trees",
        after_help = "Examples:\n  osl-import competitions ./data/competitions --dry-run\n  osl-import competitions ./data/competitions --prune\n\nPruning requires the complete dataset. Dry runs validate files without writing to the database."
    )]
    Competitions {
        #[arg(
            default_value = "./data/competitions",
            help = "Competition directories or trees"
        )]
        paths: Vec<PathBuf>,
        #[arg(long, help = "Validate files without saving changes")]
        dry_run: bool,
        #[arg(
            long,
            help = "Remove competitions outside these paths and orphaned athletes and federations"
        )]
        prune: bool,
        #[arg(
            long,
            requires = "dry_run",
            help = "Skip secret-dependent suppression checks"
        )]
        skip_privacy_check: bool,
    },
    #[command(about = "Synchronize Instagram handles from a CSV file")]
    Instagram {
        #[arg(
            default_value = "./data/athletes/instagram.csv",
            help = "Instagram CSV file"
        )]
        file: PathBuf,

        #[arg(long, help = "Check handles without saving changes")]
        dry_run: bool,
    },
    #[command(
        about = "Replace an athlete's name and remove their Instagram handle",
        after_help = "Examples:\n  osl-import redact --name \"Some Athlete\" --dry-run\n  osl-import redact --name \"Some Athlete\" --country FR"
    )]
    Redact {
        #[arg(long, value_parser = nonempty_name, help = "Athlete name as recorded in the results")]
        name: String,

        #[arg(long, value_name = "M|F|MX", help = "Narrow the request by sex")]
        sex: Option<Gender>,

        #[arg(
            long,
            value_name = "CODE",
            help = "Narrow the request by two-letter country code"
        )]
        country: Option<CountryCode>,

        #[arg(long, value_parser = clap::value_parser!(i16).range(1..),
            help = "Distinguish athletes who share a name, sex and country")]
        disambiguation: Option<i16>,

        #[arg(
            long,
            default_value = "./data/competitions",
            help = "Competition directory or tree"
        )]
        directory: PathBuf,

        #[arg(
            long,
            default_value = "./data/athletes/instagram.csv",
            help = "Instagram CSV file"
        )]
        instagram_file: PathBuf,

        #[arg(long, help = "Preview the redaction without changing files")]
        dry_run: bool,
    },
    #[command(about = "Check that competition files contain no suppressed names")]
    Privacy {
        #[arg(
            long,
            default_value = "./data/competitions",
            help = "Competition directory or tree"
        )]
        directory: PathBuf,
    },
    #[command(about = "Recalculate stored RIS scores with the current formula")]
    RecomputeRis {
        #[arg(long, help = "Count eligible scores without updating them")]
        dry_run: bool,
    },
    #[command(about = "Format competition files")]
    Fmt {
        #[arg(
            default_value = "./data/competitions",
            help = "Competition directories or trees"
        )]
        paths: Vec<PathBuf>,

        #[arg(long, help = "Exit with an error if any file needs formatting")]
        check: bool,

        #[arg(long, help = "List files needing formatting without changing them")]
        dry_run: bool,
    },
}

fn nonempty_name(value: &str) -> std::result::Result<String, String> {
    let value = value.trim();
    if value.is_empty() {
        return Err("athlete name cannot be empty".into());
    }
    Ok(value.to_string())
}
