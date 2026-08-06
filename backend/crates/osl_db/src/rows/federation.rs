use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct FederationRow {
    pub federation_id: Uuid,
    pub name: String,
    pub rulebook_id: Option<Uuid>,
    pub country: Option<String>,
    pub abbreviation: Option<String>,
}
