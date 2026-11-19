use osl_domain::{Constants, Edition, Gender};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// An edition of the RIS formula.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RisFormulaResponse {
    /// Edition year.
    pub year: i32,
    /// Whether rankings use this edition.
    pub is_current: bool,
    /// Constants by gender.
    pub constants: RisConstants,
}

/// Formula constants by gender.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RisConstants {
    /// Constants for men.
    pub men: GenderConstants,
    /// Constants for women.
    pub women: GenderConstants,
}

/// Benchmark constants.
///
/// RIS is `total * 100 / benchmark`, where
/// `benchmark = a + (k - a) / (1 + q * exp(-b * (bodyweight - v)))`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GenderConstants {
    /// Lower asymptote.
    pub a: f64,
    /// Upper asymptote.
    pub k: f64,
    /// Growth rate.
    pub b: f64,
    /// Bodyweight offset, in kilograms.
    pub v: f64,
    /// Shape constant.
    pub q: f64,
}

/// Inputs for a RIS score.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[schema(example = json!({"bodyweight": "80", "total": "300", "gender": "M"}))]
pub struct ComputeRisRequest {
    /// Bodyweight in kilograms.
    pub bodyweight: Decimal,
    /// Total in kilograms.
    pub total: Decimal,
    /// `M` or `F`.
    #[serde(deserialize_with = "crate::shared::dto::from_str")]
    pub gender: Gender,
    /// Edition year. Defaults to the current edition.
    pub formula_year: Option<i32>,
}

/// A RIS score.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ComputeRisResponse {
    /// RIS points, to two decimals.
    pub ris_score: Decimal,
    /// Edition year used.
    pub formula_year: i32,
}

impl From<Constants> for GenderConstants {
    fn from(constants: Constants) -> Self {
        Self {
            a: constants.a,
            k: constants.k,
            b: constants.b,
            v: constants.v,
            q: constants.q,
        }
    }
}

impl From<Edition> for RisFormulaResponse {
    fn from(edition: Edition) -> Self {
        Self {
            year: edition.year(),
            is_current: edition == Edition::CURRENT,
            constants: RisConstants {
                men: edition.constants(osl_domain::Gender::M).into(),
                women: edition.constants(osl_domain::Gender::F).into(),
            },
        }
    }
}

/// A result scored with the RIS formula.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RisPerformanceResponse {
    /// Unique identifier for the entry.
    pub participant_id: uuid::Uuid,
    /// Athlete full name.
    pub athlete_name: String,
    /// Athlete URL slug.
    pub athlete_slug: String,
    /// Competition name.
    pub competition_name: String,
    /// Competition URL slug.
    pub competition_slug: String,
    /// Competition start date.
    pub competition_date: chrono::NaiveDate,
    /// Bodyweight in kilograms.
    pub bodyweight: Decimal,
    /// Total in kilograms.
    pub total: Decimal,
}

/// Computed MPDS results by gender, lightest first.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RisDistributionResponse {
    /// Results for men.
    pub men: Vec<RisPerformanceResponse>,
    /// Results for women.
    pub women: Vec<RisPerformanceResponse>,
}
