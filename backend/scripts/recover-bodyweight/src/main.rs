//! One shot script that recover bodyweight of an athlete from a RIS.
//! This script is filling bodyweight in sources passed as parameter
//!
//! ```text
//! f  = Total * 100 / RIS
//! BW = v - ln( (K - f) / (Q * (f - A)) ) / B
//! ```
//! examples:
//! ```text
//! cargo run -p recover-bodyweight -- --edition 2024 --check data/competitions/finalrep
//! cargo run -p recover-bodyweight -- --edition 2026 data/competitions/sli/2026
//! ```
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use clap::Parser;
use osl_domain::{AthleteStatus, Constants, Edition, Gender, Movement};
use osl_importer::canonical::models::{BodyweightSource, CanonicalFormat};
use osl_importer::canonical::store;
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};

const CLASS_TOLERANCE: Decimal = Decimal::from_parts(5, 0, 0, false, 1);
const LIGHTEST_PLAUSIBLE: Decimal = Decimal::from_parts(35, 0, 0, false, 0);
const HEAVIEST_PLAUSIBLE: Decimal = Decimal::from_parts(200, 0, 0, false, 0);

const IMPLAUSIBLE_REFUSAL_RATE: f64 = 0.2;

#[derive(Parser)]
#[command(about = "Recover bodyweight from a published RIS score")]
struct Cli {
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    #[arg(long, value_name = "YEAR")]
    edition: i32,

    #[arg(long)]
    check: bool,
}

#[derive(Default)]
struct Outcome {
    recovered: usize,
    refused: Vec<String>,
    ineligible: BTreeMap<&'static str, usize>,
    cuts: Vec<Decimal>,
}

impl Outcome {
    fn considered(&self) -> usize {
        self.recovered + self.refused.len()
    }

    fn median_cut(&mut self) -> Option<Decimal> {
        if self.cuts.is_empty() {
            return None;
        }

        self.cuts.sort();
        let middle = self.cuts.len() / 2;
        if self.cuts.len().is_multiple_of(2) {
            Some((self.cuts[middle - 1] + self.cuts[middle]) / Decimal::TWO)
        } else {
            Some(self.cuts[middle])
        }
    }

