use osl_domain::Gender;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::rows::{
    athlete::AthleteRow, competition::CompetitionRow, competition_movement::CompetitionMovementRow,
    federation::FederationRow,
};

#[derive(Debug)]
pub struct CompetitionListItem {
    pub competition: CompetitionRow,
    pub federation: FederationRow,
    pub movements: Vec<CompetitionMovementRow>,
}

#[derive(Debug)]
pub struct CompetitionDetail {
    pub competition: CompetitionRow,
    pub federation: FederationRow,
    pub categories: Vec<CategoryParticipants>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contest {
    pub weight_class_id: Uuid,
    pub division_id: Option<Uuid>,
    pub division: Option<String>,
    pub gender: Gender,
    pub weight_class_min: Option<Decimal>,
    pub weight_class_max: Option<Decimal>,
}

#[derive(Debug)]
pub struct CategoryParticipants {
    pub category: Contest,
    pub participants: Vec<ParticipantDetail>,
}

#[derive(Debug)]
pub struct ParticipantDetail {
    pub athlete: AthleteRow,
    pub bodyweight: Option<Decimal>,
    /// Placing within the contest, computed from the lifts.
    pub rank: Option<i32>,
    pub ris_score: Option<Decimal>,
    pub is_disqualified: bool,
    pub disqualified_reason: Option<String>,
    pub lifts: Vec<LiftDetail>,
    pub total: Option<Decimal>,
}

#[derive(Debug)]
pub struct LiftDetail {
    pub movement_name: String,
    /// Best successful attempt. 0 is a bodyweight-only lift, and None means
    /// the movement was contested with no attempt succeeding.
    pub best_weight: Option<Decimal>,
    pub attempts: Vec<AttemptSummary>,
}

#[derive(Debug)]
pub struct AttemptSummary {
    pub attempt_number: i16,
    pub weight: Decimal,
    pub is_successful: bool,
}
