use chrono::NaiveDate;
use osl_domain::{Gender, Movement};
use rust_decimal::Decimal;
use sqlx::FromRow;

use super::ranking::SeriesOption;

#[derive(Debug, FromRow)]
pub struct RecordRow {
    pub movement_name: Movement,
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

#[derive(Debug)]
pub struct RecordFacets {
    pub countries: Vec<String>,
    pub federations: Vec<String>,
    pub series: Vec<SeriesOption>,
    pub years: Vec<i32>,
    pub custom_weight_classes: bool,
}
