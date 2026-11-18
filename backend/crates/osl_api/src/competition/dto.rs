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
///
/// `federation` adds `federation`, `results` adds `categories`, `movements` adds `movements` and
/// fills `event_code`, and `editions` adds `editions`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompetitionInclude {
    Federation,
    Results,
    Movements,
    Editions,
}

/// Optional section on each listed competition. `federation` adds `federation`, and `movements`
/// adds `movements` and fills `event_code`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompetitionListInclude {
    Federation,
    Movements,
}

/// A competition. Sections appear only when requested with `include`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompetitionResponse {
    /// Stable identifier of the competition.
    pub competition_id: Uuid,
    /// Name of the competition.
    pub name: String,
    /// When the competition was added to the archive.
    pub created_at: chrono::NaiveDateTime,
    /// URL identifier of the competition.
    pub slug: String,
    /// Current state of the competition.
    pub status: CompetitionStatus,
    /// Metric the competition ranks on, when known.
    #[schema(required = true)]
    pub scoring: Option<osl_domain::Scoring>,
    /// Identifier of the organising federation.
    pub federation_id: Uuid,
    /// City, when known.
    #[schema(required = true)]
    pub city: Option<String>,
    /// Venue, when known.
    #[schema(required = true)]
    pub venue: Option<String>,
    /// Region, when known.
    #[schema(required = true)]
    pub region: Option<String>,
    /// Host country as an ISO 3166-1 alpha-2 country code, such as `FR`.
    #[schema(required = true)]
    pub country: Option<String>,
    /// First day, when known.
    #[schema(required = true)]
    pub start_date: Option<NaiveDate>,
    /// Last day, when known.
    #[schema(required = true)]
    pub end_date: Option<NaiveDate>,
    /// Organising federation. Requires `include=federation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub federation: Option<FederationInfo>,
    /// Movements contested, in display order. Requires `include=movements`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub movements: Option<Vec<MovementInfo>>,
    /// Movement letters such as `MPDS` or `PD`. `null` unless `include=movements` is requested.
    #[schema(required = true)]
    pub event_code: Option<String>,
    /// Results by category. Requires `include=results`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub categories: Option<Vec<CategoryDetail>>,
    /// Editions of the same series, oldest first. Requires `include=editions` and is empty without
    /// a series.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub editions: Option<Vec<CompetitionEdition>>,
    /// Entries in the competition. Returned by the list endpoint only.
    #[schema(required = true)]
    pub lifter_count: Option<i64>,
}

/// One edition of a competition series.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompetitionEdition {
    /// URL identifier of the edition.
    pub slug: String,
    /// Name of the edition.
    pub name: String,
    /// First day of the edition.
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

/// The federation organising a competition.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(as = CompetitionFederation)]
pub struct FederationInfo {
    /// Stable identifier of the federation.
    pub federation_id: Uuid,
    /// Full name.
    pub name: String,
    /// Short name, when known.
    #[schema(required = true)]
    pub abbreviation: Option<String>,
    /// Federation's ISO 3166-1 alpha-2 country code, such as `FR`.
    #[schema(required = true)]
    pub country: Option<String>,
}

/// A movement contested at a competition.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MovementInfo {
    /// The movement.
    pub movement_name: Movement,
    /// Position when listing the movements.
    #[schema(required = true)]
    pub display_order: Option<i32>,
    /// One-letter code: `M`, `P`, `D` or `S`.
    pub code: String,
}

/// Results for one category.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CategoryDetail {
    /// The category.
    pub category: CategoryInfo,
    /// Entries, best placed first.
    pub participants: Vec<ParticipantDetail>,
}

/// A contested category.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CategoryInfo {
    /// Display name, such as `Men -80kg`.
    pub name: String,
    /// Division, when the competition has divisions.
    #[schema(required = true)]
    pub division: Option<String>,
    /// Sex of the category, or `MX` when mixed.
    pub gender: Gender,
    /// Weight class label, such as `-80kg`. Empty without a weight class.
    pub weight_class: String,
}

/// One athlete's entry in a category.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ParticipantDetail {
    /// Stable identifier of this entry.
    pub participant_id: Uuid,
    /// The athlete.
    pub athlete: AthleteInfo,
    /// Bodyweight in kilograms, when known.
    #[schema(required = true)]
    pub bodyweight: Option<rust_decimal::Decimal>,
    /// Age at the competition, when known.
    #[schema(required = true)]
    pub age: Option<i16>,
    /// Place in the category, or `null` when unplaced.
    #[schema(required = true)]
    pub rank: Option<i32>,
    /// RIS points, when available.
    #[schema(required = true)]
    pub ris_score: Option<rust_decimal::Decimal>,
    /// Where the RIS comes from, when there is one.
    #[schema(required = true)]
    pub ris_source: Option<RisSource>,
    /// Whether the athlete competed.
    pub status: AthleteStatus,
    /// Why the athlete did not compete, when published.
    #[schema(required = true)]
    pub status_reason: Option<String>,
    /// Results for each movement.
    pub lifts: Vec<LiftDetail>,
    /// Total in kilograms, or `null` when incomplete or not competed.
    #[schema(required = true)]
    pub total: Option<rust_decimal::Decimal>,
    /// `WR` or `NR` when this total is the current world or national record for its weight class.
    #[schema(required = true)]
    pub total_record: Option<RecordMark>,
}

/// The athlete behind a competition entry.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(as = CompetitionAthlete)]
pub struct AthleteInfo {
    /// Stable identifier of the athlete.
    pub athlete_id: uuid::Uuid,
    /// First name. Empty for redacted athletes.
    pub first_name: String,
    /// Last name, or a placeholder such as `Redacted Athlete #1` for redacted athletes.
    pub last_name: String,
    /// Sex the athlete competes under.
    pub gender: Gender,
    /// Athlete's ISO 3166-1 alpha-2 country code, such as `FR`.
    #[schema(required = true)]
    pub country: Option<String>,
    /// URL identifier of the athlete.
    pub slug: String,
}

/// An entry's result for one movement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LiftDetail {
    /// The movement.
    pub movement_name: Movement,
    /// Best successful attempt in kilograms, or `null` when no attempt succeeded.
    #[schema(required = true)]
    pub best_weight: Option<rust_decimal::Decimal>,
    /// `WR` or `NR` when this lift is the current world or national record for its weight class.
    #[schema(required = true)]
    pub record: Option<RecordMark>,
    /// Attempts in the order they were taken.
    pub attempts: Vec<AttemptInfo>,
}

/// One attempt at a lift.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AttemptInfo {
    /// Attempt number, starting at 1.
    pub attempt_number: i16,
    /// Weight attempted in kilograms.
    pub weight: rust_decimal::Decimal,
    /// Whether the attempt was successful.
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
            scoring: comp.scoring,
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

/// Filter values that still return competitions.
#[derive(Debug, Serialize, ToSchema)]
pub struct CompetitionFacets {
    /// Federation names.
    pub federations: Vec<String>,
    /// Country codes.
    pub countries: Vec<String>,
    /// Years, newest first.
    pub years: Vec<i32>,
    /// Event codes such as `MPDS`.
    pub formats: Vec<String>,
}
