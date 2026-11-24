use chrono::NaiveDate;
use osl_db::projections::competition::{
    AttemptSummary, CategoryParticipants, Contest, LiftDetail as DbLiftDetail,
    ParticipantDetail as DbParticipantDetail,
};
use osl_db::rows::{
    athlete::AthleteRow, competition::CompetitionRow, competition_movement::CompetitionMovementRow,
    federation::FederationRow,
};
use osl_domain::{
    AthleteStatus, CompetitionStatus, Gender, Movement, RecordMark, RisSource, WeightClass,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Optional competition section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompetitionInclude {
    Federation,
    Results,
    Movements,
    Editions,
}

/// Optional section on each competition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompetitionListInclude {
    Federation,
    Movements,
}

/// A competition.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompetitionResponse {
    /// Unique identifier for the competition.
    pub competition_id: Uuid,
    /// Competition name.
    pub name: String,
    /// Time the competition was added.
    pub created_at: chrono::NaiveDateTime,
    /// URL slug.
    pub slug: String,
    /// Competition status.
    pub status: CompetitionStatus,
    /// Metric the competition ranks on.
    #[schema(required = true)]
    pub rank_by: Option<osl_domain::RankBy>,
    /// Unique identifier for the organising federation.
    pub federation_id: Uuid,
    /// City.
    #[schema(required = true)]
    pub city: Option<String>,
    /// Venue.
    #[schema(required = true)]
    pub venue: Option<String>,
    /// Region.
    #[schema(required = true)]
    pub region: Option<String>,
    /// Two-letter country code (ISO 3166-1 alpha-2).
    #[schema(required = true)]
    pub country: Option<String>,
    /// Start date.
    #[schema(required = true)]
    pub start_date: Option<NaiveDate>,
    /// End date.
    #[schema(required = true)]
    pub end_date: Option<NaiveDate>,
    /// Organising federation. Included with `include=federation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub federation: Option<FederationInfo>,
    /// Movements contested. Included with `include=movements`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub movements: Option<Vec<MovementInfo>>,
    /// Movement letters, such as `MPDS`. Included with `include=movements`.
    #[schema(required = true)]
    pub event_code: Option<String>,
    /// Results by category. Included with `include=results`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub categories: Option<Vec<CategoryDetail>>,
    /// Editions of the same series, oldest first. Included with `include=editions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub editions: Option<Vec<CompetitionEdition>>,
    /// Number of entries. List endpoint only.
    #[schema(required = true)]
    pub lifter_count: Option<i64>,
}

/// An edition of a competition series.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompetitionEdition {
    /// URL slug.
    pub slug: String,
    /// Edition name.
    pub name: String,
    /// Start date.
    pub start_date: NaiveDate,
}

impl From<osl_db::projections::competition::CompetitionEdition> for CompetitionEdition {
    fn from(edition: osl_db::projections::competition::CompetitionEdition) -> Self {
        Self {
            slug: edition.slug,
            name: edition.name,
            start_date: edition.start_date,
        }
    }
}

/// A federation.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(as = CompetitionFederation)]
pub struct FederationInfo {
    /// Unique identifier for the federation.
    pub federation_id: Uuid,
    /// Full name.
    pub name: String,
    /// Short name.
    #[schema(required = true)]
    pub abbreviation: Option<String>,
    /// Two-letter country code (ISO 3166-1 alpha-2).
    #[schema(required = true)]
    pub country: Option<String>,
}

/// A contested movement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MovementInfo {
    /// Movement.
    pub movement_name: Movement,
    /// Display position.
    #[schema(required = true)]
    pub display_order: Option<i32>,
    /// Letter: `M`, `P`, `D` or `S`.
    pub code: String,
}

/// Results for one category.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CategoryDetail {
    /// Category.
    pub category: CategoryInfo,
    /// Entries, best placed first.
    pub participants: Vec<ParticipantDetail>,
}

/// A category.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CategoryInfo {
    /// Category name, such as `Men -80kg`.
    pub name: String,
    /// Division.
    #[schema(required = true)]
    pub division: Option<String>,
    /// Category gender. `MX` when mixed.
    pub gender: Gender,
    /// Weight class, such as `-80kg`. Empty when open.
    pub weight_class: String,
    /// Ranking metric for this contest.
    pub rank_by: osl_domain::RankBy,
}

/// An entry in a category.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ParticipantDetail {
    /// Unique identifier for the entry.
    pub participant_id: Uuid,
    /// Athlete.
    pub athlete: AthleteInfo,
    /// Bodyweight in kilograms.
    #[schema(required = true)]
    pub bodyweight: Option<rust_decimal::Decimal>,
    /// Age at the competition.
    #[schema(required = true)]
    pub age: Option<i16>,
    /// Place in the category.
    #[schema(required = true)]
    pub rank: Option<i32>,
    /// RIS points.
    #[schema(required = true)]
    pub ris_score: Option<rust_decimal::Decimal>,
    /// Origin of the RIS.
    #[schema(required = true)]
    pub ris_source: Option<RisSource>,
    /// Participation status.
    pub status: AthleteStatus,
    /// Published reason for the status.
    #[schema(required = true)]
    pub status_reason: Option<String>,
    /// One entry per movement.
    pub lifts: Vec<LiftDetail>,
    /// Total in kilograms.
    #[schema(required = true)]
    pub total: Option<rust_decimal::Decimal>,
    /// `WR` or `NR` when this total is the current world or national record for its weight class.
    #[schema(required = true)]
    pub total_record: Option<RecordMark>,
}

