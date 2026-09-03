use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ScorableParticipant {
    pub participant_id: Uuid,
    pub bodyweight: Decimal,
    pub gender: String,
    pub total: Decimal,
}
