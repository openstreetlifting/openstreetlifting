use chrono::NaiveDate;
use osl_domain::{CompetitionStatus, CountryCode};
use serde::{Deserialize, Serialize};

use super::models::{CompetitionData, FederationData};

pub const FILE_NAME: &str = "meet.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeetFile {
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
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    pub country: CountryCode,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CompetitionStatus>,
}

impl CompetitionSection {
    pub fn into_data(self, slug: String, federation: FederationData) -> CompetitionData {
        CompetitionData {
            name: self.name,
            slug,
            federation,
            start_date: self.start_date,
            end_date: self.end_date,
            city: self.city,
            region: self.region,
            country: self.country,
            status: self.status,
        }
    }

    pub fn from_data(competition: &CompetitionData) -> Self {
        Self {
            name: competition.name.clone(),
            start_date: competition.start_date,
            end_date: competition.end_date,
            city: competition.city.clone(),
            region: competition.region.clone(),
            country: competition.country,
            status: competition.status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEET: &str = r#"
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
    fn a_well_formed_file_parses() {
        let meet: MeetFile = toml::from_str(MEET).unwrap();
        assert_eq!(meet.competition.city.as_deref(), Some("Sevran"));
    }

    #[test]
    fn a_misspelled_key_is_rejected() {
        let text = MEET.replace("city =", "citty =");
        let error = toml::from_str::<MeetFile>(&text).unwrap_err().to_string();
        assert!(error.contains("citty"), "{error}");
    }

    #[test]
    fn a_leftover_format_version_is_rejected() {
        let text = format!("format_version = \"2.0.0\"\n{MEET}");
        let error = toml::from_str::<MeetFile>(&text).unwrap_err().to_string();
        assert!(error.contains("format_version"), "{error}");
    }
}
