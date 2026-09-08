use chrono::NaiveDateTime;
use osl_db::params::RankingMovement;
use osl_db::projections::athlete::{
    AthleteCompetitionRow, AthleteDetail, AthleteLiftRow, AthleteStrengthRow, PersonalRecordRow,
};
use osl_db::projections::ranking::AthleteMetricStandingRow;
use osl_db::rows::athlete::AthleteRow;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::competition::dto::AttemptInfo;
use crate::shared::enums::{AthleteStatus, Gender, Movement, RisSource};
use crate::shared::query::Include;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteResponse {
    pub athlete_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_name: Option<String>,
    pub slug: String,
    pub gender: Gender,
    pub country: String,
    pub profile_picture_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instagram_handle: Option<String>,
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competitions: Option<Vec<AthleteCompetitionSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_records: Option<Vec<PersonalRecord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_competitions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standing: Option<AthleteStanding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength_profile: Option<StrengthProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StrengthProfile {
    pub category: String,
    pub lifts: Vec<StrengthComparison>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StrengthComparison {
    pub movement_name: Movement,
    pub value: Option<rust_decimal::Decimal>,
    /// Percentage of other athletes below this result, with ties counting as half.
    pub percentile: Option<f64>,
    /// Other athletes with a valid best for this lift in the same sex and weight class.
    pub field: i64,
}

impl StrengthProfile {
    pub fn from_rows(rows: Vec<AthleteStrengthRow>) -> Option<Self> {
        let first = rows.first()?;
        Some(Self {
            category: osl_domain::category_label(
                None,
                first.category_gender,
                first.weight_class_min,
                first.weight_class_max,
            ),
            lifts: rows
                .into_iter()
                .map(|row| StrengthComparison {
                    movement_name: row.movement_name.into(),
                    value: row.value,
                    percentile: row.percentile,
                    field: row.field,
                })
                .collect(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StandingPlace {
    pub place: i64,
    pub field: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CountryStanding {
    pub code: String,
    pub place: i64,
    pub field: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MetricStanding {
    pub value: rust_decimal::Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    pub global: StandingPlace,
    pub country: CountryStanding,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteStanding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ris: Option<MetricStanding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<MetricStanding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muscleup: Option<MetricStanding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pullup: Option<MetricStanding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dips: Option<MetricStanding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squat: Option<MetricStanding>,
}

impl MetricStanding {
    fn from_row(row: AthleteMetricStandingRow) -> Self {
        let class = if row.metric == RankingMovement::Ris {
            None
        } else {
            Some(
                osl_domain::WeightClass::of(row.weight_class_min, row.weight_class_max)
                    .expect("ranked participants always have a weight class")
                    .to_string(),
            )
        };

        Self {
            value: row.value,
            class,
            global: StandingPlace {
                place: row.global_place,
                field: row.global_field,
            },
            country: CountryStanding {
                code: row.country,
                place: row.country_place,
                field: row.country_field,
            },
        }
    }
}

impl AthleteStanding {
    pub fn from_rows(rows: Vec<AthleteMetricStandingRow>) -> Option<Self> {
        if rows.is_empty() {
            return None;
        }

        let mut standing = Self {
            ris: None,
            total: None,
            muscleup: None,
            pullup: None,
            dips: None,
            squat: None,
        };

        for row in rows {
            let slot = match row.metric {
                RankingMovement::Ris => &mut standing.ris,
                RankingMovement::Total => &mut standing.total,
                RankingMovement::Muscleup => &mut standing.muscleup,
                RankingMovement::Pullup => &mut standing.pullup,
                RankingMovement::Dips => &mut standing.dips,
                RankingMovement::Squat => &mut standing.squat,
            };
            *slot = Some(MetricStanding::from_row(row));
        }

        Some(standing)
    }
}

/// No best weight means a bombed movement; `event` distinguishes it from an uncontested one.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteLift {
    pub movement_name: Movement,
    pub best_weight: Option<rust_decimal::Decimal>,
    pub attempts: Vec<AttemptInfo>,
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
    /// Reported scores cannot be re-scored without bodyweight; computed scores
    /// use the current formula. Absent alongside a missing score.
    pub ris_source: Option<RisSource>,
    pub status: AthleteStatus,
    /// The movements the competition ran, as letters of MPDS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    pub lifts: Vec<AthleteLift>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PersonalRecord {
    pub movement_name: Movement,
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
            native_name: athlete.native_name,
            slug: athlete.slug,
            gender: athlete.gender.into(),
            country: athlete.country,
            profile_picture_url: athlete.profile_picture_url,
            instagram_handle: None,
            created_at: athlete.created_at,
            competitions: None,
            personal_records: None,
            total_competitions: None,
            standing: None,
            strength_profile: None,
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
            ris_source: row.ris_source.map(Into::into),
            status: row.status.into(),
            event: row.event_code,
            lifts: row.lifts.into_iter().map(AthleteLift::from).collect(),
        }
    }
}

impl From<AthleteLiftRow> for AthleteLift {
    fn from(row: AthleteLiftRow) -> Self {
        Self {
            movement_name: row.movement_name.into(),
            best_weight: row.best_weight,
            attempts: row.attempts.into_iter().map(AttemptInfo::from).collect(),
        }
    }
}

impl From<PersonalRecordRow> for PersonalRecord {
    fn from(row: PersonalRecordRow) -> Self {
        Self {
            movement_name: row.movement_name.into(),
            max_weight: row.max_weight,
            competition_name: row.competition_name,
            competition_slug: row.competition_slug,
            date: row.date,
        }
    }
}

impl AthleteResponse {
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
