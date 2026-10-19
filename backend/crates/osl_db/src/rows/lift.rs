use osl_domain::Movement;
use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct LiftRow {
    pub lift_id: Uuid,
    pub participant_id: Uuid,
    pub movement_name: Movement,
    pub max_weight: Option<Decimal>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
