//! The list of athletes who asked to be taken off the site.
//!
//! Names are keyed by HMAC under a salt kept outside the repository: a public
//! file naming everyone who asked to be forgotten publishes exactly what it was
//! built to remove. Without the salt the list cannot answer, and says so rather
//! than passing quietly.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, bail};
use hmac::{Hmac, Mac};
use osl_domain::redaction::RedactedAthlete;
use osl_domain::{CountryCode, Gender};
use sha2::Sha256;

use crate::canonical::models::CanonicalFormat;
use crate::identity::{AthleteQuery, match_key};

pub const DEFAULT_PATH: &str = "./data/athletes/privacy.csv";
pub const SALT_ENV: &str = "OSL_PRIVACY_SALT";

/// Numbers from here up belong to the staging fixture, so a real redaction and
/// the seeded one never land on the same stand-in.
pub const STAGING_ID_FLOOR: u32 = 9000;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct PrivacyRecord {
    #[serde(rename = "Hash")]
    hash: String,
    #[serde(rename = "Sex")]
    sex: Option<String>,
    #[serde(rename = "Country")]
    country: Option<String>,
    #[serde(rename = "Disambiguation")]
    disambiguation: Option<i16>,
    #[serde(rename = "RedactedId")]
    redacted_id: u32,
}

#[derive(Debug)]
pub struct PrivacyEntry {
    pub hash: String,
    pub query: AthleteQuery,
    pub redacted: RedactedAthlete,
}

#[derive(Debug)]
pub struct PrivacyList {
    path: PathBuf,
    salt: Option<String>,
    entries: Vec<PrivacyEntry>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Lookup {
    Unsalted,
    NotListed,
    Listed(RedactedAthlete),
}

impl PrivacyList {
    pub fn load(path: &Path) -> Result<Self> {
        Self::load_with_salt(path, std::env::var(SALT_ENV).ok().filter(|s| !s.is_empty()))
    }

    pub fn load_with_salt(path: &Path, salt: Option<String>) -> Result<Self> {
        if !path.exists() {
            return Ok(Self {
                path: path.to_path_buf(),
                salt,
                entries: Vec::new(),
            });
        }

        let mut reader = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_path(path)
            .with_context(|| format!("reading {}", path.display()))?;

        let mut entries = Vec::new();
        let mut seen_ids = Vec::new();

        for (index, record) in reader.deserialize::<PrivacyRecord>().enumerate() {
            let line = index + 2;
            let record = record.with_context(|| format!("{}: line {line}", path.display()))?;
            let entry = parse(record)
                .map_err(|problem| anyhow!("{}: line {line}: {problem}", path.display()))?;

            if seen_ids.contains(&entry.redacted.id()) {
                bail!(
                    "{}: line {line}: RedactedId {} is used twice, so two athletes would share \
                     a page",
                    path.display(),
                    entry.redacted.id()
                );
            }

            seen_ids.push(entry.redacted.id());
            entries.push(entry);
        }

        Ok(Self {
            path: path.to_path_buf(),
            salt,
            entries,
        })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn is_salted(&self) -> bool {
        self.salt.is_some()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn next_id(&self) -> u32 {
        self.entries
            .iter()
            .map(|entry| entry.redacted.id())
            .filter(|id| *id < STAGING_ID_FLOOR)
            .max()
            .unwrap_or(0)
            + 1
    }

    pub fn hash(&self, full_name: &str) -> Option<String> {
        self.salt
            .as_deref()
            .map(|salt| hash_with(salt, &match_key(full_name)))
    }

    pub fn lookup(
        &self,
        full_name: &str,
        gender: &str,
        country: &str,
        disambiguation: Option<i16>,
    ) -> Lookup {
        let Some(hash) = self.hash(full_name) else {
            return Lookup::Unsalted;
        };

        self.entries
            .iter()
            .find(|entry| {
                entry.hash == hash && entry.query.matches_parts(gender, country, disambiguation)
            })
            .map_or(Lookup::NotListed, |entry| Lookup::Listed(entry.redacted))
    }

    pub fn append(&mut self, entry: PrivacyEntry) -> Result<()> {
        self.entries.push(entry);
        self.entries.sort_by_key(|entry| entry.redacted.id());
        self.write()
    }

    fn write(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut writer = csv::Writer::from_path(&self.path)
            .with_context(|| format!("writing {}", self.path.display()))?;

        for entry in &self.entries {
            writer.serialize(PrivacyRecord {
                hash: entry.hash.clone(),
                sex: entry.query.gender.clone(),
                country: entry.query.country.clone(),
                disambiguation: entry.query.disambiguation,
                redacted_id: entry.redacted.id(),
            })?;
        }

        writer.flush()?;
        Ok(())
    }
}

fn parse(record: PrivacyRecord) -> std::result::Result<PrivacyEntry, String> {
    let hash = record.hash.trim().to_lowercase();

    if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!(
            "'{}' is not a hash. It is the 64 hex characters `osl-import redact` writes",
            record.hash
        ));
    }

