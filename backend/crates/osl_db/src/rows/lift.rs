use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct LiftRow {
    pub lift_id: Uuid,
    pub participant_id: Uuid,
    pub movement_name: String,
    pub max_weight: Decimal,
    pub equipment_setting: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
