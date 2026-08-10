use chrono::{NaiveDate, NaiveDateTime};
use osl_domain::{FormulaConstants, RisFormula};
use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct RisFormulaVersionRow {
    pub formula_id: Uuid,
    pub year: i32,
    pub effective_from: NaiveDate,
    pub effective_until: Option<NaiveDate>,
    pub is_current: bool,

    pub men_a: Decimal,
    pub men_k: Decimal,
    pub men_b: Decimal,
    pub men_v: Decimal,
    pub men_q: Decimal,

    pub women_a: Decimal,
    pub women_k: Decimal,
    pub women_b: Decimal,
    pub women_v: Decimal,
    pub women_q: Decimal,

    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
}

impl From<RisFormulaVersionRow> for RisFormula {
    fn from(row: RisFormulaVersionRow) -> Self {
        Self {
            formula_id: row.formula_id,
            year: row.year,
            effective_from: row.effective_from,
            effective_until: row.effective_until,
            is_current: row.is_current,
            men: FormulaConstants {
                a: row.men_a,
                k: row.men_k,
                b: row.men_b,
                v: row.men_v,
                q: row.men_q,
            },
            women: FormulaConstants {
                a: row.women_a,
                k: row.women_k,
                b: row.women_b,
                v: row.women_v,
                q: row.women_q,
            },
        }
    }
}
