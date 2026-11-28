use osl_domain::{AthleteStatus, Gender};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct AthleteRow {
    pub athlete_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub native_name: Option<String>,
    pub gender: Gender,
    pub created_at: chrono::NaiveDateTime,
    pub country: Option<String>,
    pub profile_picture_url: Option<String>,
    pub slug: String,
    #[sqlx(default)]
    pub slug_history: sqlx::types::Json<Vec<String>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct AthleteSearchRow {
    pub first_name: String,
    pub last_name: String,
    pub gender: Gender,
    pub country: Option<String>,
    pub slug: String,
    pub rankings: Vec<String>,
    pub total_items: i64,
    pub competition_id: Option<Uuid>,
    pub competition_name: Option<String>,
    pub competition_slug: Option<String>,
    pub competition_date: Option<chrono::NaiveDate>,
    pub federation_name: Option<String>,
    pub federation_abbreviation: Option<String>,
    pub event_code: Option<String>,
    pub entry_status: Option<AthleteStatus>,
    pub entry_status_reason: Option<String>,
    pub entry_gender: Option<Gender>,
    pub weight_class_min: Option<rust_decimal::Decimal>,
    pub weight_class_max: Option<rust_decimal::Decimal>,
    pub muscleup: Option<rust_decimal::Decimal>,
    pub pullup: Option<rust_decimal::Decimal>,
    pub dips: Option<rust_decimal::Decimal>,
    pub squat: Option<rust_decimal::Decimal>,
}
