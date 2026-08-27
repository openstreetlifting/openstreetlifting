use osl_domain::Gender;
use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

use crate::rows::athlete::AthleteRow;

#[derive(Debug, FromRow)]
pub struct AthleteCompetitionRow {
    pub competition_id: Uuid,
    pub competition_name: String,
    pub competition_slug: String,
    pub competition_date: Option<chrono::NaiveDate>,
    pub division: Option<String>,
    pub category_gender: Gender,
    pub weight_class_min: Option<Decimal>,
    pub weight_class_max: Option<Decimal>,
    pub rank: Option<i32>,
    pub total: Option<Decimal>,
    pub ris_score: Option<Decimal>,
    pub status: String,
}

#[derive(Debug, FromRow)]
pub struct PersonalRecordRow {
    pub movement_name: String,
    pub max_weight: Decimal,
    pub competition_name: String,
    pub competition_slug: String,
    pub date: Option<chrono::NaiveDate>,
}

#[derive(Debug)]
pub struct AthleteDetail {
    pub athlete: AthleteRow,
    pub competitions: Vec<AthleteCompetitionRow>,
    pub personal_records: Vec<PersonalRecordRow>,
    pub total_competitions: i64,
    pub instagram_handle: Option<String>,
}
