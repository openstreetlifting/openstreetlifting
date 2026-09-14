//! Taking an athlete's name off the archive without taking their results out.
//!
//! The canonical files are published, so a redaction that only reached the
//! database would leave the name in the repository and in the CSV downloads.
//! Nothing is written until exactly one athlete has been named.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use osl_domain::redaction::RedactedAthlete;

use crate::canonical::store;
use crate::canonical::{format as canonical_format, models::CanonicalFormat};
use crate::identity::{AthleteQuery, match_key};
use crate::privacy::{PrivacyEntry, PrivacyList};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileIdentity {
    pub gender: String,
    pub country: String,
    pub disambiguation: Option<i16>,
}

impl FileIdentity {
    pub fn describe(&self) -> String {
        match self.disambiguation {
            Some(number) => format!("{}/{} #{number}", self.gender, self.country),
            None => format!("{}/{}", self.gender, self.country),
        }
    }
}

#[derive(Debug)]
pub struct RedactionPlan {
    pub label: String,
    pub identity: FileIdentity,
    pub redacted: RedactedAthlete,
    pub hash: String,
    pub competitions: Vec<PathBuf>,
    pub entries: usize,
    pub handles: usize,
}

pub fn plan(
    tree: &Path,
    instagram: &Path,
    list: &PrivacyList,
    query: &AthleteQuery,
    label: &str,
) -> Result<RedactionPlan> {
    let Some(hash) = list.hash(label) else {
        bail!(
            "{} is not set, so the redaction could not be recorded and the next import would \
             undo it",
            crate::privacy::SALT_ENV
        );
    };

    let mut directories = Vec::new();
    store::collect_competitions(tree, &mut directories)?;
    directories.sort();

    let mut identities: BTreeSet<FileIdentity> = BTreeSet::new();
    let mut competitions = Vec::new();
    let mut entries = 0;

    for directory in &directories {
        let canonical = store::read(directory)?;
        let mut hits = 0;

        for category in &canonical.categories {
            for athlete in &category.athletes {
                if !query.matches_entry(athlete, category) {
                    continue;
                }

                identities.insert(FileIdentity {
                    gender: athlete
                        .gender
                        .unwrap_or(category.gender)
                        .as_str()
                        .to_string(),
                    country: athlete.country.as_str().to_string(),
                    disambiguation: athlete.disambiguation,
                });
                hits += 1;
            }
        }

        if hits > 0 {
            competitions.push(directory.clone());
            entries += hits;
        }
    }

    let identity = match identities.len() {
        0 => bail!(
            "No athlete named '{label}' in {}. Check the spelling against the entries.csv that \
             names them",
            tree.display()
        ),
        1 => identities.into_iter().next().expect("one identity"),
        _ => {
            let candidates: Vec<String> = identities.iter().map(FileIdentity::describe).collect();
            bail!(
                "'{label}' names {} athletes ({}). Narrow it with --sex, --country or \
                 --disambiguation so it names one",
                candidates.len(),
                candidates.join(", ")
            )
        }
    };

    let redacted = RedactedAthlete::new(list.next_id()).expect("next_id starts at 1");
    let handles = matching_handles(instagram, query, &identity)?.len();

    Ok(RedactionPlan {
        label: label.to_string(),
        identity,
        redacted,
        hash,
        competitions,
        entries,
        handles,
    })
}

pub fn apply(
    instagram: &Path,
    list: &mut PrivacyList,
    query: &AthleteQuery,
    plan: &RedactionPlan,
) -> Result<()> {
    for directory in &plan.competitions {
        let mut canonical = store::read(directory)?;
        redact_in_place(&mut canonical, query, plan.redacted);
        canonical_format::normalize(&mut canonical);
        store::write(directory, &canonical)?;
    }

    remove_handles(instagram, query, &plan.identity)?;

    list.append(PrivacyEntry {
        hash: plan.hash.clone(),
        query: AthleteQuery {
            match_key: String::new(),
            gender: Some(plan.identity.gender.clone()),
            country: Some(plan.identity.country.clone()),
            disambiguation: plan.identity.disambiguation,
        },
        redacted: plan.redacted,
    })
}

/// Sex, country, bodyweight, every attempt and the placing stay, so the
/// rankings do not move.
fn redact_in_place(
    canonical: &mut CanonicalFormat,
    query: &AthleteQuery,
    redacted: RedactedAthlete,
) {
    for category in &mut canonical.categories {
        let gender = category.gender;

        for athlete in &mut category.athletes {
            let matches = match_key(&athlete.display_name()) == query.match_key
                && query.matches_parts(
                    athlete.gender.unwrap_or(gender).as_str(),
                    athlete.country.as_str(),
                    athlete.disambiguation,
                );

            if !matches {
                continue;
            }

            athlete.first_name = String::new();
            athlete.last_name = redacted.last_name();
            athlete.native_name = None;
        }
    }
}

const HANDLE_HEADER: &[&str] = &["Name", "Sex", "Country", "Disambiguation", "Instagram"];

fn matching_handles(
    instagram: &Path,
    query: &AthleteQuery,
    identity: &FileIdentity,
) -> Result<Vec<usize>> {
    if !instagram.exists() {
        return Ok(Vec::new());
    }

    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(instagram)?;

    let mut matching = Vec::new();

    for (index, record) in reader.records().enumerate() {
        if handle_row_names(&record?, query, identity) {
            matching.push(index);
        }
    }

    Ok(matching)
}

fn handle_row_names(
    record: &csv::StringRecord,
    query: &AthleteQuery,
    identity: &FileIdentity,
) -> bool {
    let Some(name) = record.get(0) else {
        return false;
    };

    if match_key(name) != query.match_key {
        return false;
    }

    let field = |index: usize| record.get(index).map(str::trim).filter(|v| !v.is_empty());

    let row = AthleteQuery {
        match_key: query.match_key.clone(),
        gender: field(1).map(str::to_string),
        country: field(2).map(str::to_string),
        disambiguation: field(3).and_then(|value| value.parse().ok()),
    };

    row.matches_parts(&identity.gender, &identity.country, identity.disambiguation)
}

fn remove_handles(instagram: &Path, query: &AthleteQuery, identity: &FileIdentity) -> Result<()> {
    if !instagram.exists() {
        return Ok(());
    }

    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(instagram)?;

    let mut kept = Vec::new();
    let mut removed = 0;

    for record in reader.records() {
        let record = record?;

        if handle_row_names(&record, query, identity) {
            removed += 1;
            continue;
        }

        kept.push(record);
    }

    if removed == 0 {
        return Ok(());
    }

    let mut writer = csv::Writer::from_path(instagram)?;
    writer.write_record(HANDLE_HEADER)?;

    for record in kept {
        writer.write_record(&record)?;
    }

    writer.flush()?;
    Ok(())
}
