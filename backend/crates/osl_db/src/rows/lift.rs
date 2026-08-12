use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct LiftRow {
    pub lift_id: Uuid,
    pub participant_id: Uuid,
    pub movement_name: String,
    /// Best successful attempt. 0 is a bodyweight-only lift, and None means
    /// the movement was contested with no attempt succeeding.
    pub max_weight: Option<Decimal>,
    pub equipment_setting: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
