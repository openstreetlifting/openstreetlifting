use chrono::{NaiveDate, NaiveDateTime};
use osl_domain::{FormulaConstants, RisFormula};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RisScoreResponse {
    pub formula_year: i32,
    pub ris_score: Decimal,
    pub bodyweight: Decimal,
    pub total_weight: Decimal,
    pub computed_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RisFormulaResponse {
    pub formula_id: Uuid,
    pub year: i32,
    pub is_current: bool,
    pub effective_from: NaiveDate,
    pub effective_until: Option<NaiveDate>,
    pub constants: RisConstants,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RisConstants {
    pub men: GenderConstants,
    pub women: GenderConstants,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GenderConstants {
    pub a: Decimal,
    pub k: Decimal,
    pub b: Decimal,
    pub v: Decimal,
    pub q: Decimal,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ComputeRisRequest {
    pub bodyweight: Decimal,
    pub total: Decimal,
    pub gender: String,
    pub formula_year: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ComputeRisResponse {
    pub ris_score: Decimal,
    pub formula_year: i32,
}

impl From<FormulaConstants> for GenderConstants {
    fn from(constants: FormulaConstants) -> Self {
        Self {
            a: constants.a,
            k: constants.k,
            b: constants.b,
            v: constants.v,
            q: constants.q,
        }
    }
}

impl From<RisFormula> for RisFormulaResponse {
    fn from(formula: RisFormula) -> Self {
        Self {
            formula_id: formula.formula_id,
            year: formula.year,
            is_current: formula.is_current,
            effective_from: formula.effective_from,
            effective_until: formula.effective_until,
            constants: RisConstants {
                men: formula.men.into(),
                women: formula.women.into(),
            },
        }
    }
}
