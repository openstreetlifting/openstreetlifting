use chrono::Datelike;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use osl_domain::{
    AthleteStatus, CountryCode, Gender, Movement, WeightClass, WeightClassSlug, event, slugify,
};
use rust_decimal::Decimal;

use super::entries::{self, Columns};
use super::meet::{self, CompetitionSection, MeetFile};
use super::models::{AthleteData, AttemptData, CanonicalFormat, CategoryData, LiftData};
use crate::{ImporterError, Result};

pub fn slug_of(directory: &Path) -> Result<String> {
    directory
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
        .ok_or_else(|| {
            ImporterError::ImportError(format!(
                "{} has no directory name to take a slug from",
                directory.display()
            ))
        })
}

/// The competition page links to its files at
/// data/competitions/{federation}/{year}/{slug}/, so a directory stored
/// anywhere else would publish a dead link. The path is part of the contract,
/// not a habit.
pub fn check_location(directory: &Path, canonical: &CanonicalFormat) -> Result<()> {
    let expected = expected_location(canonical);

    if directory.ends_with(&expected) {
        return Ok(());
    }

    Err(ImporterError::ValidationError(format!(
        "{} belongs in data/competitions/{}",
        directory.display(),
        expected.display()
    )))
}

fn expected_location(canonical: &CanonicalFormat) -> PathBuf {
    PathBuf::from(slugify(&canonical.competition.federation.name))
        .join(canonical.competition.start_date.year().to_string())
        .join(&canonical.competition.slug)
}

pub fn is_competition_directory(path: &Path) -> bool {
    path.is_dir() && path.join(meet::FILE_NAME).is_file()
}

pub fn read(directory: &Path) -> Result<CanonicalFormat> {
    let slug = slug_of(directory)?;
    let meet_path = directory.join(meet::FILE_NAME);

    let text = std::fs::read_to_string(&meet_path)
        .map_err(|e| ImporterError::ImportError(format!("{}: {e}", meet_path.display())))?;

    let meet: MeetFile = toml::from_str(&text)
        .map_err(|e| ImporterError::ValidationError(format!("{}: {e}", meet_path.display())))?;

    let movements = match meet.event.as_deref() {
        Some(event) => event::movements(event)
            .map_err(|e| ImporterError::ValidationError(format!("{}: {e}", meet_path.display())))?,
        None => Vec::new(),
    };

    let entries_path = directory.join(entries::FILE_NAME);
    let categories = if entries_path.is_file() {
        read_entries(&entries_path, &movements)?
    } else {
        Vec::new()
    };

    let MeetFile {
        sources,
        competition,
        federation,
        ..
    } = meet;

    Ok(CanonicalFormat {
        sources,
        competition: competition.into_data(slug, federation),
        movements,
        categories,
    })
}

pub fn render(canonical: &CanonicalFormat) -> Result<(String, Option<String>)> {
    let event = if canonical.movements.is_empty() {
        None
    } else {
        Some(canonical.movements.iter().map(|m| m.code()).collect())
    };

    let meet = MeetFile {
        event,
        sources: canonical.sources.clone(),
        competition: CompetitionSection::from_data(&canonical.competition),
        federation: canonical.competition.federation.clone(),
    };

    let rendered = toml::to_string_pretty(&meet)
        .map_err(|e| ImporterError::ImportError(format!("writing {}: {e}", meet::FILE_NAME)))?;

    let entries = if canonical.categories.is_empty() {
        None
    } else {
        Some(render_entries(canonical)?)
    };

    Ok((rendered, entries))
}

pub fn write(directory: &Path, canonical: &CanonicalFormat) -> Result<()> {
    let (meet_text, entries_text) = render(canonical)?;

    std::fs::write(directory.join(meet::FILE_NAME), meet_text)
        .map_err(|e| ImporterError::ImportError(format!("{}: {e}", directory.display())))?;

    if let Some(entries_text) = entries_text {
        std::fs::write(directory.join(entries::FILE_NAME), entries_text)
            .map_err(|e| ImporterError::ImportError(format!("{}: {e}", directory.display())))?;
    }

    Ok(())
}

