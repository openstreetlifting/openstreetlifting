use osl_domain::Gender;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ScorableParticipant {
    pub participant_id: Uuid,
    pub bodyweight: Decimal,
    pub gender: Gender,
    pub total: Decimal,
}
