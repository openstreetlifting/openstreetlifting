//! Recover the bodyweight a published RIS score was computed against.
//!
//! Some scoring software publishes a RIS but not the bodyweight behind it.
//! Once the constants are fixed, `RIS = Total * 100 / f(BW)` is a plain
//! deterministic function and `f` is strictly monotonic, so a total and a
//! score pin down exactly one bodyweight:
//!
//! ```text
//! f  = Total * 100 / RIS
//! BW = v - ln( (K - f) / (Q * (f - A)) ) / B
//! ```
//!
//! Only one thing varies between sources: which edition scored the meet. So
//! that is the argument, and the caller supplies it. Nothing here knows or
//! cares which platform produced the number.
//!
//! This is an authoring step, not a runtime behaviour. Once `BodyweightKg` is
//! in the file the importer scores the athlete from it like any other row, and
//! the recovered value never needs computing again, not even when a new
//! edition lands. That is why it lives here rather than in the importer.
//!
//! ```text
//! cargo run -p recover-bodyweight -- --edition 2024 --check data/competitions/finalrep
//! cargo run -p recover-bodyweight -- --edition 2026 data/competitions/sli/2026
//! ```

use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use clap::Parser;
use osl_domain::{AthleteStatus, Constants, Edition, Gender, Movement};
use osl_importer::canonical::models::CanonicalFormat;
use osl_importer::canonical::store;
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};

/// An athlete may weigh in a little over and still be allowed to lift, and the
/// score is published to two decimals, so a recovered value is allowed to sit
/// just outside its class rather than being refused for it.
const CLASS_TOLERANCE: Decimal = Decimal::from_parts(5, 0, 0, false, 1);
const LIGHTEST_PLAUSIBLE: Decimal = Decimal::from_parts(35, 0, 0, false, 0);
const HEAVIEST_PLAUSIBLE: Decimal = Decimal::from_parts(200, 0, 0, false, 0);

/// Inverting with the wrong edition puts most of a field outside its own
/// weight class, while a right one refuses about one row in a hundred. Past
/// this share the edition is the likely fault, not the data, so the
/// competition is left alone.
const IMPLAUSIBLE_REFUSAL_RATE: f64 = 0.2;

/// Below this many candidate rows the refusal rate carries no signal, so the
/// edition cannot be checked against the field and is taken on trust alone.
/// That is how a handful of rows left behind by one edition's run get written
/// by the next one, so they are left for a run that targets them directly.
const MIN_ROWS_TO_JUDGE_AN_EDITION: usize = 5;

#[derive(Parser)]
#[command(about = "Recover bodyweight from a published RIS score")]
struct Cli {
    /// Competition directories, or a tree to search for them.
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// The published RIS edition that scored these competitions. Nothing is
    /// inferred: a source that changed edition mid-archive needs one run per
    /// edition.
    #[arg(long, value_name = "YEAR")]
    edition: i32,

    /// Report what would be recovered without writing.
    #[arg(long)]
    check: bool,
}

#[derive(Default)]
struct Outcome {
    recovered: usize,
    refused: Vec<String>,
    /// How far under its class limit each recovered value landed. Athletes cut
    /// to just under the limit, so a healthy competition sits around a kilo or
    /// two and anything far off wants a second look.
    cuts: Vec<Decimal>,
}

impl Outcome {
    fn considered(&self) -> usize {
        self.recovered + self.refused.len()
    }

