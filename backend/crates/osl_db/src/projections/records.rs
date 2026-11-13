use chrono::NaiveDate;
use osl_domain::Gender;
use rust_decimal::Decimal;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct RecordRow {
    pub movement_name: String,
    pub gender: Gender,
    pub weight_class: Option<String>,
    pub rank: i64,
    pub first_name: String,
    pub last_name: String,
    pub slug: String,
    pub country: Option<String>,
    pub weight: Decimal,
    pub date: NaiveDate,
    pub competition_name: String,
    pub competition_slug: String,
}
