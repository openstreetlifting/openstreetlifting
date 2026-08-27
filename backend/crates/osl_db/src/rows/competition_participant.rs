use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct CompetitionParticipantRow {
    pub participant_id: Uuid,
    pub competition_id: Uuid,
    pub category_id: Uuid,
    pub athlete_id: Uuid,
    pub bodyweight: Option<Decimal>,
    pub rank: Option<i32>,
    pub status: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub status_reason: Option<String>,
    pub ris_score: Option<Decimal>,
}
