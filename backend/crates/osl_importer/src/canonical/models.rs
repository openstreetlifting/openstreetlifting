use chrono::NaiveDate;
use osl_domain::{
    AthleteStatus, CompetitionStatus, CountryCode, Gender, Movement, WeightClassSlug,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub const FORMAT_VERSION: &str = "2.0.0";

#[derive(Debug, Clone)]
pub struct CanonicalFormat {
    pub format_version: String,
    pub sources: Vec<String>,
    pub competition: CompetitionData,
    pub movements: Vec<Movement>,
    pub categories: Vec<CategoryData>,
}

#[derive(Debug, Clone)]
pub struct CompetitionData {
    pub name: String,
    pub slug: String,
    pub federation: FederationData,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: CountryCode,
    pub status: Option<CompetitionStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationData {
    pub name: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<CountryCode>,
}

#[derive(Debug, Clone)]
pub struct CategoryData {
    pub name: String,
    pub gender: Gender,
    pub weight_class_slug: Option<WeightClassSlug>,
    pub weight_class_min: Option<Decimal>,
    pub weight_class_max: Option<Decimal>,
    pub athletes: Vec<AthleteData>,
}

#[derive(Debug, Clone)]
pub struct AthleteData {
    pub first_name: String,
    pub last_name: String,
    pub disambiguation: Option<i16>,
    pub gender: Option<Gender>,
    pub country: CountryCode,
    pub bodyweight: Option<Decimal>,
    pub ris: Option<Decimal>,
    pub status: AthleteStatus,
    pub status_reason: Option<String>,
    pub lifts: Vec<LiftData>,
}

#[derive(Debug, Clone)]
pub struct LiftData {
    pub movement: Movement,
    pub attempts: Option<Vec<AttemptData>>,
    pub best_lift: Option<Decimal>,
}

#[derive(Debug, Clone)]
pub struct AttemptData {
    pub attempt_number: i16,
    pub weight: Decimal,
    pub is_successful: bool,
}
