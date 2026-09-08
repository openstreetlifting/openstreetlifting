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

/// A scored performance and the athlete and competition needed to identify it.
#[derive(Debug, Clone)]
pub struct ScoredPerformance {
    pub participant_id: Uuid,
    pub athlete_name: String,
    pub athlete_slug: String,
    pub competition_name: String,
    pub competition_slug: String,
    pub competition_date: chrono::NaiveDate,
    pub gender: String,
    pub bodyweight: Decimal,
    pub total: Decimal,
}
