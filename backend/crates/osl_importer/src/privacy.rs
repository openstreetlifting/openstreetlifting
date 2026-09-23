//! Suppression records for names removed from the published archive.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, bail};
use hmac::{Hmac, KeyInit, Mac};
use osl_domain::redaction::RedactedAthlete;
use osl_domain::{CountryCode, Gender};
use sha2::Sha256;

use crate::canonical::models::CanonicalFormat;
use crate::identity::{AthleteQuery, match_key};

pub const DEFAULT_PATH: &str = "./data/athletes/privacy.csv";
pub const KEY_ENV: &str = "OSL_PRIVACY_KEY";
const KEY_CHECK_MESSAGE: &str = "openstreetlifting/privacy-key-check/v1";

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
    #[serde(rename = "KeyCheck")]
    key_check: String,
}

#[derive(Debug, Clone)]
pub struct PrivacyEntry {
    pub hash: String,
    pub query: AthleteQuery,
    pub redacted: RedactedAthlete,
}

#[derive(Debug)]
pub struct PrivacyList {
    path: PathBuf,
    key: Option<String>,
    entries: Vec<PrivacyEntry>,
    by_hash: HashMap<String, Vec<usize>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Lookup {
    MissingKey,
    NotListed,
    Listed(RedactedAthlete),
}

impl PrivacyList {
    pub fn load(path: &Path) -> Result<Self> {
        Self::load_with_key(path, std::env::var(KEY_ENV).ok().filter(|s| !s.is_empty()))
    }

    pub fn load_with_key(path: &Path, key: Option<String>) -> Result<Self> {
        let key = key.filter(|key| !key.is_empty());
        if !path.exists() {
            return Ok(Self {
                path: path.to_path_buf(),
                key,
                entries: Vec::new(),
                by_hash: HashMap::new(),
            });
        }

        let mut reader = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_path(path)
            .with_context(|| format!("reading {}", path.display()))?;

        let mut entries = Vec::new();
        let mut seen_ids = HashSet::new();
        let mut by_hash: HashMap<String, Vec<usize>> = HashMap::new();
        let mut identities = HashSet::new();

        for (index, record) in reader.deserialize::<PrivacyRecord>().enumerate() {
            let line = index + 2;
            let record = record.with_context(|| format!("{}: line {line}", path.display()))?;
            if let Some(key) = &key {
                let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes())
                    .expect("HMAC accepts any key length");
                mac.update(KEY_CHECK_MESSAGE.as_bytes());
                let check = decode_hash(&record.key_check).with_context(|| {
                    format!("{}: line {line}: invalid KeyCheck", path.display())
                })?;
                mac.verify_slice(&check)
                    .map_err(|_| anyhow!("{KEY_ENV} does not match {}", path.display()))?;
            } else {
                decode_hash(&record.key_check).context("invalid KeyCheck")?;
            }
            let entry = parse(record)
                .map_err(|problem| anyhow!("{}: line {line}: {problem}", path.display()))?;

            if !seen_ids.insert(entry.redacted.id()) {
                bail!(
                    "{}: line {line}: RedactedId {} is used twice, so two athletes would share \
                     a page",
                    path.display(),
                    entry.redacted.id()
                );
            }

            if !identities.insert((
                entry.hash.clone(),
                entry.query.gender.clone(),
                entry.query.disambiguation,
            )) {
                bail!("{}: line {line}: identity is listed twice", path.display());
            }
            by_hash
                .entry(entry.hash.clone())
                .or_default()
                .push(entries.len());
            entries.push(entry);
        }

        Ok(Self {
            path: path.to_path_buf(),
            key,
            entries,
            by_hash,
        })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn require_key(&self) -> Result<()> {
        if !self.is_empty() && self.key.is_none() {
            bail!("{KEY_ENV} is required to check {}", self.path.display());
        }
        Ok(())
    }

    pub fn next_id(&self) -> Result<u32> {
        self.entries
            .iter()
            .map(|entry| entry.redacted.id())
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .context("No replacement IDs remain")
    }

