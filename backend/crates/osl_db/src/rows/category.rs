use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct CategoryRow {
    pub category_id: Uuid,
    pub name: String,
    pub gender: String,
    pub weight_class_min: Option<Decimal>,
    pub weight_class_max: Option<Decimal>,
}
