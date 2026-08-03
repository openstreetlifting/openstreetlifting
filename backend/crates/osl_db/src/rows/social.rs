use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct SocialRow {
    pub social_id: Uuid,
    pub name: String,
}
