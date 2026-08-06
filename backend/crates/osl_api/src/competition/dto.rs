use chrono::NaiveDate;
use osl_db::params::{CompetitionUpdate, NewCompetition};
use osl_db::projections::competition::{
    AttemptSummary, CategoryParticipants, CompetitionDetail, CompetitionListItem,
    LiftDetail as DbLiftDetail, ParticipantDetail as DbParticipantDetail,
};
use osl_db::rows::{
    athlete::AthleteRow, category::CategoryRow, competition::CompetitionRow,
    competition_movement::CompetitionMovementRow, federation::FederationRow,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::shared::query::Include;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCompetitionRequest {
    pub name: String,
    pub slug: String,

    #[serde(default = "default_status")]
    pub status: String,
    pub federation_id: Uuid,
    pub venue: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub number_of_judge: Option<i16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateCompetitionRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub status: Option<String>,
    pub federation_id: Option<Uuid>,
    pub venue: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub number_of_judge: Option<i16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompetitionResponse {
    pub competition_id: Uuid,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub slug: String,
    pub status: String,
    pub federation_id: Uuid,
    pub venue: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub number_of_judge: Option<i16>,
    /// Present only when requested via `?include=federation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation: Option<FederationInfo>,
    /// Present only when requested via `?include=movements`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movements: Option<Vec<MovementInfo>>,
    /// Present only when requested via `?include=results`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<CategoryDetail>>,
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
    pub is_required: bool,
    pub display_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CategoryDetail {
    pub category: CategoryInfo,
    pub participants: Vec<ParticipantDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CategoryInfo {
    pub category_id: uuid::Uuid,
    pub name: String,
    pub gender: String,
    pub weight_class_min: Option<rust_decimal::Decimal>,
    pub weight_class_max: Option<rust_decimal::Decimal>,
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
    pub total: rust_decimal::Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteInfo {
    pub athlete_id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub nationality: Option<String>,
    pub country: String,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LiftDetail {
    pub movement_name: String,
    pub best_weight: rust_decimal::Decimal,
    pub attempts: Vec<AttemptInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AttemptInfo {
    pub attempt_number: i16,
    pub weight: rust_decimal::Decimal,
    pub is_successful: bool,
    pub passing_judges: Option<i16>,
    pub no_rep_reason: Option<String>,
}

fn default_status() -> String {
    "draft".to_string()
}

impl CreateCompetitionRequest {
    pub fn validate_dates(&self) -> Result<(), &'static str> {
        if self.end_date < self.start_date {
            return Err("End date must be on or after start date");
        }

        if let Some(judges) = self.number_of_judge
            && judges != 1
            && judges != 3
        {
            return Err("Number of judges must be 1 or 3");
        }

        Ok(())
    }
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
            venue: comp.venue,
            city: comp.city,
            country: comp.country,
            start_date: comp.start_date,
            end_date: comp.end_date,
            number_of_judge: comp.number_of_judge,
            federation: None,
            movements: None,
            categories: None,
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
            is_required: row.is_required,
            display_order: row.display_order,
        }
    }
}

impl From<CategoryRow> for CategoryInfo {
    fn from(row: CategoryRow) -> Self {
        Self {
            category_id: row.category_id,
            name: row.name,
            gender: row.gender,
            weight_class_min: row.weight_class_min,
            weight_class_max: row.weight_class_max,
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
            nationality: row.nationality,
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
            passing_judges: row.passing_judges,
            no_rep_reason: row.no_rep_reason,
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
        } = item;

        let mut response = Self::from(competition);
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

impl From<&CreateCompetitionRequest> for NewCompetition {
    fn from(req: &CreateCompetitionRequest) -> Self {
        Self {
            name: req.name.clone(),
            slug: req.slug.clone(),
            status: req.status.clone(),
            federation_id: req.federation_id,
            venue: req.venue.clone(),
            city: req.city.clone(),
            country: req.country.clone(),
            start_date: req.start_date,
            end_date: req.end_date,
            number_of_judge: req.number_of_judge,
        }
    }
}

impl From<&UpdateCompetitionRequest> for CompetitionUpdate {
    fn from(req: &UpdateCompetitionRequest) -> Self {
        Self {
            name: req.name.clone(),
            slug: req.slug.clone(),
            status: req.status.clone(),
            federation_id: req.federation_id,
            venue: req.venue.clone(),
            city: req.city.clone(),
            country: req.country.clone(),
            start_date: req.start_date,
            end_date: req.end_date,
            number_of_judge: req.number_of_judge,
        }
    }
}
