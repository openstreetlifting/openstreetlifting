use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

use crate::rows::athlete::AthleteRow;

/// One competition an athlete took part in, with their total across lifts.
#[derive(Debug, FromRow)]
pub struct AthleteCompetitionRow {
    pub competition_id: Uuid,
    pub competition_name: String,
    pub competition_slug: String,
    pub competition_date: Option<chrono::NaiveDate>,
    pub category_name: String,
    pub rank: Option<i32>,
    pub total: Decimal,
    pub ris_score: Option<Decimal>,
    pub is_disqualified: bool,
}

/// Best lift per movement across an athlete's whole history.
#[derive(Debug, FromRow)]
pub struct PersonalRecordRow {
    pub movement_name: String,
    pub max_weight: Decimal,
    pub competition_name: String,
    pub competition_slug: String,
    pub date: Option<chrono::NaiveDate>,
}

/// Everything the detailed athlete endpoint needs, in one value.
#[derive(Debug)]
pub struct AthleteDetail {
    pub athlete: AthleteRow,
    pub competitions: Vec<AthleteCompetitionRow>,
    pub personal_records: Vec<PersonalRecordRow>,
    pub total_competitions: i64,
}
