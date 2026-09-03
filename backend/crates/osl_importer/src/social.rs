//! Attaching a social handle to an athlete, from a file that names them.
//!
//! A handle is written against a name, because a name is what the person
//! adding it knows. Identity, though, is the folded name together with gender,
//! country and disambiguation, so a name is a coarser key than an athlete: two
//! people who share one are two rows the name alone cannot tell apart.
//!
//! Nothing is guessed when that happens. The row is refused and the file is
//! narrowed by hand with the columns `entries.csv` already uses, so the file
//! can say which of the two it means instead of leaving it to whichever row
//! came back first.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result, anyhow, bail};
use osl_domain::normalized_name::NormalizedAthleteName;
use osl_domain::{CountryCode, Gender};
use sqlx::PgPool;
use uuid::Uuid;

/// A row of the file.
///
/// `Sex`, `Country` and `Disambiguation` are spelled the way `entries.csv`
/// spells them, so the values can be copied across from the result that
/// created the athlete. They are blank for everyone whose name already names
/// one person, which is nearly everyone, and `serde(default)` lets a file that
/// predates them keep working.
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

/// A row with its columns checked: an athlete to find, and the handle to give
/// them.
#[derive(Debug)]
struct HandleRow {
    /// The name as the file spells it, so every message about this row can be
    /// found by searching the file for what it says.
    label: String,
    query: AthleteQuery,
    handle: String,
}

/// Who the file says the handle belongs to.
///
/// A `None` reads as "any", so a unique name needs nothing filled in. It is
/// deliberately not "unset means NULL": a blank `Disambiguation` next to a
/// numbered athlete has to stay ambiguous, because a file that means the
/// unnumbered one has to say so by being narrowed on something else.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct AthleteQuery {
    match_key: String,
    gender: Option<String>,
    country: Option<String>,
    disambiguation: Option<i16>,
}

/// An athlete as the database identifies them.
#[derive(Debug)]
struct AthleteIdentity {
    athlete_id: Uuid,
    gender: String,
    country: String,
    disambiguation: Option<i16>,
}

impl AthleteQuery {
    fn matches(&self, athlete: &AthleteIdentity) -> bool {
        self.gender.as_deref().is_none_or(|g| g == athlete.gender)
            && self.country.as_deref().is_none_or(|c| c == athlete.country)
            && self
                .disambiguation
                .is_none_or(|number| Some(number) == athlete.disambiguation)
    }

    /// The narrowing the row asked for, worded for a message that has to say
    /// why an athlete who exists was not the one this row wanted.
    fn narrowing(&self) -> String {
        let mut columns = Vec::new();

        if let Some(gender) = &self.gender {
            columns.push(format!("Sex '{gender}'"));
        }
        if let Some(country) = &self.country {
            columns.push(format!("Country '{country}'"));
        }
        if let Some(number) = self.disambiguation {
            columns.push(format!("Disambiguation '{number}'"));
        }

        columns.join(", ")
    }
}

impl AthleteIdentity {
    fn describe(&self) -> String {
        match self.disambiguation {
            Some(number) => format!("{}/{} #{}", self.gender, self.country, number),
            None => format!("{}/{}", self.gender, self.country),
        }
    }
}

#[derive(Debug, Default)]
pub struct InstagramReport {
    pub matched: usize,
    pub unknown: Vec<String>,
    pub ambiguous: Vec<String>,
}

/// Checks the file on its own: that it parses, that each handle is spelled
/// like a handle, and that no athlete or handle is listed twice.
///
/// It cannot tell whether a name still names one athlete, which needs the
/// athletes. Prefer [`check_instagram_handles`] wherever a database is reachable.
pub fn validate_file(file: &Path) -> Result<usize> {
    Ok(read_file(file)?.len())
}

/// Resolves every row against the database without writing anything.
///
/// A name stops naming one athlete the moment a competition brings in a second
/// person who shares it, and the file that was correct yesterday is the one
/// that breaks. Running this on the pull request that adds that competition
/// puts the failure next to its cause, rather than in the deploy that runs the
/// import.
pub async fn check_instagram_handles(file: &Path, pool: &PgPool) -> Result<InstagramReport> {
    let (_, report) = resolve(read_file(file)?, pool).await?;
    Ok(report)
}

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
        .with_context(|| format!("handle '{}' is already taken by another athlete", handle))?;
    }

    tx.commit().await?;

    Ok(report)
}

/// Pairs every row with the one athlete it names, or refuses the whole file.
///
/// A row that names nobody and a row that names two people are both mistakes
/// only a human can settle, and a file that is half applied is harder to
/// reason about than one that was not applied at all, so neither is written.
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
                country: athlete.country,
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

        let candidates: Vec<&AthleteIdentity> =
            named.iter().filter(|a| row.query.matches(a)).collect();

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
        for problem in report.unknown.iter().chain(&report.ambiguous) {
            tracing::error!("{problem}");
        }
        bail!(
            "{} unknown and {} ambiguous name(s), nothing was written",
            report.unknown.len(),
            report.ambiguous.len()
        );
    }

    Ok((resolved, report))
}

/// Nobody matched. Whether that is a name the database has never seen or a
/// narrowing that excluded the athlete it meant are different mistakes, so
/// they read differently.
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

