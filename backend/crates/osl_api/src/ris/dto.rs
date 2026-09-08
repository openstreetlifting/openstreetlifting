use osl_domain::{Constants, Edition, Gender};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RisFormulaResponse {
    pub year: i32,
    pub is_current: bool,
    pub credit: String,
    pub constants: RisConstants,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RisConstants {
    pub men: GenderConstants,
    pub women: GenderConstants,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GenderConstants {
    pub a: f64,
    pub k: f64,
    pub b: f64,
    pub v: f64,
    pub q: f64,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ComputeRisRequest {
    pub bodyweight: Decimal,
    pub total: Decimal,
    #[serde(deserialize_with = "crate::shared::dto::from_str")]
    pub gender: Gender,
    pub formula_year: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ComputeRisResponse {
    pub ris_score: Decimal,
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
            credit: edition.credit().to_string(),
            constants: RisConstants {
                men: edition.constants(osl_domain::Gender::M).into(),
                women: edition.constants(osl_domain::Gender::F).into(),
            },
        }
    }
}

/// One plotted performance, including links back to its athlete and competition.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RisPerformanceResponse {
    pub participant_id: uuid::Uuid,
    pub athlete_name: String,
    pub athlete_slug: String,
    pub competition_name: String,
    pub competition_slug: String,
    pub competition_date: chrono::NaiveDate,
    pub bodyweight: Decimal,
    pub total: Decimal,
}

/// The archive's own performances, grouped by the reference curve they use.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RisDistributionResponse {
    pub men: Vec<RisPerformanceResponse>,
    pub women: Vec<RisPerformanceResponse>,
}
