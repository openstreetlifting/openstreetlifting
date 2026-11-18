use osl_domain::{Constants, Edition, Gender};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// One published edition of the RIS formula.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RisFormulaResponse {
    /// Year of the edition.
    pub year: i32,
    /// Whether rankings use this edition.
    pub is_current: bool,
    /// Constants for each sex.
    pub constants: RisConstants,
}

/// Formula constants by sex.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RisConstants {
    /// Constants for men.
    pub men: GenderConstants,
    /// Constants for women.
    pub women: GenderConstants,
}

/// Constants of the benchmark `a + (k - a) / (1 + q * exp(-b * (bodyweight - v)))`. RIS is `total *
/// 100 / benchmark`, rounded to two decimals.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GenderConstants {
    /// Lower asymptote of the benchmark curve.
    pub a: f64,
    /// Upper asymptote of the benchmark curve.
    pub k: f64,
    /// Growth rate of the curve.
    pub b: f64,
    /// Bodyweight offset of the curve, in kilograms.
    pub v: f64,
    /// Shape constant of the curve.
    pub q: f64,
}

/// Values used to calculate a RIS score.
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

/// A calculated RIS score.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ComputeRisResponse {
    /// RIS points, rounded to two decimals.
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
    /// Identifier of the competition entry.
    pub participant_id: uuid::Uuid,
    /// Full name of the athlete.
    pub athlete_name: String,
    /// URL identifier of the athlete.
    pub athlete_slug: String,
    /// Name of the competition.
    pub competition_name: String,
    /// URL identifier of the competition.
    pub competition_slug: String,
    /// Start date of the competition.
    pub competition_date: chrono::NaiveDate,
    /// Bodyweight in kilograms.
    pub bodyweight: Decimal,
    /// Total in kilograms.
    pub total: Decimal,
}

/// Computed MPDS results by sex, lightest bodyweight first.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RisDistributionResponse {
    /// Results by men.
    pub men: Vec<RisPerformanceResponse>,
    /// Results by women.
    pub women: Vec<RisPerformanceResponse>,
}
