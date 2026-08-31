use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct AthleteRow {
    pub athlete_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub native_name: Option<String>,
    pub gender: String,
    pub created_at: chrono::NaiveDateTime,
    pub country: String,
    pub profile_picture_url: Option<String>,
    pub slug: String,
    #[sqlx(default)]
    pub slug_history: sqlx::types::Json<Vec<String>>,
}