fn too_many_athletes(row: &HandleRow, candidates: &[&AthleteIdentity]) -> String {
    format!(
        "'{}' matches {} athletes: {}. Fill in Sex, Country or Disambiguation to say which one",
        row.label,
        candidates.len(),
        describe_all(candidates.iter().copied())
    )
}

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

    for record in reader.deserialize() {
        let record: InstagramRecord =
            record.with_context(|| format!("Malformed row in {}", file.display()))?;

        let row = parse(record)?;

        if let Some(first) = athletes.insert(row.query.clone(), row.label.clone()) {
            bail!("'{}' is listed twice", first);
        }

        // An account belongs to one athlete, which the database enforces, so
        // two rows claiming it is a mistake worth naming here rather than an
        // insert to let fail.
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

fn parse(record: InstagramRecord) -> Result<HandleRow> {
    let label = record.name.trim().to_string();
    if label.is_empty() {
        bail!("a row has a handle with no name");
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
        query: AthleteQuery {
            match_key: match_key(&label),
            gender: gender.map(|gender| gender.as_str().to_string()),
            country: country.map(|country| country.as_str().to_string()),
            disambiguation: record.disambiguation,
        },
        label,
        handle: handle.to_string(),
    })
}

/// A column left blank is a column the file did not fill in, whether it holds
/// nothing or only spaces.
fn optional(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
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

    fn identity(gender: &str, country: &str, disambiguation: Option<i16>) -> AthleteIdentity {
        AthleteIdentity {
            athlete_id: Uuid::nil(),
            gender: gender.to_string(),
            country: country.to_string(),
            disambiguation,
        }
    }

    fn query(gender: Option<&str>, country: Option<&str>, number: Option<i16>) -> AthleteQuery {
        AthleteQuery {
            match_key: match_key("Tony Nguyen"),
            gender: gender.map(str::to_string),
            country: country.map(str::to_string),
            disambiguation: number,
        }
    }

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

    #[test]
    fn a_blank_column_matches_any_athlete() {
        let anyone = query(None, None, None);

        assert!(anyone.matches(&identity("M", "FR", None)));
        assert!(anyone.matches(&identity("F", "US", Some(2))));
    }

    #[test]
    fn a_filled_column_only_matches_the_athlete_it_names() {
        let french = query(None, Some("FR"), None);

        assert!(french.matches(&identity("M", "FR", None)));
        assert!(!french.matches(&identity("M", "US", None)));

        let second = query(None, None, Some(2));

        assert!(second.matches(&identity("M", "FR", Some(2))));
        assert!(!second.matches(&identity("M", "FR", None)));
        assert!(!second.matches(&identity("M", "FR", Some(1))));
    }

    #[test]
    fn the_columns_narrow_together() {
        let one = query(Some("F"), Some("IT"), None);

        assert!(one.matches(&identity("F", "IT", None)));
        assert!(!one.matches(&identity("M", "IT", None)));
        assert!(!one.matches(&identity("F", "SM", None)));
    }

    fn row(
        name: &str,
        sex: Option<&str>,
        country: Option<&str>,
        handle: &str,
    ) -> Result<HandleRow> {
        parse(InstagramRecord {
            name: name.to_string(),
            sex: sex.map(str::to_string),
            country: country.map(str::to_string),
            disambiguation: None,
            instagram: handle.to_string(),
        })
    }

    #[test]
    fn a_row_reads_the_columns_it_was_given() {
        let parsed = row("Tony Nguyen", Some("m"), Some("fr"), "@tony").unwrap();

        assert_eq!(parsed.label, "Tony Nguyen");
        assert_eq!(parsed.handle, "tony");
        assert_eq!(parsed.query.gender.as_deref(), Some("M"));
        assert_eq!(parsed.query.country.as_deref(), Some("FR"));
    }

    #[test]
    fn a_blank_column_is_left_unset() {
        let parsed = row("Tony Nguyen", Some("  "), None, "tony").unwrap();

        assert_eq!(parsed.query.gender, None);
        assert_eq!(parsed.query.country, None);
    }

    #[test]
    fn a_column_that_cannot_be_read_names_the_row_it_is_in() {
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
    fn a_handle_is_still_checked_for_being_one() {
        assert!(row("Tony Nguyen", None, None, "").is_err());
        assert!(row("Tony Nguyen", None, None, "tony nguyen").is_err());
        assert!(row("", None, None, "tony").is_err());
    }

    #[test]
    fn an_ambiguous_row_is_told_what_would_settle_it() {
        let row = row("Tony Nguyen", None, None, "tony").unwrap();
        let candidates = [identity("M", "US", None), identity("M", "FR", None)];

        let problem = too_many_athletes(&row, &candidates.iter().collect::<Vec<_>>());

        assert!(problem.contains("matches 2 athletes"), "{problem}");
        assert!(problem.contains("M/FR, M/US"), "{problem}");
        assert!(problem.contains("Country"), "{problem}");
    }

    #[test]
    fn a_narrowing_that_matches_nobody_says_what_the_name_does_match() {
        let row = row("Tony Nguyen", None, Some("DE"), "tony").unwrap();
        let named = [identity("M", "FR", None), identity("M", "US", None)];

        let problem = no_athlete(&row, &named);

        assert!(problem.contains("Country 'DE'"), "{problem}");
        assert!(problem.contains("M/FR, M/US"), "{problem}");
    }

    #[test]
    fn a_name_nobody_carries_says_only_that() {
        let row = row("Tony Nguyen", None, None, "tony").unwrap();

        assert_eq!(no_athlete(&row, &[]), "No athlete named 'Tony Nguyen'");
    }
}
