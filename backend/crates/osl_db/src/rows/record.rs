use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct RecordRow {
    pub record_id: Uuid,
    pub record_type: String,
    pub weight_class_id: Uuid,
    pub division_id: Option<Uuid>,
    pub movement_name: String,
    pub athlete_id: Uuid,
    pub competition_id: Uuid,
    pub date_set: chrono::NaiveDate,
    pub weight: Decimal,
    pub gender: Option<String>,
}
