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
        help = "CSV file of name-removal records")]
    pub privacy_file: PathBuf,

    #[arg(
        short,
        long,
        global = true,
        help = "Enable debug logging (overridden by RUST_LOG)"
    )]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "Import competition files into PostgreSQL",
        after_help = "Examples:\n  osl-import competitions ./data/competitions --dry-run\n  osl-import competitions ./data/competitions --prune\n\nUse --prune only with the complete dataset: it deletes competitions absent from the supplied files.\nA dry run validates files without previewing deletions."
    )]
    Competitions {
        #[arg(
            default_value = "./data/competitions",
            help = "Competition directories to search recursively"
        )]
        paths: Vec<PathBuf>,
        #[arg(long, help = "Validate files without connecting to the database")]
        dry_run: bool,
        #[arg(
            long,
            help = "Delete competitions absent from the input and orphaned athletes and federations"
        )]
        prune: bool,
        #[arg(
            long,
            requires = "dry_run",
            help = "Skip checks for removed names (requires --dry-run)"
        )]
        skip_privacy_check: bool,
    },
    #[command(
        about = "Synchronize Instagram handles from a CSV file",
        after_help = "Dry runs also check that each row matches exactly one athlete when a database URL is set."
    )]
    Instagram {
        #[arg(
            default_value = "./data/athletes/instagram.csv",
            help = "Instagram CSV file"
        )]
        file: PathBuf,

        #[arg(long, help = "Check the CSV without changing the database")]
        dry_run: bool,
    },
    #[command(
        about = "Replace an athlete's name in files and remove their Instagram handle",
        after_help = "Examples:\n  osl-import redact --name \"Some Athlete\" --dry-run\n  osl-import redact --name \"Some Athlete\" --country FR"
    )]
    Redact {
        #[arg(long, value_parser = nonempty_name, help = "Athlete name as recorded in the results")]
        name: String,

        #[arg(long, value_name = "M|F|MX", help = "Match the athlete by sex")]
        sex: Option<Gender>,

        #[arg(
            long,
            value_name = "CODE",
            help = "Match the athlete by two-letter country code"
        )]
        country: Option<CountryCode>,

        #[arg(long, value_parser = clap::value_parser!(i16).range(1..),
            help = "Disambiguation number from entries.csv")]
        disambiguation: Option<i16>,

        #[arg(
            long,
            default_value = "./data/competitions",
            help = "Competition directory to search recursively"
        )]
        directory: PathBuf,

        #[arg(
            long,
            default_value = "./data/athletes/instagram.csv",
            help = "Instagram CSV file to remove the athlete's handle from"
        )]
        instagram_file: PathBuf,

        #[arg(long, help = "Preview the redaction without changing files")]
        dry_run: bool,
    },
    #[command(about = "Check competition files for names listed for removal")]
    Privacy {
        #[arg(
            long,
            default_value = "./data/competitions",
            help = "Competition directory to search recursively"
        )]
        directory: PathBuf,
    },
    #[command(about = "Recalculate stored RIS scores with the current formula")]
    RecomputeRis {
        #[arg(long, help = "Count eligible results without updating scores")]
        dry_run: bool,
    },
    #[command(about = "Format competition files")]
    Fmt {
        #[arg(
            default_value = "./data/competitions",
            help = "Competition directories to search recursively"
        )]
        paths: Vec<PathBuf>,

        #[arg(
            long,
            help = "Check formatting without changing files; exit nonzero if changes are needed"
        )]
        check: bool,

        #[arg(
            long,
            help = "List competition directories needing formatting without changing files"
        )]
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
