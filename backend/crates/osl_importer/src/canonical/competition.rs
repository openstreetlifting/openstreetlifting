use chrono::NaiveDate;
use osl_domain::{CompetitionStatus, CountryCode};
use serde::{Deserialize, Serialize};

use super::models::{CompetitionData, FederationData};

pub const FILE_NAME: &str = "competition.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompetitionFile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<String>,

    pub competition: CompetitionSection,
    pub federation: FederationData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompetitionSection {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    pub start_date: NaiveDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    pub country: CountryCode,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CompetitionStatus>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scoring: Option<osl_domain::Scoring>,
}

impl CompetitionSection {
    #[must_use]
    pub fn into_data(self, slug: String, federation: FederationData) -> CompetitionData {
        CompetitionData {
            name: self.name,
            series: self.series,
            slug,
            federation,
            start_date: self.start_date,
            end_date: self.end_date.unwrap_or(self.start_date),
            city: self.city,
            venue: self.venue,
            region: self.region,
            country: self.country,
            status: self.status,
            scoring: self.scoring,
        }
    }

    #[must_use]
    pub fn from_data(competition: &CompetitionData) -> Self {
        Self {
            name: competition.name.clone(),
            series: competition.series.clone(),
            start_date: competition.start_date,
            end_date: (competition.end_date != competition.start_date)
                .then_some(competition.end_date),
            city: competition.city.clone(),
            venue: competition.venue.clone(),
            region: competition.region.clone(),
            country: competition.country,
            status: competition.status,
            scoring: competition.scoring,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMPETITION: &str = r#"
event = "MPDS"

[competition]
name = "Elite"
start_date = "2026-05-15"
end_date = "2026-05-17"
city = "Sevran"
country = "FR"
status = "completed"

[federation]
name = "FNSL"
country = "FR"
"#;

    #[test]
    fn parses_valid_files() {
        let parsed: CompetitionFile = toml::from_str(COMPETITION).unwrap();
        assert_eq!(parsed.competition.city.as_deref(), Some("Sevran"));
        assert_eq!(parsed.competition.series, None);
    }

    #[test]
    fn preserves_series_when_formatting_metadata() {
        let text = COMPETITION.replace(
            "name = \"Elite\"",
            "name = \"Elite\"\nseries = \"fnsl-elite\"",
        );
        let file: CompetitionFile = toml::from_str(&text).unwrap();
        let data = file.competition.into_data("elite".into(), file.federation);
        assert_eq!(data.series.as_deref(), Some("fnsl-elite"));
        let output = toml::to_string(&CompetitionSection::from_data(&data)).unwrap();
        assert!(output.contains("series = \"fnsl-elite\""));
    }

    #[test]
    fn defaults_end_date_for_single_day_events() {
        let text = COMPETITION.replace("end_date = \"2026-05-17\"", "venue = \"Main Hall\"");
        let file: CompetitionFile = toml::from_str(&text).unwrap();
        let data = file.competition.into_data("elite".into(), file.federation);
        assert_eq!(data.end_date, data.start_date);
        assert_eq!(data.venue.as_deref(), Some("Main Hall"));
        let output = toml::to_string(&CompetitionSection::from_data(&data)).unwrap();
        assert!(!output.contains("end_date"));
        assert!(output.contains("venue = \"Main Hall\""));
    }

    #[test]
    fn preserves_explicit_end_dates() {
        let file: CompetitionFile = toml::from_str(COMPETITION).unwrap();
        let data = file.competition.into_data("elite".into(), file.federation);
        assert_eq!(data.end_date, NaiveDate::from_ymd_opt(2026, 5, 17).unwrap());
        let output = toml::to_string(&CompetitionSection::from_data(&data)).unwrap();
        assert!(output.contains("end_date = \"2026-05-17\""));
    }

    #[test]
    fn rejects_unknown_keys() {
        let text = COMPETITION.replace("city =", "citty =");
        let error = toml::from_str::<CompetitionFile>(&text)
            .unwrap_err()
            .to_string();
        assert!(error.contains("citty"), "{error}");
    }

    #[test]
    fn rejects_legacy_format_versions() {
        let text = format!("format_version = \"2.0.0\"\n{COMPETITION}");
        let error = toml::from_str::<CompetitionFile>(&text)
            .unwrap_err()
            .to_string();
        assert!(error.contains("format_version"), "{error}");
    }
}
