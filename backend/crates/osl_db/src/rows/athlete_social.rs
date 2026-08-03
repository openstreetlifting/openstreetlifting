use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct AthleteSocialRow {
    pub athlete_social_id: Uuid,
    pub athlete_id: Uuid,
    pub social_id: Uuid,
    pub handle: String,
}
