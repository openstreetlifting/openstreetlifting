use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct CompetitionMovementRow {
    pub competition_id: Uuid,
    pub movement_name: String,
    pub is_required: bool,
    pub display_order: Option<i32>,
}