    let gender = optional(record.sex.as_deref())
        .map(|sex| {
            sex.parse::<Gender>()
                .map(|gender| gender.as_str().to_string())
                .map_err(|problem| format!("unreadable Sex: {problem}"))
        })
        .transpose()?;

    let country = optional(record.country.as_deref())
        .map(|country| {
            CountryCode::parse(country)
                .map(|country| country.as_str().to_string())
                .map_err(|problem| format!("unreadable Country: {problem}"))
        })
        .transpose()?;

    if let Some(number) = record.disambiguation
        && number < 1
    {
        return Err(format!(
            "disambiguation {number}. It numbers the people who share a name, starting at 1"
        ));
    }

    let redacted = RedactedAthlete::new(record.redacted_id)
        .ok_or_else(|| "RedactedId starts at 1".to_string())?;

    Ok(PrivacyEntry {
        hash,
        query: AthleteQuery {
            match_key: String::new(),
            gender,
            country,
            disambiguation: record.disambiguation,
        },
        redacted,
    })
}

fn optional(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

pub fn hash_with(salt: &str, match_key: &str) -> String {
    let mut mac =
        Hmac::<Sha256>::new_from_slice(salt.as_bytes()).expect("HMAC takes a key of any length");
    mac.update(match_key.as_bytes());

    mac.finalize()
        .into_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Keeps the next import from undoing a redaction the files already carry.
pub fn check_competition(canonical: &CanonicalFormat, list: &PrivacyList) -> Result<()> {
    if list.is_empty() {
        return Ok(());
    }

    let mut found = Vec::new();

    for category in &canonical.categories {
        for athlete in &category.athletes {
            let gender = athlete.gender.unwrap_or(category.gender);

            if let Lookup::Listed(redacted) = list.lookup(
                &athlete.display_name(),
                gender.as_str(),
                athlete.country.as_str(),
                athlete.disambiguation,
            ) {
                found.push(redacted);
            }
        }
    }

    if found.is_empty() {
        return Ok(());
    }

    bail!(
        "{} entry(ies) name an athlete who asked to be taken off the site. Write them as {} \
         instead",
        found.len(),
        found
            .iter()
            .map(RedactedAthlete::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_hash_hides_the_name() {
        let hashed = hash_with("salt", &match_key("Alina Riyaz"));

        assert_eq!(hashed.len(), 64);
        assert!(!hashed.contains("alina"));
        assert!(hashed.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn the_same_athlete_spelled_differently_hashes_the_same() {
        assert_eq!(
            hash_with("salt", &match_key("Léa Mérandon")),
            hash_with("salt", &match_key("LEA MERANDON"))
        );
    }

    #[test]
    fn a_different_salt_gives_a_different_hash() {
        assert_ne!(
            hash_with("one", &match_key("Alina Riyaz")),
            hash_with("two", &match_key("Alina Riyaz"))
        );
    }

    #[test]
    fn different_people_do_not_collide() {
        assert_ne!(
            hash_with("salt", &match_key("Tom Berthier")),
            hash_with("salt", &match_key("Tom Bertier"))
        );
    }
}
