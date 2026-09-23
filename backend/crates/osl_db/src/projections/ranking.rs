use chrono::NaiveDate;
use osl_domain::{Gender, RisSource};
use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

use crate::params::RankingMovement;

#[derive(Debug, FromRow)]
pub struct RankingRow {
    pub rank: i64,
    pub athlete_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub slug: String,
    pub country: String,
    pub gender: Gender,
    pub instagram_handle: Option<String>,
    pub bodyweight: Option<Decimal>,
    pub division: Option<String>,
    pub weight_class_min: Option<Decimal>,
    pub weight_class_max: Option<Decimal>,
    pub competition_id: Uuid,
    pub competition_name: String,
    pub competition_slug: String,
    pub start_date: Option<NaiveDate>,
    pub federation_name: String,
    pub federation_abbreviation: Option<String>,
    /// Absent when the movement result is unknown or was not contested.
    pub muscleup: Option<Decimal>,
    pub pullup: Option<Decimal>,
    pub dips: Option<Decimal>,
    pub squat: Option<Decimal>,
    /// The stored event total, supplied by the source or prepared from complete lifts.
    /// Compare totals only within the same event.
    pub total: Option<Decimal>,
    pub event_code: Option<String>,
    pub ris_score: Option<Decimal>,
    pub ris_source: Option<RisSource>,
}

#[derive(Debug, FromRow)]
pub struct AthleteMetricStandingRow {
    pub metric: RankingMovement,
    pub value: Decimal,
    pub weight_class_min: Option<Decimal>,
    pub weight_class_max: Option<Decimal>,
    pub global_place: i64,
    pub global_field: i64,
    pub country: String,
    pub country_place: i64,
    pub country_field: i64,
}
