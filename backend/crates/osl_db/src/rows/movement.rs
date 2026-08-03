use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct MovementRow {
    pub name: String,
    pub display_order: i32,
}