    fn median_cut(&self) -> Option<Decimal> {
        if self.cuts.is_empty() {
            return None;
        }

        let mut cuts = self.cuts.clone();
        cuts.sort();
        Some(cuts[cuts.len() / 2])
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

    for directory in competition_directories(&cli.paths)? {
        let mut canonical = store::read(&directory)?;
        let slug = canonical.competition.slug.clone();

        let outcome = recover(&mut canonical, edition);

        if outcome.considered() == 0 {
            println!("  nothing   {slug} has no published score to reverse");
            continue;
        }

        if outcome.considered() < MIN_ROWS_TO_JUDGE_AN_EDITION {
            println!(
                "  SKIPPED   {slug}, only {} row(s) to reverse, too few to tell whether the {} \
                 edition scored it. Target it directly if you are sure",
                outcome.considered(),
                edition.year()
            );
            refused.extend(outcome.refused);
            continue;
        }

        if outcome.refusal_rate() > IMPLAUSIBLE_REFUSAL_RATE {
            println!(
                "  SKIPPED   {slug}, {} of {} rows refused. The {} edition probably did not score it",
                outcome.refused.len(),
                outcome.considered(),
                edition.year()
            );
            refused.extend(outcome.refused);
            continue;
        }

        if outcome.recovered > 0 {
            let cut = match outcome.median_cut() {
                Some(cut) => format!("median cut {cut:>5} kg under the class limit"),
                None => "no weight classes to compare against".to_string(),
            };
            println!(
                "  ok        {slug}, {} recovered, {} refused, {cut}",
                outcome.recovered,
                outcome.refused.len()
            );
        }

        refused.extend(outcome.refused);
        recovered += outcome.recovered;

        if outcome.recovered > 0 {
            written += 1;

            if !cli.check {
                note_provenance(&mut canonical, edition);
                store::write(&directory, &canonical)?;
            }
        }
    }

    for refusal in &refused {
        println!("  refused   {refusal}");
    }

    let verb = if cli.check { "would recover" } else { "recovered" };
    println!(
        "\n{verb} {recovered} bodyweight(s) across {written} competition(s) \
         using the {} edition, {} refused",
        edition.year(),
        refused.len()
    );

    Ok(())
}

/// A constants table with two of its rows transposed still looks plausible,
/// and the RIS presentation has shipped exactly that error. Nothing runs until
/// the published Worlds 2023 scores reproduce.
fn verify_constants() -> Result<()> {
    for (gender, bodyweight, total, published) in WORLDS_2023 {
        let gender = if gender == "F" { Gender::F } else { Gender::M };
        let computed = osl_domain::ris::compute(dec(bodyweight), dec(total), gender, Edition::V2024);

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

    // The formula divides by a benchmark fitted to four-lift totals, so a
    // shorter event recovers nothing.
    if canonical.movements != Movement::ALL {
        return outcome;
    }

    let slug = canonical.competition.slug.clone();

    for category in &mut canonical.categories {
        let bounds = category.bounds();
        let label = category.label();
        let category_gender = category.gender;

        for athlete in &mut category.athletes {
            if athlete.bodyweight.is_some() || athlete.status != AthleteStatus::Competed {
                continue;
            }
            let Some(ris) = athlete.ris else {
                continue;
            };

            let name = athlete.display_name();
            let gender = athlete.gender.unwrap_or(category_gender);
            let total: Decimal = athlete.lifts.iter().filter_map(|lift| lift.best()).sum();

            match solve_bodyweight(ris, total, edition.constants(gender)) {
                Err(reason) => outcome.refused.push(format!("{slug}: {name}, {reason}")),
                Ok(bodyweight) => match refuse(bodyweight, bounds, &label) {
                    Some(reason) => outcome.refused.push(format!("{slug}: {name}, {reason}")),
                    None => {
                        if let (_, Some(limit)) = bounds {
                            outcome.cuts.push(limit - bodyweight);
                        }

                        // The canonical format takes a bodyweight or a reported
                        // score, never both, because a bodyweight means we
                        // compute the score ourselves. The reported one is not
                        // lost: the bodyweight, the total and the edition named
                        // in `sources` reproduce it exactly.
                        athlete.bodyweight = Some(bodyweight);
                        athlete.ris = None;
                        outcome.recovered += 1;
                    }
                },
            }
        }
    }

    outcome
}

/// Above the flat top of the curve many bodyweights give the same score, so
/// the answer is gone rather than imprecise.
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
        return Err("the score sits where the curve has flattened, so no bodyweight solves it".into());
    }

    let ratio = (c.k - benchmark) / (c.q * (benchmark - c.a));
    let bodyweight = c.v - ratio.ln() / c.b;

    Decimal::from_f64_retain(bodyweight)
        .map(|bodyweight| bodyweight.round_dp(2))
        .ok_or_else(|| "the recovered bodyweight is not a number".into())
}

/// A recovered bodyweight outside the class the athlete contested is wrong
/// whatever the arithmetic says. A competition without weight classes has
/// nothing to check against, so only the plausible range applies.
fn refuse(
    bodyweight: Decimal,
    bounds: (Option<Decimal>, Option<Decimal>),
    category: &str,
) -> Option<String> {
    if !(LIGHTEST_PLAUSIBLE..=HEAVIEST_PLAUSIBLE).contains(&bodyweight) {
        return Some(format!("{bodyweight} kg is not a plausible bodyweight"));
    }

    let (min, max) = bounds;
    if max.is_some_and(|max| bodyweight > max + CLASS_TOLERANCE)
        || min.is_some_and(|min| bodyweight <= min - CLASS_TOLERANCE)
    {
        return Some(format!("{bodyweight} kg is outside {category}"));
    }

    None
}

/// Which edition produced the value, so a reader can check the arithmetic.
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

/// Bodyweight, total and score as the RIS presentation publishes them for
/// Final Rep Worlds 2023, which the 2024 edition scored.
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
    // The presentation prints 532.7, the only total in its table that is not a
    // multiple of 0.25. At 532.75 the published score reproduces.
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
