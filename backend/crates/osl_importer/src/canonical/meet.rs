use chrono::NaiveDate;
use osl_domain::{CompetitionStatus, CountryCode};
use serde::{Deserialize, Serialize};

use super::models::{CompetitionData, FederationData};

pub const FILE_NAME: &str = "meet.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetFile {
    pub format_version: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<String>,

    pub competition: CompetitionSection,
    pub federation: FederationData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
