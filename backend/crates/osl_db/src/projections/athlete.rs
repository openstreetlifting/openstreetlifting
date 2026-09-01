use osl_domain::Gender;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::projections::competition::AttemptSummary;
use crate::rows::athlete::AthleteRow;

/// The best an athlete made on one movement at one competition. A row with no
/// weight is a movement they contested and never made, which the athlete page
/// reads differently from a movement the meet never ran.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AthleteLiftRow {
    pub movement_name: String,
    pub best_weight: Option<Decimal>,
    pub attempts: Vec<AttemptSummary>,
}

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
    pub ris_source: Option<String>,
    pub status: String,
    pub event_code: Option<String>,
    pub lifts: Vec<AthleteLiftRow>,
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
