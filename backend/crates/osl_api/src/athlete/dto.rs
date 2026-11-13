use chrono::NaiveDateTime;
use osl_db::params::RankingMovement;
use osl_db::projections::athlete::{
    AthleteBalanceRow, AthleteCompetitionRow, AthleteDetail, AthleteLiftRow, PersonalRecordRow,
};
use osl_db::projections::ranking::{AthleteMetricStandingRow, AthleteNeighborRow};
use osl_db::rows::athlete::AthleteRow;
use osl_domain::{AthleteStatus, Gender, Movement, RecordMark, RisSource};
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
    #[schema(required = true)]
    pub native_name: Option<String>,
    pub slug: String,
    pub gender: Gender,
    #[schema(required = true)]
    pub country: Option<String>,
    #[schema(required = true)]
    pub profile_picture_url: Option<String>,
    #[schema(required = true)]
    pub instagram_handle: Option<String>,
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub competitions: Option<Vec<AthleteCompetitionSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub personal_records: Option<Vec<PersonalRecord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub total_competitions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standing: Option<Option<AthleteStanding>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lift_balance: Option<Option<LiftBalance>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbors: Option<Option<AthleteNeighborhood>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteNeighbor {
    pub athlete_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub slug: String,
    #[schema(required = true)]
    pub country: Option<String>,
    pub value: rust_decimal::Decimal,
    pub gap: rust_decimal::Decimal,
    pub place: i64,
    pub field: i64,
    pub competition_name: String,
    pub competition_slug: String,
    #[schema(required = true)]
    pub competition_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteNeighborhood {
    pub category: String,
    pub global: Vec<AthleteNeighbor>,
    #[schema(required = true)]
    pub national: Option<Vec<AthleteNeighbor>>,
}

impl AthleteNeighborhood {
    #[must_use]
    pub fn from_rows(rows: Vec<AthleteNeighborRow>, gender: Gender) -> Option<Self> {
        let first = rows.first()?;
        let category = osl_domain::category_label(
            None,
            gender,
            first.weight_class_min,
            first.weight_class_max,
        );
        let mut global = Vec::new();
        let mut national = Vec::new();
        for row in rows {
            let target = if row.scope == "global" {
                &mut global
            } else {
                &mut national
            };
            target.push(AthleteNeighbor {
                athlete_id: row.athlete_id,
                first_name: row.first_name,
                last_name: row.last_name,
                slug: row.slug,
                country: row.country,
                value: row.value,
                gap: row.gap,
                place: row.place,
                field: row.field,
                competition_name: row.competition_name,
                competition_slug: row.competition_slug,
                competition_date: row.competition_date,
            });
        }
        Some(Self {
            category,
            global,
            national: (!national.is_empty()).then_some(national),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LiftBalance {
    pub category: String,
    pub total: rust_decimal::Decimal,
    pub competition_name: String,
    pub competition_slug: String,
    pub competition_date: chrono::NaiveDate,
    /// Other athletes with a four-lift result in the same sex and weight class.
    pub field: i64,
    pub lifts: Vec<LiftShare>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LiftShare {
    pub movement_name: Movement,
    pub value: rust_decimal::Decimal,
    /// Percentage of the athlete's best total taken by this lift.
    pub share: f64,
    /// Mean share across the other athletes in the class.
    #[schema(required = true)]
    pub class_share: Option<f64>,
    /// Sample standard deviation of the share across the other athletes in the class.
    #[schema(required = true)]
    pub class_spread: Option<f64>,
}

impl LiftBalance {
    #[must_use]
    pub fn from_rows(rows: Vec<AthleteBalanceRow>) -> Option<Self> {
        let first = rows.first()?;
        Some(Self {
            category: osl_domain::category_label(
                None,
                first.category_gender,
                first.weight_class_min,
                first.weight_class_max,
            ),
            total: first.total,
            competition_name: first.competition_name.clone(),
            competition_slug: first.competition_slug.clone(),
            competition_date: first.competition_date,
            field: first.field,
            lifts: rows
                .into_iter()
                .map(|row| LiftShare {
                    movement_name: row.movement_name,
                    value: row.value,
                    share: row.share,
                    class_share: row.class_share,
                    class_spread: row.class_spread,
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
    #[schema(required = true)]
    pub class: Option<String>,
    pub global: StandingPlace,
    #[schema(required = true)]
    pub country: Option<CountryStanding>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteStanding {
    #[schema(required = true)]
    pub ris: Option<MetricStanding>,
    #[schema(required = true)]
    pub total: Option<MetricStanding>,
    #[schema(required = true)]
    pub muscleup: Option<MetricStanding>,
    #[schema(required = true)]
    pub pullup: Option<MetricStanding>,
    #[schema(required = true)]
    pub dips: Option<MetricStanding>,
    #[schema(required = true)]
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
            country: row.country.map(|code| CountryStanding {
                code,
                place: row.country_place,
                field: row.country_field,
            }),
        }
    }
}

impl AthleteStanding {
    #[must_use]
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteLift {
    pub movement_name: Movement,
    #[schema(required = true)]
    pub best_weight: Option<rust_decimal::Decimal>,
    /// `WR` or `NR` when this lift is the current world or national record for its weight class.
    #[schema(required = true)]
    pub record: Option<RecordMark>,
    pub attempts: Vec<AttemptInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteCompetitionSummary {
    pub competition_id: Uuid,
    pub competition_name: String,
    pub competition_slug: String,
    #[schema(required = true)]
    pub competition_date: Option<chrono::NaiveDate>,
    pub category_name: String,
    #[schema(required = true)]
    pub division: Option<String>,
    #[schema(required = true)]
    pub rank: Option<i32>,
    #[schema(required = true)]
    pub total: Option<rust_decimal::Decimal>,
    #[schema(required = true)]
    pub ris_score: Option<rust_decimal::Decimal>,
    #[schema(required = true)]
    pub ris_source: Option<RisSource>,
    pub status: AthleteStatus,
    #[schema(required = true)]
    pub event: Option<String>,
    pub lifts: Vec<AthleteLift>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PersonalRecord {
    pub movement_name: Movement,
    pub max_weight: rust_decimal::Decimal,
    /// `WR` or `NR` when this best is the current world or national record for its weight class.
    #[schema(required = true)]
    pub record: Option<RecordMark>,
    pub competition_name: String,
    pub competition_slug: String,
    #[schema(required = true)]
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
            lift_balance: None,
            neighbors: None,
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
            record: row.record,
            attempts: row.attempts.into_iter().map(AttemptInfo::from).collect(),
        }
    }
}

impl From<PersonalRecordRow> for PersonalRecord {
    fn from(row: PersonalRecordRow) -> Self {
        Self {
            movement_name: row.movement_name,
            max_weight: row.max_weight,
            record: row.record,
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
