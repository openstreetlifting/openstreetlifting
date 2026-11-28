use osl_domain::Gender;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct AthleteRow {
    pub athlete_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub native_name: Option<String>,
    pub gender: Gender,
    pub created_at: chrono::NaiveDateTime,
    pub country: Option<String>,
    pub profile_picture_url: Option<String>,
    pub slug: String,
    #[sqlx(default)]
    pub slug_history: sqlx::types::Json<Vec<String>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct AthleteSearchRow {
    pub first_name: String,
    pub last_name: String,
    pub gender: Gender,
    pub country: Option<String>,
    pub slug: String,
    pub rankings: Vec<String>,
    pub total_items: i64,
}