fn read_entries(path: &Path, movements: &[Movement]) -> Result<Vec<CategoryData>> {
    let mut reader = csv::Reader::from_path(path)
        .map_err(|e| ImporterError::ImportError(format!("{}: {e}", path.display())))?;

    let header = reader
        .headers()
        .map_err(|e| ImporterError::ImportError(format!("{}: {e}", path.display())))?
        .clone();

    let columns = Columns::read(&header)
        .map_err(|e| ImporterError::ValidationError(format!("{}: {e}", path.display())))?;

    let mut categories: Vec<CategoryData> = Vec::new();

    for (offset, record) in reader.records().enumerate() {
        let line = offset + 2;
        let record = record.map_err(|e| {
            ImporterError::ValidationError(format!("{} line {line}: {e}", path.display()))
        })?;

        let (division, gender, bound, athlete) =
            read_entry(&columns, &record, movements).map_err(|e| {
                ImporterError::ValidationError(format!("{} line {line}: {e}", path.display()))
            })?;

        let (slug, min, max) = weight_class_bounds(gender, bound);

        let existing = categories.iter_mut().find(|c| {
            c.division == division
                && c.gender == gender
                && c.weight_class_slug == slug
                && c.weight_class_min == min
                && c.weight_class_max == max
        });

        if let Some(existing) = existing {
            existing.athletes.push(athlete);
            continue;
        }

        categories.push(CategoryData {
            division,
            gender,
            weight_class_slug: slug,
            weight_class_min: min,
            weight_class_max: max,
            athletes: vec![athlete],
        });
    }

    Ok(categories)
}

type Entry = (Option<String>, Gender, WeightClass, AthleteData);

fn read_entry(
    columns: &Columns,
    record: &csv::StringRecord,
    movements: &[Movement],
) -> std::result::Result<Entry, String> {
    let division = optional(columns, record, entries::DIVISION);
    let gender = Gender::from_str(columns.get(record, entries::SEX))?;

    let weight_class = columns.get(record, entries::WEIGHT_CLASS);
    if weight_class.is_empty() {
        return Err(format!("{} is empty", entries::WEIGHT_CLASS));
    }
    let weight_class = parse_weight_class(weight_class)?;

    let first_name = required(columns, record, entries::FIRST_NAME)?;
    let last_name = required(columns, record, entries::LAST_NAME)?;
    let country = CountryCode::parse(columns.get(record, entries::COUNTRY))?;

    let disambiguation = match optional(columns, record, entries::DISAMBIGUATION) {
        Some(raw) => Some(
            raw.parse::<i16>()
                .map_err(|_| format!("{} '{raw}' is not a number", entries::DISAMBIGUATION))?,
        ),
        None => None,
    };

    let bodyweight = entries::parse_decimal(columns.get(record, entries::BODYWEIGHT))
        .map_err(|e| format!("{}: {e}", entries::BODYWEIGHT))?;

    let ris = entries::parse_decimal(columns.get(record, entries::RIS))
        .map_err(|e| format!("{}: {e}", entries::RIS))?;

    let status = match optional(columns, record, entries::STATUS) {
        Some(raw) => AthleteStatus::from_str(&raw)?,
        None => AthleteStatus::Competed,
    };

    let athlete = AthleteData {
        first_name,
        last_name,
        disambiguation,
        gender: Some(gender),
        country,
        bodyweight,
        ris,
        status,
        status_reason: optional(columns, record, entries::STATUS_REASON),
        lifts: read_lifts(columns, record, movements)?,
    };

    Ok((division, gender, weight_class, athlete))
}

fn read_lifts(
    columns: &Columns,
    record: &csv::StringRecord,
    movements: &[Movement],
) -> std::result::Result<Vec<LiftData>, String> {
    let mut lifts = Vec::new();

    for movement in Movement::ALL {
        let mut attempts = Vec::new();

        for number in 1..=entries::ATTEMPTS_PER_MOVEMENT {
            let column = entries::attempt_column(movement, number);
            let attempt = entries::parse_attempt(columns.get(record, &column))
                .map_err(|e| format!("{column}: {e}"))?;

            if let Some(attempt) = attempt {
                attempts.push(AttemptData {
                    attempt_number: number,
                    weight: attempt.weight,
                    is_successful: attempt.is_successful,
                });
            }
        }

        let best_column = entries::best_column(movement);
        let best = entries::parse_decimal(columns.get(record, &best_column))
            .map_err(|e| format!("{best_column}: {e}"))?;

        if attempts.is_empty() && best.is_none() {
            continue;
        }

        if !movements.contains(&movement) {
            return Err(format!(
                "{} is not in the event, so its columns must be empty",
                movement.name()
            ));
        }

        let has_attempts = !attempts.is_empty();

        lifts.push(LiftData {
            movement,
            attempts: has_attempts.then_some(attempts),
            best_lift: (!has_attempts).then_some(best).flatten(),
        });
    }

    Ok(lifts)
}

