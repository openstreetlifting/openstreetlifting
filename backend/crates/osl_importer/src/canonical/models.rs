use chrono::{DateTime, NaiveDate, Utc};
use osl_domain::{AthleteStatus, SourceType, WeightClassSlug};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalFormat {
    pub source: SourceMetadata,
    pub competition: CompetitionData,
    pub movements: Vec<MovementData>,
    pub categories: Vec<CategoryData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMetadata {
    pub format_version: String,

    #[serde(rename = "type")]
    pub r#type: SourceType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    pub extracted_at: DateTime<Utc>,

    pub extractor: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_filename: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitionData {
    pub name: String,

    pub slug: String,

    pub federation: FederationData,

    pub start_date: NaiveDate,

    pub end_date: NaiveDate,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    pub country: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_judges: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationData {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementData {
    pub name: String,

    pub order: i16,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryData {
    pub name: String,

    pub gender: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_class_slug: Option<WeightClassSlug>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_class_max: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_open_category: Option<bool>,

    pub athletes: Vec<AthleteData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AthleteData {
    pub first_name: String,

    pub last_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    pub country: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bodyweight: Option<Decimal>,

    pub status: AthleteStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,

    pub lifts: Vec<LiftData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiftData {
    pub movement: String,
    pub attempts: Vec<AttemptData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptData {
    pub attempt_number: i16,
    pub weight: Decimal,
    pub is_successful: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub judge_note: Option<String>,
}
