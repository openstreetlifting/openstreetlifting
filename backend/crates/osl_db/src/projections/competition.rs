use osl_domain::Gender;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::rows::{
    athlete::AthleteRow, competition::CompetitionRow, competition_movement::CompetitionMovementRow,
    federation::FederationRow,
};

/// A competition and how many lifters it recorded. The count is what tells a
/// reader a competition has results behind it, so it is carried by the list itself
/// rather than looked up per row.
#[derive(Debug, FromRow)]
pub struct CompetitionSummaryRow {
    #[sqlx(flatten)]
    pub competition: CompetitionRow,
    pub lifter_count: i64,
}

#[derive(Debug)]
pub struct CompetitionListItem {
    pub competition: CompetitionRow,
    pub federation: FederationRow,
    pub movements: Vec<CompetitionMovementRow>,
    pub lifter_count: i64,
}

#[derive(Debug)]
pub struct CompetitionDetail {
    pub competition: CompetitionRow,
    pub federation: FederationRow,
    pub movements: Vec<CompetitionMovementRow>,
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
    pub ris_source: Option<String>,
    pub status: String,
    pub status_reason: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptSummary {
    pub attempt_number: i16,
    pub weight: Decimal,
    pub is_successful: bool,
}
