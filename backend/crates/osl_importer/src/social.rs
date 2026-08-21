use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result, bail};
use osl_domain::normalized_name::NormalizedAthleteName;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
struct InstagramRecord {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Instagram")]
    instagram: String,
}

#[derive(Debug, Default)]
pub struct InstagramReport {
    pub matched: usize,
    pub unknown: Vec<String>,
    pub ambiguous: Vec<String>,
}

pub fn validate_file(file: &Path) -> Result<usize> {
    Ok(read_file(file)?.len())
}

pub async fn load_instagram_handles(file: &Path, pool: &PgPool) -> Result<InstagramReport> {
    let records = read_file(file)?;

    let athletes =
        sqlx::query!(r#"SELECT athlete_id as "athlete_id: Uuid", match_key FROM athletes"#)
            .fetch_all(pool)
            .await?;

    let mut by_match_key: BTreeMap<String, Vec<Uuid>> = BTreeMap::new();
    for athlete in athletes {
        by_match_key
            .entry(athlete.match_key)
            .or_default()
            .push(athlete.athlete_id);
    }

    let mut report = InstagramReport::default();
    let mut resolved: Vec<(Uuid, String)> = Vec::new();

    for record in records {
        match by_match_key.get(&match_key(&record.name)) {
            None => report.unknown.push(record.name),
            Some(ids) if ids.len() > 1 => report.ambiguous.push(record.name),
            Some(ids) => {
                resolved.push((ids[0], record.instagram));
                report.matched += 1;
            }
        }
    }

    if !report.unknown.is_empty() || !report.ambiguous.is_empty() {
        for name in &report.unknown {
            tracing::error!("No athlete named '{}'", name);
        }
        for name in &report.ambiguous {
            tracing::error!("'{}' matches more than one athlete", name);
        }
        bail!(
            "{} unknown and {} ambiguous name(s), nothing was written",
            report.unknown.len(),
            report.ambiguous.len()
        );
    }

    let mut tx = pool.begin().await?;

    let social_id = sqlx::query_scalar!(
        r#"SELECT social_id as "social_id: Uuid" FROM socials WHERE name = 'instagram'"#
    )
    .fetch_optional(&mut *tx)
    .await?
    .context("the 'instagram' row is missing from socials, run the migrations")?;

    sqlx::query!(
        "DELETE FROM athlete_socials WHERE social_id = $1",
        social_id
    )
    .execute(&mut *tx)
    .await?;

    for (athlete_id, handle) in &resolved {
        sqlx::query!(
            r#"
            INSERT INTO athlete_socials (athlete_id, social_id, handle)
            VALUES ($1, $2, $3)
            "#,
            athlete_id,
            social_id,
            handle.trim_start_matches('@')
        )
        .execute(&mut *tx)
        .await
        .with_context(|| format!("handle '{}' is already taken by another athlete", handle))?;
    }

    tx.commit().await?;

    Ok(report)
}

fn read_file(file: &Path) -> Result<Vec<InstagramRecord>> {
    let mut reader = csv::Reader::from_path(file)
        .with_context(|| format!("Failed to read {}", file.display()))?;

    let mut records = Vec::new();
    let mut seen: BTreeMap<String, String> = BTreeMap::new();

    for record in reader.deserialize() {
        let record: InstagramRecord =
            record.with_context(|| format!("Malformed row in {}", file.display()))?;

        if let Some(first) = seen.insert(match_key(&record.name), record.name.clone()) {
            bail!("'{}' is listed twice", first);
        }

        let handle = record.instagram.trim_start_matches('@');
        if handle.is_empty() || handle.len() > 30 {
            bail!("'{}' is not an Instagram handle", record.instagram);
        }
        if !handle
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_')
        {
            bail!("'{}' is not an Instagram handle", record.instagram);
        }

        records.push(record);
    }

    Ok(records)
}

fn match_key(full_name: &str) -> String {
    let mut parts = full_name.trim().splitn(2, char::is_whitespace);
    let first = parts.next().unwrap_or_default();
    let last = parts.next().unwrap_or_default();
    NormalizedAthleteName::new(first, last).match_name()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_the_same_way_the_importer_does() {
        let athlete = NormalizedAthleteName::new("Léa", "Mérandon").match_name();
        assert_eq!(match_key("Léa Mérandon"), athlete);
        assert_eq!(match_key("LEA MERANDON"), athlete);
        assert_eq!(match_key("  lea   merandon "), athlete);
    }

    #[test]
    fn a_middle_name_stays_part_of_the_key() {
        let athlete = NormalizedAthleteName::new("Jean Luc", "Picard").match_name();
        assert_eq!(match_key("Jean Luc Picard"), athlete);
        assert_eq!(match_key("Jean-Luc Picard"), athlete);
    }

    #[test]
    fn different_people_do_not_collide() {
        assert_ne!(match_key("Tom Berthier"), match_key("Tom Bertier"));
    }
}
