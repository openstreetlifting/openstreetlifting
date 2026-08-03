use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct AttemptRow {
    pub attempt_id: Uuid,
    pub lift_id: Uuid,
    pub attempt_number: i16,
    pub weight: Decimal,
    pub is_successful: bool,
    pub passing_judges: Option<i16>,
    pub no_rep_reason: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub created_by: Option<String>,
}
