use rust_decimal::Decimal;

use crate::rows::{
    athlete::AthleteRow, category::CategoryRow, competition::CompetitionRow,
    competition_movement::CompetitionMovementRow, federation::FederationRow,
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

#[derive(Debug)]
pub struct CategoryParticipants {
    pub category: CategoryRow,
    pub participants: Vec<ParticipantDetail>,
}

#[derive(Debug)]
pub struct ParticipantDetail {
    pub athlete: AthleteRow,
    pub bodyweight: Option<Decimal>,
    /// Rank computed per category by the ranking window function
    pub rank: Option<i32>,
    pub ris_score: Option<Decimal>,
    pub is_disqualified: bool,
    pub disqualified_reason: Option<String>,
    pub lifts: Vec<LiftDetail>,
    pub total: Decimal,
}

#[derive(Debug)]
pub struct LiftDetail {
    pub movement_name: String,
    pub best_weight: Decimal,
    pub attempts: Vec<AttemptSummary>,
}

#[derive(Debug)]
pub struct AttemptSummary {
    pub attempt_number: i16,
    pub weight: Decimal,
    pub is_successful: bool,
    pub passing_judges: Option<i16>,
    pub no_rep_reason: Option<String>,
}
