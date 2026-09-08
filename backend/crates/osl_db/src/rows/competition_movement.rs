use osl_domain::Movement;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct CompetitionMovementRow {
    pub competition_id: Uuid,
    pub movement_name: Movement,
    pub display_order: Option<i32>,
}
