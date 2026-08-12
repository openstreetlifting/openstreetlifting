use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct RankingRow {
    pub rank: i64,
    pub athlete_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub slug: String,
    pub country: String,
    pub gender: String,
    pub bodyweight: Option<Decimal>,
    pub competition_id: Uuid,
    pub competition_name: String,
    pub start_date: Option<NaiveDate>,
    /// Absent when the meet did not contest the movement, rather than zero.
    pub muscleup: Option<Decimal>,
    pub pullup: Option<Decimal>,
    pub dips: Option<Decimal>,
    pub squat: Option<Decimal>,
    /// The sum of what this athlete contested, so it only compares with
    /// another total from the same event.
    pub total: Option<Decimal>,
    pub event_code: Option<String>,
    pub ris_score: Option<Decimal>,
    pub ris_source: Option<String>,
}
