use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result, anyhow, bail};
use osl_domain::{CountryCode, Gender};
#[cfg(feature = "database")]
use sqlx::PgPool;
#[cfg(feature = "database")]
use uuid::Uuid;

use crate::identity::AthleteQuery;

/// row of the file.
/// `Sex`, `Country` and `Disambiguation` are spelled like in `entries.csv`
#[derive(Debug, serde::Deserialize)]
struct InstagramRecord {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Sex", default)]
    sex: Option<String>,
    #[serde(rename = "Country", default)]
    country: Option<String>,
    #[serde(rename = "Disambiguation", default)]
    disambiguation: Option<i16>,
    #[serde(rename = "Instagram")]
    instagram: String,
}

#[derive(Debug)]
pub(crate) struct HandleRow {
    label: String,
    pub(crate) query: AthleteQuery,
    handle: String,
}

#[cfg(any(feature = "database", test))]
#[derive(Debug)]
struct AthleteIdentity {
    #[cfg(feature = "database")]
    athlete_id: Uuid,
    gender: String,
    country: String,
    disambiguation: Option<i16>,
}

#[cfg(any(feature = "database", test))]
impl AthleteIdentity {
    fn describe(&self) -> String {
        self.disambiguation.map_or_else(
            || format!("{}/{}", self.gender, self.country),
            |number| format!("{}/{} #{number}", self.gender, self.country),
        )
    }
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

#[cfg(feature = "database")]
pub async fn check_instagram_handles(file: &Path, pool: &PgPool) -> Result<InstagramReport> {
    let (_, report) = resolve(read_file(file)?, pool).await?;
    Ok(report)
}

#[cfg(feature = "database")]
pub async fn load_instagram_handles(file: &Path, pool: &PgPool) -> Result<InstagramReport> {
    let (resolved, report) = resolve(read_file(file)?, pool).await?;

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
            handle
        )
        .execute(&mut *tx)
        .await
        .with_context(|| format!("handle '{handle}' is already taken by another athlete"))?;
    }

    tx.commit().await?;

    Ok(report)
}

#[cfg(feature = "database")]
async fn resolve(
    rows: Vec<HandleRow>,
    pool: &PgPool,
) -> Result<(Vec<(Uuid, String)>, InstagramReport)> {
    let athletes = sqlx::query!(
        r#"
        SELECT athlete_id as "athlete_id: Uuid", match_key, gender, country, disambiguation
        FROM athletes
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut by_match_key: BTreeMap<String, Vec<AthleteIdentity>> = BTreeMap::new();
    for athlete in athletes {
        by_match_key
            .entry(athlete.match_key)
            .or_default()
            .push(AthleteIdentity {
                athlete_id: athlete.athlete_id,
                gender: athlete.gender,
                country: athlete.country.unwrap_or_default(),
                disambiguation: athlete.disambiguation,
            });
    }

    let mut report = InstagramReport::default();
    let mut resolved: Vec<(Uuid, String)> = Vec::new();

    for row in rows {
        let named = by_match_key
            .get(&row.query.match_key)
            .map(Vec::as_slice)
            .unwrap_or_default();

        let candidates: Vec<&AthleteIdentity> = named
            .iter()
            .filter(|a| {
                row.query
                    .matches_parts(&a.gender, &a.country, a.disambiguation)
            })
            .collect();

        match candidates.as_slice() {
            [] => report.unknown.push(no_athlete(&row, named)),
            [athlete] => {
                resolved.push((athlete.athlete_id, row.handle));
                report.matched += 1;
            }
            _ => report.ambiguous.push(too_many_athletes(&row, &candidates)),
        }
    }

    if !report.unknown.is_empty() || !report.ambiguous.is_empty() {
        tracing::error!(
            unknown = report.unknown.len(),
            ambiguous = report.ambiguous.len(),
            "Instagram athlete matching failed"
        );
        bail!(
            "{} unknown and {} ambiguous name(s), nothing was written",
            report.unknown.len(),
            report.ambiguous.len()
        );
    }

    Ok((resolved, report))
}

#[cfg(any(feature = "database", test))]
fn no_athlete(row: &HandleRow, named: &[AthleteIdentity]) -> String {
    if named.is_empty() {
        return format!("No athlete named '{}'", row.label);
    }

    format!(
        "No athlete named '{}' with {}. The name matches {}",
        row.label,
        row.query.narrowing(),
        describe_all(named.iter())
    )
}

#[cfg(any(feature = "database", test))]
fn too_many_athletes(row: &HandleRow, candidates: &[&AthleteIdentity]) -> String {
    format!(
        "'{}' matches {} athletes: {}. Fill in Sex, Country or Disambiguation to say which one",
        row.label,
        candidates.len(),
        describe_all(candidates.iter().copied())
    )
}

#[cfg(any(feature = "database", test))]
fn describe_all<'a>(athletes: impl Iterator<Item = &'a AthleteIdentity>) -> String {
    let mut described: Vec<String> = athletes.map(AthleteIdentity::describe).collect();
    described.sort();
    described.join(", ")
}

fn read_file(file: &Path) -> Result<Vec<HandleRow>> {
    let mut reader = csv::Reader::from_path(file)
        .with_context(|| format!("Failed to read {}", file.display()))?;

    let mut rows = Vec::new();
    let mut athletes: BTreeMap<AthleteQuery, String> = BTreeMap::new();
    let mut handles: BTreeMap<String, String> = BTreeMap::new();

    let headers = reader.headers()?.clone();
    for record in reader.records() {
        let record = record.with_context(|| format!("Malformed row in {}", file.display()))?;
        let row = parse_record(&record, &headers)?;

        if let Some(first) = athletes.insert(row.query.clone(), row.label.clone()) {
            bail!("'{first}' is listed twice");
        }

        if let Some(first) = handles.insert(row.handle.to_lowercase(), row.label.clone()) {
            bail!(
                "'{}' and '{}' are both listed with the handle '{}'",
                first,
                row.label,
                row.handle
            );
        }

        rows.push(row);
    }

    Ok(rows)
}

