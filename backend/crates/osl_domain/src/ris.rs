use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::error::Result;

/// A versioned RIS (Relative Index for Streetlifting) formula.
///
/// Formula: RIS = Total × 100 / (A + (K - A) / (1 + Q · e^(-B · (BW - v))))
///
/// Constants are stored per gender. The persisted shape lives in `osl_db`
/// as `RisFormulaVersionRow`, which flattens the two constant sets into
/// `men_*` / `women_*` columns.
#[derive(Debug, Clone)]
pub struct RisFormula {
    pub formula_id: Uuid,
    pub year: i32,
    pub effective_from: NaiveDate,
    pub effective_until: Option<NaiveDate>,
    pub is_current: bool,
    pub men: FormulaConstants,
    pub women: FormulaConstants,
}

#[derive(Debug, Clone, Copy)]
pub struct FormulaConstants {
    pub a: Decimal,
    pub k: Decimal,
    pub b: Decimal,
    pub v: Decimal,
    pub q: Decimal,
}

impl RisFormula {
    /// Unknown genders fall back to the men's constants, preserving the
    /// behaviour the importer and API have relied on.
    pub fn constants_for_gender(&self, gender: &str) -> FormulaConstants {
        match gender.to_uppercase().as_str() {
            "F" | "FEMALE" | "WOMEN" => self.women,
            _ => self.men,
        }
    }
}

pub fn compute_ris(
    bodyweight: Decimal,
    total: Decimal,
    gender: &str,
    formula: &RisFormula,
) -> Result<Decimal> {
    let constants = formula.constants_for_gender(gender);

    let bw_minus_v = bodyweight - constants.v;
    let exp_arg = -constants.b * bw_minus_v;

    let exp_term = decimal_exp(exp_arg);
    let denominator_fraction =
        (constants.k - constants.a) / (Decimal::ONE + constants.q * exp_term);
    let denominator = constants.a + denominator_fraction;

    let ris_score = (total * Decimal::from(100)) / denominator;

    Ok(ris_score.round_dp(2))
}

fn decimal_exp(x: Decimal) -> Decimal {
    let x_f64: f64 = x.to_string().parse().unwrap_or(0.0);
    let result = x_f64.exp();
    Decimal::from_f64_retain(result).unwrap_or(Decimal::ONE)
}
