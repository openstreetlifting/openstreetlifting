use chrono::NaiveDate;
use osl_db::projections::competition::{
    AttemptSummary, CategoryParticipants, CompetitionDetail, CompetitionListItem, Contest,
    LiftDetail as DbLiftDetail, ParticipantDetail as DbParticipantDetail,
};
use osl_db::rows::{
    athlete::AthleteRow, competition::CompetitionRow, competition_movement::CompetitionMovementRow,
    federation::FederationRow,
};
use osl_domain::{Movement, WeightClass};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::shared::query::Include;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompetitionResponse {
    pub competition_id: Uuid,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub slug: String,
    pub status: String,
    pub federation_id: Uuid,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation: Option<FederationInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movements: Option<Vec<MovementInfo>>,
    /// The event this competition ran, e.g. `MPDS`. Its letters are the
    /// movements it contested in display order, which is what gives one set of
    /// movements exactly one spelling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<CategoryDetail>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifter_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FederationInfo {
    pub federation_id: Uuid,
    pub name: String,
    pub abbreviation: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MovementInfo {
    pub movement_name: String,
    pub display_order: Option<i32>,
    /// The letter this movement contributes to an event code. Absent for a
    /// movement the domain does not know, which is the only honest answer:
    /// spelling one here would invent a code nothing can read back.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CategoryDetail {
    pub category: CategoryInfo,
    pub participants: Vec<ParticipantDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CategoryInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<String>,
    pub gender: String,
    pub weight_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ParticipantDetail {
    pub athlete: AthleteInfo,
    pub bodyweight: Option<rust_decimal::Decimal>,
    pub rank: Option<i32>,
    pub ris_score: Option<rust_decimal::Decimal>,
    /// `computed` when the score was worked out from a bodyweight and a
    /// total, `reported` when the source stated it and it cannot be
    /// restated on the current formula. Absent alongside a missing score.
    /// TOOD introduce a enum varient for this ?
    pub ris_source: Option<String>,
    pub status: String,
    pub status_reason: Option<String>,
    pub lifts: Vec<LiftDetail>,
    pub total: Option<rust_decimal::Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteInfo {
    pub athlete_id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub country: String,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LiftDetail {
    pub movement_name: String,
    /// Best successful attempt. Zero is a bodyweight-only lift, and absent
    /// means the movement was contested with no attempt succeeding.
    pub best_weight: Option<rust_decimal::Decimal>,
    pub attempts: Vec<AttemptInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AttemptInfo {
    pub attempt_number: i16,
    pub weight: rust_decimal::Decimal,
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
            federation_id: comp.federation_id,
            city: comp.city,
            region: comp.region,
            country: comp.country,
            start_date: comp.start_date,
            end_date: comp.end_date,
            federation: None,
            movements: None,
            event_code: None,
            categories: None,
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

/// The letters of the movements contested, in the order they are shown. A
/// movement the domain cannot name has no letter, and a code missing a letter
/// would name a different event, so the whole code is withheld rather than a
/// wrong one returned.
fn event_code(movements: &[MovementInfo]) -> Option<String> {
    if movements.is_empty() {
        return None;
    }

    movements
        .iter()
        .map(|movement| movement.code.as_deref())
        .collect::<Option<Vec<&str>>>()
        .map(|codes| codes.concat())
}

impl From<CompetitionMovementRow> for MovementInfo {
    fn from(row: CompetitionMovementRow) -> Self {
        let code =
            Movement::from_name(&row.movement_name).map(|movement| movement.code().to_string());

        Self {
            movement_name: row.movement_name,
            display_order: row.display_order,
            code,
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
            gender: contest.gender.as_str().to_string(),
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
            attempts: lift.attempts.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<DbParticipantDetail> for ParticipantDetail {
    fn from(participant: DbParticipantDetail) -> Self {
        Self {
            athlete: participant.athlete.into(),
            bodyweight: participant.bodyweight,
            rank: participant.rank,
            ris_score: participant.ris_score,
            ris_source: participant.ris_source,
            status: participant.status,
            status_reason: participant.status_reason,
            lifts: participant.lifts.into_iter().map(Into::into).collect(),
            total: participant.total,
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
    pub fn from_list_item(item: CompetitionListItem, include: &Include) -> Self {
        let CompetitionListItem {
            competition,
            federation,
            movements,
            lifter_count,
        } = item;

        let mut response = Self::from(competition);
        response.lifter_count = Some(lifter_count);
        if include.has("federation") {
            response.federation = Some(federation.into());
        }
        if include.has("movements") {
            let movements: Vec<MovementInfo> = movements.into_iter().map(Into::into).collect();
            response.event_code = event_code(&movements);
            response.movements = Some(movements);
        }
        response
    }

    pub fn from_detail(detail: CompetitionDetail, include: &Include) -> Self {
        let CompetitionDetail {
            competition,
            federation,
            movements,
            categories,
        } = detail;

        let mut response = Self::from(competition);
        if include.has("federation") {
            response.federation = Some(federation.into());
        }
        if include.has("movements") {
            let movements: Vec<MovementInfo> = movements.into_iter().map(Into::into).collect();
            response.event_code = event_code(&movements);
            response.movements = Some(movements);
        }
        if include.has("results") {
            response.categories = Some(categories.into_iter().map(Into::into).collect());
        }
        response
    }
}