pub(crate) fn parse_record(
    record: &csv::StringRecord,
    headers: &csv::StringRecord,
) -> Result<HandleRow> {
    parse(&record.deserialize(Some(headers))?)
}

fn parse(record: &InstagramRecord) -> Result<HandleRow> {
    let label = record.name.trim().to_string();
    if label.is_empty() {
        bail!("a row has a handle with no name");
    }

    if osl_domain::redaction::RedactedAthlete::from_match_key(&crate::identity::match_key(&label))
        .is_some()
    {
        bail!("'{label}' asked to be taken off the site, so they cannot carry a handle");
    }

    let handle = record.instagram.trim().trim_start_matches('@');
    if handle.is_empty() || handle.len() > 30 {
        bail!("'{}' is not an Instagram handle", record.instagram);
    }
    if !handle
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_')
    {
        bail!("'{}' is not an Instagram handle", record.instagram);
    }

    let gender = optional(record.sex.as_deref())
        .map(|sex| {
            sex.parse::<Gender>()
                .map_err(|problem| anyhow!("'{label}' has an unreadable Sex: {problem}"))
        })
        .transpose()?;

    let country = optional(record.country.as_deref())
        .map(|country| {
            CountryCode::parse(country)
                .map_err(|problem| anyhow!("'{label}' has an unreadable Country: {problem}"))
        })
        .transpose()?;

    if let Some(number) = record.disambiguation
        && number < 1
    {
        bail!(
            "'{label}' has disambiguation {number}. It numbers the people who share a name, \
             starting at 1"
        );
    }

    Ok(HandleRow {
        query: AthleteQuery::new(
            &label,
            gender.map(|gender| gender.as_str().to_string()),
            country.map(|country| country.as_str().to_string()),
            record.disambiguation,
        ),
        label,
        handle: handle.to_string(),
    })
}

fn optional(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(gender: &str, country: &str, disambiguation: Option<i16>) -> AthleteIdentity {
        AthleteIdentity {
            #[cfg(feature = "database")]
            athlete_id: Uuid::nil(),
            gender: gender.to_string(),
            country: country.to_string(),
            disambiguation,
        }
    }

    fn row(
        name: &str,
        sex: Option<&str>,
        country: Option<&str>,
        handle: &str,
    ) -> Result<HandleRow> {
        parse(&InstagramRecord {
            name: name.to_string(),
            sex: sex.map(str::to_string),
            country: country.map(str::to_string),
            disambiguation: None,
            instagram: handle.to_string(),
        })
    }

    #[test]
    fn parses_identity_columns() {
        let parsed = row("Tony Nguyen", Some("m"), Some("fr"), "@tony").unwrap();

        assert_eq!(parsed.label, "Tony Nguyen");
        assert_eq!(parsed.handle, "tony");
        assert_eq!(parsed.query.gender.as_deref(), Some("M"));
        assert_eq!(parsed.query.country.as_deref(), Some("FR"));
    }

    #[test]
    fn ignores_blank_identity_columns() {
        let parsed = row("Tony Nguyen", Some("  "), None, "tony").unwrap();

        assert_eq!(parsed.query.gender, None);
        assert_eq!(parsed.query.country, None);
    }

    #[test]
    fn identifies_rows_with_invalid_columns() {
        let problem = row("Tony Nguyen", Some("male"), None, "tony")
            .unwrap_err()
            .to_string();
        assert!(problem.contains("Tony Nguyen"), "{problem}");
        assert!(problem.contains("Sex"), "{problem}");

        let problem = row("Tony Nguyen", None, Some("France"), "tony")
            .unwrap_err()
            .to_string();
        assert!(problem.contains("Country"), "{problem}");
    }

    #[test]
    fn rejects_invalid_handles() {
        assert!(row("Tony Nguyen", None, None, "").is_err());
        assert!(row("Tony Nguyen", None, None, "tony nguyen").is_err());
        assert!(row("", None, None, "tony").is_err());
    }

    #[test]
    fn lists_candidates_when_identity_is_ambiguous() {
        let row = row("Tony Nguyen", None, None, "tony").unwrap();
        let candidates = [identity("M", "US", None), identity("M", "FR", None)];

        let problem = too_many_athletes(&row, &candidates.iter().collect::<Vec<_>>());

        assert!(problem.contains("matches 2 athletes"), "{problem}");
        assert!(problem.contains("M/FR, M/US"), "{problem}");
        assert!(problem.contains("Country"), "{problem}");
    }

    #[test]
    fn lists_candidates_when_filters_do_not_match() {
        let row = row("Tony Nguyen", None, Some("DE"), "tony").unwrap();
        let named = [identity("M", "FR", None), identity("M", "US", None)];

        let problem = no_athlete(&row, &named);

        assert!(problem.contains("Country 'DE'"), "{problem}");
        assert!(problem.contains("M/FR, M/US"), "{problem}");
    }

    #[test]
    fn reports_unknown_athletes() {
        let row = row("Tony Nguyen", None, None, "tony").unwrap();

        assert_eq!(no_athlete(&row, &[]), "No athlete named 'Tony Nguyen'");
    }
}
