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
    pub muscleup: Decimal,
    pub pullup: Decimal,
    pub dips: Decimal,
    pub squat: Decimal,
    pub total: Decimal,
    pub ris_score: Option<Decimal>,
    pub ris_source: Option<String>,
}
