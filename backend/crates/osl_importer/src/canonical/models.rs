use chrono::{DateTime, NaiveDate, Utc};
use osl_domain::{AthleteStatus, CompetitionStatus, CountryCode, Gender, WeightClassSlug};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Format version this build reads and writes.
pub const FORMAT_VERSION: &str = "1.7.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalFormat {
    /// First key of the document: a reader picks how to parse the rest from
    /// it, so it cannot live inside a section that a later version may move.
    pub format_version: String,

    pub source: SourceMetadata,
    pub competition: CompetitionData,
    pub movements: Vec<MovementData>,
    pub categories: Vec<CategoryData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMetadata {
    #[serde(rename = "type")]
    pub r#type: SourceType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    pub extracted_at: DateTime<Utc>,

    /// Identifier of the producer, e.g. `liftcontrol-scraper@1.2.0`.
    ///
    /// Free text on purpose. This is the only field that names a specific
    /// source, which is what keeps the rest of the format agnostic.
    pub extractor: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_filename: Option<String>,
}

/// How the data was obtained, not who published it.
///
/// Keyed on medium rather than vendor, so a new federation or platform costs
/// no new variant: liftcontrol and any future results endpoint are both
/// `Api`, an Instagram result card is `Image`. The producer names itself in
/// `SourceMetadata::extractor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    Api,
    Html,
    Pdf,
    Csv,
    Image,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitionData {
    pub name: String,

    pub slug: String,

    pub federation: FederationData,

    pub start_date: NaiveDate,

    pub end_date: NaiveDate,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    pub country: CountryCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CompetitionStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationData {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<CountryCode>,
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

    pub gender: Gender,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_class_slug: Option<WeightClassSlug>,

    /// Lower bound, exclusive. Set on its own for an open class such as +87.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_class_min: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_class_max: Option<Decimal>,

    pub athletes: Vec<AthleteData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AthleteData {
    pub first_name: String,

    pub last_name: String,

    /// Separates two different people who genuinely share a name, gender and
    /// country. Set it on the second one onwards, never on someone whose name
    /// is simply spelled inconsistently: that is one person and matching
    /// already handles it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disambiguation: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,

    /// Country the athlete represented at this competition.
    pub country: CountryCode,

    /// Citizenship, when it differs from `country` and is known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<CountryCode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bodyweight: Option<Decimal>,

    /// Score as the source stated it, for sources that publish a RIS but no
    /// bodyweight. Never set alongside `bodyweight`, which we compute from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ris: Option<Decimal>,

    pub status: AthleteStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,

    pub lifts: Vec<LiftData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiftData {
    pub movement: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempts: Option<Vec<AttemptData>>,

    /// Best weight as the source stated it, for sources that publish only a
    /// best lift per movement. Never set alongside `attempts`, which we
    /// derive it from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_lift: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptData {
    pub attempt_number: i16,
    pub weight: Decimal,
    pub is_successful: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub judge_note: Option<String>,
}
