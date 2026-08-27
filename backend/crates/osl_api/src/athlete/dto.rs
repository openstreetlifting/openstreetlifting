use chrono::NaiveDateTime;
use osl_db::projections::athlete::{AthleteCompetitionRow, AthleteDetail, PersonalRecordRow};
use osl_db::rows::athlete::AthleteRow;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::shared::query::Include;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteResponse {
    pub athlete_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub slug: String,
    pub gender: String,
    pub country: String,
    pub profile_picture_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instagram_handle: Option<String>,
    pub created_at: NaiveDateTime,
    /// Present only when requested via `?include=competitions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competitions: Option<Vec<AthleteCompetitionSummary>>,
    /// Present only when requested via `?include=records`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_records: Option<Vec<PersonalRecord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_competitions: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteCompetitionSummary {
    pub competition_id: Uuid,
    pub competition_name: String,
    pub competition_slug: String,
    pub competition_date: Option<chrono::NaiveDate>,
    pub category_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<String>,
    pub rank: Option<i32>,
    pub total: Option<rust_decimal::Decimal>,
    pub ris_score: Option<rust_decimal::Decimal>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PersonalRecord {
    pub movement_name: String,
    pub max_weight: rust_decimal::Decimal,
    pub competition_name: String,
    pub competition_slug: String,
    pub date: Option<chrono::NaiveDate>,
}

impl From<AthleteRow> for AthleteResponse {
    fn from(athlete: AthleteRow) -> Self {
        Self {
            athlete_id: athlete.athlete_id,
            first_name: athlete.first_name,
            last_name: athlete.last_name,
            slug: athlete.slug,
            gender: athlete.gender,
            country: athlete.country,
            profile_picture_url: athlete.profile_picture_url,
            instagram_handle: None,
            created_at: athlete.created_at,
            competitions: None,
            personal_records: None,
            total_competitions: None,
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
            category_name: osl_domain::category_label(
                None,
                row.category_gender,
                row.weight_class_min,
                row.weight_class_max,
            ),
            division: row.division,
            rank: row.rank,
            total: row.total,
            ris_score: row.ris_score,
            status: row.status,
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

impl AthleteResponse {
    /// Builds the response from a fully-loaded detail, keeping only the
    /// sections the caller asked for.
    pub fn from_detail(detail: AthleteDetail, include: &Include) -> Self {
        let AthleteDetail {
            athlete,
            competitions,
            personal_records,
            total_competitions,
            instagram_handle,
        } = detail;

        let mut response = Self::from(athlete);
        response.instagram_handle = instagram_handle;
        if include.has("competitions") {
            response.competitions = Some(competitions.into_iter().map(Into::into).collect());
            response.total_competitions = Some(total_competitions);
        }
        if include.has("records") {
            response.personal_records =
                Some(personal_records.into_iter().map(Into::into).collect());
        }
        response
    }
}
