use chrono::NaiveDateTime;
use osl_db::projections::athlete::{
    AthleteCompetitionRow, AthleteDetail, AthleteLiftRow, PersonalRecordRow,
};
use osl_db::projections::ranking::AthleteMetricStandingRow;
use osl_db::rows::athlete::AthleteRow;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::competition::dto::AttemptInfo;
use crate::shared::query::Include;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteResponse {
    pub athlete_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_name: Option<String>,
    pub slug: String,
    pub gender: String,
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
        let class = if row.metric == "ris" {
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
            let metric = row.metric.clone();
            let metric_standing = MetricStanding::from_row(row);

            match metric.as_str() {
                "ris" => standing.ris = Some(metric_standing),
                "total" => standing.total = Some(metric_standing),
                "muscleup" => standing.muscleup = Some(metric_standing),
                "pullup" => standing.pullup = Some(metric_standing),
                "dips" => standing.dips = Some(metric_standing),
                "squat" => standing.squat = Some(metric_standing),
                _ => unreachable!("metric_candidates only emits known ranking metrics"),
            }
        }

        Some(standing)
    }
}

/// An athlete's best on one movement at one competition. A missing weight is a
/// movement they contested and never made, which is not the same as a movement
/// the competition never ran, and `event` is what tells the two apart.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteLift {
    pub movement_name: String,
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
    /// `computed` when the score was worked out from a bodyweight and a
    /// total, `reported` when the source stated it and it cannot be
    /// restated on the current formula. Absent alongside a missing score.
    /// TODO: introduce a enum constant for this
    pub ris_source: Option<String>,
    pub status: String,
    /// The movements the competition ran, as letters of MPDS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    pub lifts: Vec<AthleteLift>,
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
            native_name: athlete.native_name,
            slug: athlete.slug,
            gender: athlete.gender,
            country: athlete.country,
            profile_picture_url: athlete.profile_picture_url,
            instagram_handle: None,
            created_at: athlete.created_at,
            competitions: None,
            personal_records: None,
            total_competitions: None,
            standing: None,
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
            ris_source: row.ris_source,
            status: row.status,
            event: row.event_code,
            lifts: row.lifts.into_iter().map(AthleteLift::from).collect(),
        }
    }
}

impl From<AthleteLiftRow> for AthleteLift {
    fn from(row: AthleteLiftRow) -> Self {
        Self {
            movement_name: row.movement_name,
            best_weight: row.best_weight,
            attempts: row.attempts.into_iter().map(AttemptInfo::from).collect(),
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