fn render_entries(canonical: &CanonicalFormat) -> Result<String> {
    let mut writer = csv::Writer::from_writer(Vec::new());

    let divisioned = canonical
        .categories
        .iter()
        .any(|category| category.division.is_some());

    writer
        .write_record(entries::headers(divisioned))
        .map_err(|e| ImporterError::ImportError(format!("writing {}: {e}", entries::FILE_NAME)))?;

    for category in &canonical.categories {
        let weight_class = weight_class_cell(category);

        for athlete in &category.athletes {
            let mut row: Vec<String> = divisioned
                .then(|| category.division.clone().unwrap_or_default())
                .into_iter()
                .collect();

            row.extend([
                category.gender.as_str().to_string(),
                weight_class.clone(),
                athlete.first_name.clone(),
                athlete.last_name.clone(),
                athlete
                    .disambiguation
                    .map(|d| d.to_string())
                    .unwrap_or_default(),
                athlete.country.as_str().to_string(),
                entries::render_decimal(athlete.bodyweight),
                entries::render_decimal(athlete.ris),
                athlete.status.as_str().to_string(),
                athlete.status_reason.clone().unwrap_or_default(),
            ]);

            for movement in Movement::ALL {
                let lift = athlete.lifts.iter().find(|l| l.movement == movement);

                for number in 1..=entries::ATTEMPTS_PER_MOVEMENT {
                    let cell = lift
                        .and_then(|l| l.attempts.as_ref())
                        .and_then(|attempts| attempts.iter().find(|a| a.attempt_number == number))
                        .map(|a| {
                            entries::render_attempt(&entries::Attempt {
                                weight: a.weight,
                                is_successful: a.is_successful,
                            })
                        })
                        .unwrap_or_default();

                    row.push(cell);
                }

                row.push(entries::render_decimal(lift.and_then(best_of)));
            }

            writer.write_record(&row).map_err(|e| {
                ImporterError::ImportError(format!("writing {}: {e}", entries::FILE_NAME))
            })?;
        }
    }

    let bytes = writer
        .into_inner()
        .map_err(|e| ImporterError::ImportError(format!("writing {}: {e}", entries::FILE_NAME)))?;

    String::from_utf8(bytes)
        .map_err(|e| ImporterError::ImportError(format!("writing {}: {e}", entries::FILE_NAME)))
}

/// What the meet page shows for the movement, and what the importer stores as
/// `max_weight`. Derived whenever the attempts are known, so the column can
/// never contradict them.
fn best_of(lift: &LiftData) -> Option<Decimal> {
    match lift.attempts.as_ref() {
        Some(attempts) => attempts
            .iter()
            .filter(|a| a.is_successful)
            .map(|a| a.weight)
            .max(),
        None => lift.best_lift,
    }
}

fn required(
    columns: &Columns,
    record: &csv::StringRecord,
    column: &str,
) -> std::result::Result<String, String> {
    let value = columns.get(record, column);

    if value.is_empty() {
        return Err(format!("{column} is empty"));
    }

    Ok(value.to_string())
}

fn optional(columns: &Columns, record: &csv::StringRecord, column: &str) -> Option<String> {
    let value = columns.get(record, column);
    (!value.is_empty()).then(|| value.to_string())
}

fn parse_weight_class(cell: &str) -> std::result::Result<WeightClass, String> {
    let invalid = || format!("'{cell}' is not a weight class, expected 80 or 101+");

    match cell.strip_suffix('+') {
        Some(base) => base.parse().map(WeightClass::Above).map_err(|_| invalid()),
        None => cell.parse().map(WeightClass::UpTo).map_err(|_| invalid()),
    }
}

type Bounds = (Option<WeightClassSlug>, Option<Decimal>, Option<Decimal>);

fn weight_class_bounds(gender: Gender, bound: WeightClass) -> Bounds {
    let candidate = match bound {
        WeightClass::UpTo(max) => format!("{}-{}", gender.as_str(), max.normalize()),
        WeightClass::Above(min) => format!("{}+{}", gender.as_str(), min.normalize()),
    };

    if let Ok(slug) = WeightClassSlug::from_str(&candidate) {
        return (Some(slug), None, None);
    }

    match bound {
        WeightClass::UpTo(max) => (None, None, Some(max)),
        WeightClass::Above(min) => (None, Some(min), None),
    }
}

fn weight_class_cell(category: &CategoryData) -> String {
    let (min, max) = category.bounds();

    match WeightClass::of(min, max) {
        Some(WeightClass::UpTo(max)) => max.normalize().to_string(),
        Some(WeightClass::Above(min)) => format!("{}+", min.normalize()),
        None => String::new(),
    }
}