    pub fn matching<'a>(
        &'a self,
        query: &'a AthleteQuery,
    ) -> impl Iterator<Item = &'a PrivacyEntry> {
        let hash = self
            .key
            .as_deref()
            .map(|key| hash_with(key, &query.match_key));
        hash.and_then(|hash| self.by_hash.get(&hash))
            .into_iter()
            .flatten()
            .map(|index| &self.entries[*index])
            .filter(|entry| {
                query.matches_parts(
                    entry.query.gender.as_deref().unwrap_or_default(),
                    entry.query.country.as_deref().unwrap_or_default(),
                    entry.query.disambiguation,
                )
            })
    }

    pub fn hash(&self, full_name: &str) -> Option<String> {
        self.key
            .as_deref()
            .map(|key| hash_with(key, &match_key(full_name)))
    }

    pub fn lookup(&self, full_name: &str, gender: &str, disambiguation: Option<i16>) -> Lookup {
        let Some(hash) = self.hash(full_name) else {
            return Lookup::MissingKey;
        };

        self.by_hash
            .get(&hash)
            .into_iter()
            .flatten()
            .map(|index| &self.entries[*index])
            .find(|entry| {
                entry
                    .query
                    .gender
                    .as_deref()
                    .is_none_or(|value| value == gender)
                    && entry.query.disambiguation == disambiguation
            })
            .map_or(Lookup::NotListed, |entry| Lookup::Listed(entry.redacted))
    }

    pub fn append(&mut self, entry: PrivacyEntry) -> Result<()> {
        let key = self
            .key
            .as_deref()
            .context(format!("{KEY_ENV} is required to record a redaction"))?;
        if self.entries.iter().any(|existing| {
            existing.hash == entry.hash
                && existing.query.gender == entry.query.gender
                && existing.query.disambiguation == entry.query.disambiguation
                && existing.redacted == entry.redacted
        }) {
            return Ok(());
        }
        anyhow::ensure!(
            !self
                .entries
                .iter()
                .any(|existing| existing.redacted == entry.redacted
                    || (existing.hash == entry.hash
                        && existing.query.gender == entry.query.gender
                        && existing.query.disambiguation == entry.query.disambiguation)),
            "Conflicting suppression record; reload the privacy list"
        );
        let mut entries: Vec<_> = self.entries.iter().chain(std::iter::once(&entry)).collect();
        entries.sort_by_key(|entry| entry.redacted.id());
        let key_check = hash_with(key, KEY_CHECK_MESSAGE);
        let mut writer = csv::Writer::from_writer(Vec::new());
        for entry in entries {
            writer.serialize(PrivacyRecord {
                hash: entry.hash.clone(),
                sex: entry.query.gender.clone(),
                country: entry.query.country.clone(),
                disambiguation: entry.query.disambiguation,
                redacted_id: entry.redacted.id(),
                key_check: key_check.clone(),
            })?;
        }
        let bytes = writer.into_inner()?;
        crate::atomic_file::Replacement::prepare(&self.path, &bytes)?.commit()?;
        self.by_hash
            .entry(entry.hash.clone())
            .or_default()
            .push(self.entries.len());
        self.entries.push(entry);
        Ok(())
    }
}

fn decode_hash(value: &str) -> Result<Vec<u8>> {
    anyhow::ensure!(
        value.len() == 64 && value.bytes().all(|c| c.is_ascii_hexdigit()),
        "expected a SHA-256 fingerprint"
    );
    (0..64)
        .step_by(2)
        .map(|i| u8::from_str_radix(&value[i..i + 2], 16).map_err(Into::into))
        .collect()
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

pub fn hash_with(key: &str, match_key: &str) -> String {
    let mut mac =
        Hmac::<Sha256>::new_from_slice(key.as_bytes()).expect("HMAC takes a key of any length");
    mac.update(match_key.as_bytes());

    mac.finalize()
        .into_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Keeps the next import from undoing a redaction the files already carry.
pub fn check_competition(canonical: &CanonicalFormat, list: &PrivacyList) -> Result<()> {
    list.require_key()?;
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
        let hashed = hash_with("test-key", &match_key("Alina Riyaz"));

        assert_eq!(hashed.len(), 64);
        assert!(!hashed.contains("alina"));
        assert!(hashed.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn the_same_athlete_spelled_differently_hashes_the_same() {
        assert_eq!(
            hash_with("test-key", &match_key("Léa Mérandon")),
            hash_with("test-key", &match_key("LEA MERANDON"))
        );
    }

    #[test]
    fn a_different_key_gives_a_different_hash() {
        assert_ne!(
            hash_with("one", &match_key("Alina Riyaz")),
            hash_with("two", &match_key("Alina Riyaz"))
        );
    }

    #[test]
    fn different_people_do_not_collide() {
        assert_ne!(
            hash_with("test-key", &match_key("Tom Berthier")),
            hash_with("test-key", &match_key("Tom Bertier"))
        );
    }
}
