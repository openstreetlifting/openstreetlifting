//! Removes athlete names from published competition files while preserving results.
//!
//! The site and CSV downloads share these files. Redaction requires an
//! unambiguous athlete match.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use osl_domain::redaction::RedactedAthlete;

use crate::atomic_file::Replacement;
use crate::canonical::store;
use crate::canonical::{
    entries, format as canonical_format, models::CanonicalFormat, validator::CanonicalValidator,
};
use crate::identity::{AthleteQuery, match_key};
use crate::privacy::{PrivacyEntry, PrivacyList};
use crate::social;

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
            crate::privacy::KEY_ENV
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

    for entry in list.matching(query) {
        identities.insert(FileIdentity {
            gender: entry.query.gender.clone().unwrap_or_default(),
            country: entry.query.country.clone().unwrap_or_default(),
            disambiguation: entry.query.disambiguation,
        });
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

    let redacted = match list.lookup(
        label,
        &identity.gender,
        &identity.country,
        identity.disambiguation,
    ) {
        crate::privacy::Lookup::Listed(redacted) => redacted,
        _ => RedactedAthlete::new(list.next_id()?).expect("next_id starts at 1"),
    };
    let handles = handle_changes(instagram, query, &identity)?.map_or(0, |(count, _)| count);

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
    let mut replacements = Vec::new();
    for directory in &plan.competitions {
        let mut canonical = store::read(directory)?;
        redact_in_place(&mut canonical, query, plan.redacted);
        canonical_format::normalize(&mut canonical);
        CanonicalValidator::validate(&canonical)?;
        let (_, rendered) = store::render(&canonical)?;
        if let Some(rendered) = rendered {
            replacements.push(Replacement::prepare(
                &directory.join(entries::FILE_NAME),
                rendered.as_bytes(),
            )?);
        }
    }
    if let Some((_, rendered)) = handle_changes(instagram, query, &plan.identity)? {
        replacements.push(Replacement::prepare(instagram, &rendered)?);
    }

    // Record first: an interrupted rewrite must still block publication of the old name.
    list.append(PrivacyEntry {
        hash: plan.hash.clone(),
        query: AthleteQuery {
            match_key: String::new(),
            gender: Some(plan.identity.gender.clone()),
            country: Some(plan.identity.country.clone()),
            disambiguation: plan.identity.disambiguation,
        },
        redacted: plan.redacted,
    })?;
    for replacement in replacements {
        replacement.commit()?;
    }
    Ok(())
}

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

fn handle_changes(
    instagram: &Path,
    query: &AthleteQuery,
    identity: &FileIdentity,
) -> Result<Option<(usize, Vec<u8>)>> {
    if !instagram.exists() {
        return Ok(None);
    }
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(instagram)?;
    let headers = reader.headers()?.clone();
    let mut writer = csv::Writer::from_writer(Vec::new());
    writer.write_record(&headers)?;
    let mut removed = 0;
    for record in reader.records() {
        let record = record?;
        let row = social::parse_record(&record, &headers)?;
        if row.query.match_key == query.match_key
            && row
                .query
                .matches_parts(&identity.gender, &identity.country, identity.disambiguation)
        {
            removed += 1;
        } else {
            writer.write_record(&record)?;
        }
    }
    let rendered = writer.into_inner()?;
    Ok((removed > 0).then_some((removed, rendered)))
}
