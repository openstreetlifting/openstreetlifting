use rust_decimal::Decimal;

use crate::error::Result;
use crate::models::RisFormulaVersion;

pub fn compute_ris(
    bodyweight: Decimal,
    total: Decimal,
    gender: &str,
    formula: &RisFormulaVersion,
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
