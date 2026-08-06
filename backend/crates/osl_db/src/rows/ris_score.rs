use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct RisScoreHistoryRow {
    pub ris_score_id: Uuid,
    pub participant_id: Uuid,
    pub formula_id: Uuid,
    pub ris_score: Decimal,
    pub bodyweight: Decimal,
    pub total_weight: Decimal,
    pub computed_at: chrono::NaiveDateTime,
}