/// An athlete.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(as = CompetitionAthlete)]
pub struct AthleteInfo {
    /// Unique identifier for the athlete.
    pub athlete_id: uuid::Uuid,
    /// First name. Empty when redacted.
    pub first_name: String,
    /// Last name, or a placeholder when redacted.
    pub last_name: String,
    /// Gender the athlete competes in.
    pub gender: Gender,
    /// Two-letter country code (ISO 3166-1 alpha-2).
    #[schema(required = true)]
    pub country: Option<String>,
    /// URL slug.
    pub slug: String,
}

/// A lift.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LiftDetail {
    /// Movement.
    pub movement_name: Movement,
    /// Best successful attempt in kilograms. `null` when every attempt failed.
    #[schema(required = true)]
    pub best_weight: Option<rust_decimal::Decimal>,
    /// `WR` or `NR` when this lift is the current world or national record for its weight class.
    #[schema(required = true)]
    pub record: Option<RecordMark>,
    /// Attempts, in order.
    pub attempts: Vec<AttemptInfo>,
}

/// An attempt.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AttemptInfo {
    /// Attempt number, from 1.
    pub attempt_number: i16,
    /// Weight in kilograms.
    pub weight: rust_decimal::Decimal,
    /// Whether the attempt succeeded.
    pub is_successful: bool,
}

impl From<CompetitionRow> for CompetitionResponse {
    fn from(comp: CompetitionRow) -> Self {
        Self {
            competition_id: comp.competition_id,
            name: comp.name,
            created_at: comp.created_at,
            slug: comp.slug,
            status: comp.status,
            rank_by: comp.rank_by,
            federation_id: comp.federation_id,
            city: comp.city,
            venue: comp.venue,
            region: comp.region,
            country: comp.country,
            start_date: comp.start_date,
            end_date: comp.end_date,
            federation: None,
            movements: None,
            event_code: None,
            categories: None,
            editions: None,
            lifter_count: None,
        }
    }
}

impl From<FederationRow> for FederationInfo {
    fn from(row: FederationRow) -> Self {
        Self {
            federation_id: row.federation_id,
            name: row.name,
            abbreviation: row.abbreviation,
            country: row.country,
        }
    }
}

fn event_code(movements: &[MovementInfo]) -> Option<String> {
    if movements.is_empty() {
        return None;
    }

    Some(
        movements
            .iter()
            .map(|movement| movement.code.as_str())
            .collect(),
    )
}

impl From<CompetitionMovementRow> for MovementInfo {
    fn from(row: CompetitionMovementRow) -> Self {
        let movement = row.movement_name;

        Self {
            movement_name: movement,
            display_order: row.display_order,
            code: movement.code().to_string(),
        }
    }
}

impl From<Contest> for CategoryInfo {
    fn from(contest: Contest) -> Self {
        Self {
            name: osl_domain::category_label(
                contest.division.as_deref(),
                contest.gender,
                contest.weight_class_min,
                contest.weight_class_max,
            ),
            division: contest.division,
            gender: contest.gender,
            weight_class: WeightClass::label(contest.weight_class_min, contest.weight_class_max),
            rank_by: contest.rank_by,
        }
    }
}

impl From<AthleteRow> for AthleteInfo {
    fn from(row: AthleteRow) -> Self {
        Self {
            athlete_id: row.athlete_id,
            first_name: row.first_name,
            last_name: row.last_name,
            gender: row.gender,
            country: row.country,
            slug: row.slug,
        }
    }
}

impl From<AttemptSummary> for AttemptInfo {
    fn from(row: AttemptSummary) -> Self {
        Self {
            attempt_number: row.attempt_number,
            weight: row.weight,
            is_successful: row.is_successful,
        }
    }
}

impl From<DbLiftDetail> for LiftDetail {
    fn from(lift: DbLiftDetail) -> Self {
        Self {
            movement_name: lift.movement_name,
            best_weight: lift.best_weight,
            record: lift.record,
            attempts: lift.attempts.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<DbParticipantDetail> for ParticipantDetail {
    fn from(participant: DbParticipantDetail) -> Self {
        Self {
            participant_id: participant.participant_id,
            athlete: participant.athlete.into(),
            bodyweight: participant.bodyweight,
            age: participant.age,
            rank: participant.rank,
            ris_score: participant.ris_score,
            ris_source: participant.ris_source,
            status: participant.status,
            status_reason: participant.status_reason,
            lifts: participant.lifts.into_iter().map(Into::into).collect(),
            total: participant.total,
            total_record: participant.total_record,
        }
    }
}

impl From<CategoryParticipants> for CategoryDetail {
    fn from(entry: CategoryParticipants) -> Self {
        Self {
            category: entry.category.into(),
            participants: entry.participants.into_iter().map(Into::into).collect(),
        }
    }
}

impl CompetitionResponse {
    pub fn set_movements(&mut self, rows: Vec<CompetitionMovementRow>) {
        let movements: Vec<MovementInfo> = rows.into_iter().map(Into::into).collect();
        self.event_code = event_code(&movements);
        self.movements = Some(movements);
    }

    pub fn set_results(&mut self, categories: Vec<CategoryParticipants>) {
        self.categories = Some(categories.into_iter().map(Into::into).collect());
    }
}

/// Filter values that return competitions.
#[derive(Debug, Serialize, ToSchema)]
pub struct CompetitionFacets {
    /// Federation names.
    pub federations: Vec<String>,
    /// Country codes.
    pub countries: Vec<String>,
    /// Years, newest first.
    pub years: Vec<i32>,
    /// Event codes, such as `MPDS`.
    pub formats: Vec<String>,
}
