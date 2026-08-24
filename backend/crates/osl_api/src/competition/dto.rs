use chrono::NaiveDate;
use osl_db::projections::competition::{
    AttemptSummary, CategoryParticipants, CompetitionDetail, CompetitionListItem, Contest,
    LiftDetail as DbLiftDetail, ParticipantDetail as DbParticipantDetail,
};
use osl_db::rows::{
    athlete::AthleteRow, competition::CompetitionRow, competition_movement::CompetitionMovementRow,
    federation::FederationRow,
};
use osl_domain::WeightClass;
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
    /// Present only when requested via `?include=federation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation: Option<FederationInfo>,
    /// Present only when requested via `?include=movements`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movements: Option<Vec<MovementInfo>>,
    /// Present only when requested via `?include=results`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<CategoryDetail>>,
    /// How many lifters the meet recorded. Absent on a single competition,
    /// where the results themselves carry it.
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
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CategoryDetail {
    pub category: CategoryInfo,
    pub participants: Vec<ParticipantDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CategoryInfo {
    /// What the contest is called, e.g. `Elite Men -80kg`.
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<String>,
    pub gender: String,
    /// The class alone, e.g. `-80kg`.
    pub weight_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ParticipantDetail {
    pub athlete: AthleteInfo,
    pub bodyweight: Option<rust_decimal::Decimal>,
    pub rank: Option<i32>,
    pub ris_score: Option<rust_decimal::Decimal>,
    pub is_disqualified: bool,
    pub disqualified_reason: Option<String>,
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

impl From<CompetitionMovementRow> for MovementInfo {
    fn from(row: CompetitionMovementRow) -> Self {
        Self {
            movement_name: row.movement_name,
            display_order: row.display_order,
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
            is_disqualified: participant.is_disqualified,
            disqualified_reason: participant.disqualified_reason,
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
    /// Builds a list entry, attaching only the sections the caller asked for.
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
            response.movements = Some(movements.into_iter().map(Into::into).collect());
        }
        response
    }

    /// Builds a single competition, attaching only the sections the caller
    /// asked for.
    pub fn from_detail(detail: CompetitionDetail, include: &Include) -> Self {
        let CompetitionDetail {
            competition,
            federation,
            categories,
        } = detail;

        let mut response = Self::from(competition);
        if include.has("federation") {
            response.federation = Some(federation.into());
        }
        if include.has("results") {
            response.categories = Some(categories.into_iter().map(Into::into).collect());
        }
        response
    }
}