    fn refusal_rate(&self) -> f64 {
        match self.considered() {
            0 => 0.0,
            considered => self.refused.len() as f64 / considered as f64,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let Some(edition) = Edition::from_year(cli.edition) else {
        bail!(
            "no RIS edition was published for {}. Known editions: {}",
            cli.edition,
            Edition::ALL
                .map(|edition| edition.year().to_string())
                .join(", ")
        );
    };

    verify_constants()?;

    let mut recovered = 0usize;
    let mut written = 0usize;
    let mut refused = Vec::new();
    let mut ineligible = 0usize;
    let mut withheld = 0usize;

    for directory in competition_directories(&cli.paths)? {
        let mut canonical = store::read(&directory)?;
        let mut outcome = recover(&mut canonical, edition);
        let slug = &canonical.competition.slug;
        for (reason, count) in &outcome.ineligible {
            println!("  ineligible {slug}: {count} athlete(s), {reason}");
            ineligible += count;
        }

        if outcome.considered() == 0 {
            println!("  nothing   {slug} has no eligible recovery candidates");
            continue;
        }

        if outcome.refusal_rate() > IMPLAUSIBLE_REFUSAL_RATE {
            println!(
                "  WITHHELD  {slug}, {} valid recovery/recoveries withheld: {} of {} candidates rejected (over 20%). Check the source data and edition",
                outcome.recovered,
                outcome.refused.len(),
                outcome.considered(),
            );
            withheld += outcome.recovered;
            refused.extend(outcome.refused);
            continue;
        }

        let cut = match outcome.median_cut() {
            Some(cut) => format!("median cut {cut:>5} kg under the class limit"),
            None => "no weight classes to compare against".to_string(),
        };
        println!(
            "  accepted  {slug}, {} recovery/recoveries, {} rejected, {cut}",
            outcome.recovered,
            outcome.refused.len()
        );

        refused.extend(outcome.refused);
        recovered += outcome.recovered;
        written += 1;

        if !cli.check {
            note_provenance(&mut canonical, edition);
            store::write(&directory, &canonical)?;
        }
    }

    for refusal in &refused {
        println!("  rejected  {refusal}");
    }

    let verb = if cli.check {
        "would recover"
    } else {
        "recovered"
    };
    println!(
        "\n{verb} {recovered} bodyweight(s) across {written} competition(s) \
         using the {} edition; {} rejected, {withheld} withheld, {ineligible} ineligible",
        edition.year(),
        refused.len()
    );

    Ok(())
}

fn verify_constants() -> Result<()> {
    for (gender, bodyweight, total, published) in WORLDS_2023 {
        let gender = if gender == "F" { Gender::F } else { Gender::M };
        let computed =
            osl_domain::ris::compute(dec(bodyweight), dec(total), gender, Edition::V2024);

        if computed != dec(published) {
            bail!(
                "the 2024 constants score {bodyweight}kg / {total} as {computed}, \
                 the presentation publishes {published}. Refusing to write anything."
            );
        }
    }

    Ok(())
}

fn recover(canonical: &mut CanonicalFormat, edition: Edition) -> Outcome {
    let mut outcome = Outcome::default();

    if canonical.movements != Movement::ALL {
        outcome.ineligible.insert(
            "event does not contain all four movements",
            canonical.categories.iter().map(|c| c.athletes.len()).sum(),
        );
        return outcome;
    }

    let slug = &canonical.competition.slug;

    for category in &mut canonical.categories {
        let bounds = category.bounds();
        let label = category.label();
        let category_gender = category.gender;

        for athlete in &mut category.athletes {
            let ineligible = if athlete.bodyweight.is_some() {
                Some("bodyweight already present")
            } else if athlete.status != AthleteStatus::Competed {
                Some("status is not competed")
            } else {
                None
            };
            if let Some(reason) = ineligible {
                *outcome.ineligible.entry(reason).or_default() += 1;
                continue;
            }
            let Some(ris) = athlete.ris else {
                *outcome.ineligible.entry("no published RIS").or_default() += 1;
                continue;
            };

            let gender = athlete.gender.unwrap_or(category_gender);
            let bodyweight = athlete.complete_total().and_then(|total| {
                if athlete
                    .reported_ris_edition
                    .is_some_and(|reported| reported != edition)
                {
                    return Err("requested edition differs from ReportedRisEdition".into());
                }
                let bodyweight = solve_bodyweight(ris, total, edition.constants(gender))?;
                validate_bodyweight(bodyweight, bounds, &label)?;
                if osl_domain::ris::compute(bodyweight, total, gender, edition) != ris {
                    return Err("rounded bodyweight does not reproduce the published RIS".into());
                }
                Ok(bodyweight)
            });
            match bodyweight {
                Err(reason) => {
                    outcome
                        .refused
                        .push(format!("{slug}: {}, {reason}", athlete.display_name()));
                }
                Ok(bodyweight) => {
                    if let (_, Some(limit)) = bounds {
                        outcome.cuts.push(limit - bodyweight);
                    }

                    athlete.bodyweight = Some(bodyweight);
                    athlete.bodyweight_source = Some(BodyweightSource::Recovered);
                    athlete.reported_ris_edition = Some(edition);
                    outcome.recovered += 1;
                }
            }
        }
    }

    outcome
}

fn solve_bodyweight(ris: Decimal, total: Decimal, c: Constants) -> Result<Decimal, String> {
    let ris = ris.to_f64().unwrap_or_default();
    let total = total.to_f64().unwrap_or_default();

    if ris <= 0.0 || total <= 0.0 {
        return Err("a zero score or total leaves no equation to solve".into());
    }

    let benchmark = total * 100.0 / ris;
    if benchmark <= c.a {
        return Err("the score sits below the curve".into());
    }
    if benchmark >= c.k {
        return Err(
            "the score sits where the curve has flattened, so no bodyweight solves it".into(),
        );
    }

    let ratio = (c.k - benchmark) / (c.q * (benchmark - c.a));
    let bodyweight = c.v - ratio.ln() / c.b;

    Decimal::from_f64_retain(bodyweight)
        .map(|bodyweight| bodyweight.round_dp(2))
        .ok_or_else(|| "the recovered bodyweight is not a number".into())
}

fn validate_bodyweight(
    bodyweight: Decimal,
    bounds: (Option<Decimal>, Option<Decimal>),
    category: &str,
) -> Result<Decimal, String> {
    if !(LIGHTEST_PLAUSIBLE..=HEAVIEST_PLAUSIBLE).contains(&bodyweight) {
        return Err(format!("{bodyweight} kg is not a plausible bodyweight"));
    }

    let (min, max) = bounds;
    if max.is_some_and(|max| bodyweight > max + CLASS_TOLERANCE)
        || min.is_some_and(|min| bodyweight <= min - CLASS_TOLERANCE)
    {
        return Err(format!("{bodyweight} kg is outside {category}"));
    }

    Ok(bodyweight)
}

fn note_provenance(canonical: &mut CanonicalFormat, edition: Edition) {
    let note = format!(
        "Bodyweight recovered by reversing the {} RIS formula on the published score",
        edition.year()
    );

    if !canonical.sources.contains(&note) {
        canonical.sources.push(note);
    }
}

fn competition_directories(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut directories = Vec::new();

    for path in paths {
        if !path.exists() {
            bail!("{} does not exist", path.display());
        }

        collect(path, &mut directories)?;
    }

    directories.sort();
    directories.dedup();

    if directories.is_empty() {
        bail!("no competition directories found under the given paths");
    }

    Ok(directories)
}

fn collect(path: &Path, found: &mut Vec<PathBuf>) -> Result<()> {
    if store::is_competition_directory(path) {
        found.push(path.to_path_buf());
        return Ok(());
    }

    for entry in std::fs::read_dir(path)? {
        let entry = entry?.path();
        if entry.is_dir() {
            collect(&entry, found)?;
        }
    }

    Ok(())
}

fn dec(value: f64) -> Decimal {
    Decimal::from_f64(value).expect("a reference constant is a decimal")
}

const WORLDS_2023: [(&str, f64, f64, f64); 24] = [
    ("M", 91.7, 586.5, 113.43),
    ("M", 85.6, 557.25, 110.79),
    ("M", 73.0, 494.0, 109.90),
    ("M", 93.0, 570.0, 109.77),
    ("M", 66.0, 442.75, 108.08),
    ("M", 106.9, 572.5, 107.63),
    ("M", 98.0, 562.5, 106.99),
    ("M", 79.6, 513.75, 106.65),
    ("M", 86.3, 532.5, 105.46),
    ("M", 113.2, 562.5, 105.34),
    ("M", 87.1, 532.75, 105.06),
    ("M", 72.7, 470.0, 104.94),
    ("M", 72.5, 467.5, 104.64),
    ("M", 93.9, 541.5, 104.00),
    ("M", 93.0, 540.0, 103.99),
    ("F", 62.3, 257.5, 108.28),
    ("F", 56.0, 245.0, 108.09),
    ("F", 67.6, 260.0, 108.05),
    ("F", 53.7, 230.5, 105.08),
    ("F", 56.6, 237.5, 104.03),
    ("F", 65.9, 248.75, 103.62),
    ("F", 62.2, 245.0, 103.07),
    ("F", 54.7, 228.75, 102.70),
    ("F", 62.7, 242.5, 101.82),
];
