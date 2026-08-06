use chrono::NaiveDateTime;
use osl_db::params::{AthleteUpdate, NewAthlete};
use osl_db::projections::athlete::{AthleteCompetitionRow, AthleteDetail, PersonalRecordRow};
use osl_db::rows::athlete::AthleteRow;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Response containing basic athlete information
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteResponse {
    pub athlete_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub slug: String,
    pub gender: String,
    pub nationality: Option<String>,
    pub country: String,
    pub profile_picture_url: Option<String>,
    pub created_at: NaiveDateTime,
}

/// Detailed athlete response with competition history
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteDetailResponse {
    pub athlete_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub slug: String,
    pub gender: String,
    pub nationality: Option<String>,
    pub country: String,
    pub profile_picture_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub competitions: Vec<AthleteCompetitionSummary>,
    pub personal_records: Vec<PersonalRecord>,
    pub total_competitions: i64,
}

/// Summary of athlete's performance in a competition
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteCompetitionSummary {
    pub competition_id: Uuid,
    pub competition_name: String,
    pub competition_slug: String,
    pub competition_date: Option<chrono::NaiveDate>,
    pub category_name: String,
    pub rank: Option<i32>,
    pub total: rust_decimal::Decimal,
    pub ris_score: Option<rust_decimal::Decimal>,
    pub is_disqualified: bool,
}

/// Personal record for a specific movement
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PersonalRecord {
    pub movement_name: String,
    pub max_weight: rust_decimal::Decimal,
    pub competition_name: String,
    pub competition_slug: String,
    pub date: Option<chrono::NaiveDate>,
}

/// Request payload for creating a new athlete
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAthleteRequest {
    pub first_name: String,

    pub last_name: String,

    pub gender: String,

    pub nationality: Option<String>,

    pub country: String,

    pub profile_picture_url: Option<String>,
}

/// Request payload for updating an existing athlete
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateAthleteRequest {
    pub first_name: Option<String>,

    pub last_name: Option<String>,

    pub gender: Option<String>,

    pub nationality: Option<String>,

    pub country: Option<String>,

    pub profile_picture_url: Option<String>,
}

// Validation helper

impl From<AthleteRow> for AthleteResponse {
    fn from(athlete: AthleteRow) -> Self {
        Self {
            athlete_id: athlete.athlete_id,
            first_name: athlete.first_name,
            last_name: athlete.last_name,
            slug: athlete.slug,
            gender: athlete.gender,
            nationality: athlete.nationality,
            country: athlete.country,
            profile_picture_url: athlete.profile_picture_url,
            created_at: athlete.created_at,
        }
    }
}

impl From<AthleteCompetitionRow> for AthleteCompetitionSummary {
    fn from(row: AthleteCompetitionRow) -> Self {
        Self {
            competition_id: row.competition_id,
            competition_name: row.competition_name,
            competition_slug: row.competition_slug,
            competition_date: row.competition_date,
            category_name: row.category_name,
            rank: row.rank,
            total: row.total,
            ris_score: row.ris_score,
            is_disqualified: row.is_disqualified,
        }
    }
}

impl From<PersonalRecordRow> for PersonalRecord {
    fn from(row: PersonalRecordRow) -> Self {
        Self {
            movement_name: row.movement_name,
            max_weight: row.max_weight,
            competition_name: row.competition_name,
            competition_slug: row.competition_slug,
            date: row.date,
        }
    }
}

impl From<AthleteDetail> for AthleteDetailResponse {
    fn from(detail: AthleteDetail) -> Self {
        let AthleteDetail {
            athlete,
            competitions,
            personal_records,
            total_competitions,
        } = detail;

        Self {
            athlete_id: athlete.athlete_id,
            first_name: athlete.first_name,
            last_name: athlete.last_name,
            slug: athlete.slug,
            gender: athlete.gender,
            nationality: athlete.nationality,
            country: athlete.country,
            profile_picture_url: athlete.profile_picture_url,
            created_at: athlete.created_at,
            competitions: competitions.into_iter().map(Into::into).collect(),
            personal_records: personal_records.into_iter().map(Into::into).collect(),
            total_competitions,
        }
    }
}

impl From<&CreateAthleteRequest> for NewAthlete {
    fn from(req: &CreateAthleteRequest) -> Self {
        Self {
            first_name: req.first_name.clone(),
            last_name: req.last_name.clone(),
            gender: req.gender.clone(),
            nationality: req.nationality.clone(),
            country: req.country.clone(),
            profile_picture_url: req.profile_picture_url.clone(),
        }
    }
}

impl From<&UpdateAthleteRequest> for AthleteUpdate {
    fn from(req: &UpdateAthleteRequest) -> Self {
        Self {
            first_name: req.first_name.clone(),
            last_name: req.last_name.clone(),
            gender: req.gender.clone(),
            nationality: req.nationality.clone(),
            country: req.country.clone(),
            profile_picture_url: req.profile_picture_url.clone(),
        }
    }
}
