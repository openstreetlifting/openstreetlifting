use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct RulebookRow {
    pub rulebook_id: Uuid,
    pub name: Option<String>,
    pub url: Option<String>,
}
