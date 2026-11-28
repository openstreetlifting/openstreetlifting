use chrono::NaiveDateTime;
use osl_db::params::RankingMovement;
use osl_db::projections::athlete::{
    AthleteBalanceRow, AthleteCompetitionRow, AthleteLiftRow, AthleteRecords, BestRisRow,
    BestTotalRow, PersonalRecordRow, count_competitions,
};
use osl_db::projections::ranking::{AthleteMetricStandingRow, AthleteNeighborRow};
use osl_db::rows::athlete::{AthleteRow, AthleteSearchRow};
use osl_domain::{AthleteStatus, Gender, Movement, RecordMark, RisSource};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::competition::dto::AttemptInfo;
use crate::ranking::dto::{CompetitionInfo, FederationInfo, RankingMetric, decimal_to_f64};

/// Optional athlete section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AthleteInclude {
    Competitions,
    Records,
    Standing,
    LiftBalance,
    Neighbors,
}

/// An athlete.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteResponse {
    /// Unique identifier for the athlete.
    pub athlete_id: Uuid,
    /// First name. Empty when redacted.
    pub first_name: String,
    /// Last name, or a placeholder when redacted.
    pub last_name: String,
    /// Name in the athlete's native script.
    #[schema(required = true)]
    pub native_name: Option<String>,
    /// URL slug.
    pub slug: String,
    /// Gender the athlete competes in.
    pub gender: Gender,
    /// Two-letter country code (ISO 3166-1 alpha-2).
    #[schema(required = true)]
    pub country: Option<String>,
    /// Profile picture URL.
    #[schema(required = true)]
    pub profile_picture_url: Option<String>,
    /// Instagram handle, without `@`.
    #[schema(required = true)]
    pub instagram_handle: Option<String>,
    /// Time the athlete was added.
    pub created_at: NaiveDateTime,
    /// Competition history, newest first. Included with `include=competitions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub competitions: Option<Vec<AthleteCompetitionSummary>>,
    /// Best lift per movement. Included with `include=records`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub personal_records: Option<Vec<PersonalRecord>>,
    /// Best MPDS total. Included with `include=records`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_total: Option<Option<BestTotal>>,
    /// Best MPDS RIS. Included with `include=records`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_ris: Option<Option<BestRis>>,
    /// Competitions entered, each counted once. Included with `include=competitions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub total_competitions: Option<i64>,
    /// Ranking places. Included with `include=standing`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standing: Option<Option<AthleteStanding>>,
    /// Share of each lift in the best MPDS total. Included with `include=lift_balance`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lift_balance: Option<Option<LiftBalance>>,
    /// Athletes ranked nearby on MPDS total. Included with `include=neighbors`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbors: Option<Option<AthleteNeighborhood>>,
}

/// An athlete ranked near the requested athlete.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteNeighbor {
    /// Unique identifier for the athlete.
    pub athlete_id: Uuid,
    /// First name. Empty when redacted.
    pub first_name: String,
    /// Last name, or a placeholder when redacted.
    pub last_name: String,
    /// URL slug.
    pub slug: String,
    /// Two-letter country code (ISO 3166-1 alpha-2).
    #[schema(required = true)]
    pub country: Option<String>,
    /// Best MPDS total in kilograms.
    pub value: rust_decimal::Decimal,
    /// Difference from the requested athlete's total, in kilograms.
    pub gap: rust_decimal::Decimal,
    /// Place, from 1.
    pub place: i64,
    /// Athletes ranked.
    pub field: i64,
    /// Competition name.
    pub competition_name: String,
    /// Competition URL slug.
    pub competition_slug: String,
    /// Competition start date.
    #[schema(required = true)]
    pub competition_date: Option<chrono::NaiveDate>,
}

/// Athletes ranked around the requested athlete, in their latest weight class.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteNeighborhood {
    /// Category compared, such as `Men -80kg`.
    pub category: String,
    /// Nearby athletes worldwide.
    pub global: Vec<AthleteNeighbor>,
    /// Nearby athletes from the same country.
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

/// Share of each lift in the best MPDS total.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LiftBalance {
    /// Category of the total, such as `Men -80kg`.
    pub category: String,
    /// Total in kilograms.
    pub total: rust_decimal::Decimal,
    /// Competition name.
    pub competition_name: String,
    /// Competition URL slug.
    pub competition_slug: String,
    /// Competition start date.
    pub competition_date: chrono::NaiveDate,
    /// Other athletes with an MPDS result in the same sex and weight class.
    pub field: i64,
    /// One entry per lift.
    pub lifts: Vec<LiftShare>,
}

/// One lift's share of a total.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LiftShare {
    /// Movement.
    pub movement_name: Movement,
    /// Weight in kilograms.
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

/// A ranking place.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StandingPlace {
    /// Place, from 1.
    pub place: i64,
    /// Athletes ranked.
    pub field: i64,
}

/// A ranking place within one country.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CountryStanding {
    /// Two-letter country code (ISO 3166-1 alpha-2).
    pub code: String,
    /// Place, from 1.
    pub place: i64,
    /// Athletes ranked from this country.
    pub field: i64,
}

/// Ranking place for one metric.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MetricStanding {
    /// Best value: kilograms, or points for RIS.
    pub value: rust_decimal::Decimal,
    /// Weight class, such as `-80kg`. `null` for RIS.
    #[schema(required = true)]
    pub class: Option<String>,
    /// Place worldwide.
    pub global: StandingPlace,
    /// Place within the athlete's country.
    #[schema(required = true)]
    pub country: Option<CountryStanding>,
}

/// Ranking places by metric. `null` when unranked.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteStanding {
    /// RIS, from MPDS competitions.
    #[schema(required = true)]
    pub ris: Option<MetricStanding>,
    /// Total, from MPDS competitions.
    #[schema(required = true)]
    pub total: Option<MetricStanding>,
    /// Muscle-up.
    #[schema(required = true)]
    pub muscleup: Option<MetricStanding>,
    /// Pull-up.
    #[schema(required = true)]
    pub pullup: Option<MetricStanding>,
    /// Dips.
    #[schema(required = true)]
    pub dips: Option<MetricStanding>,
    /// Squat.
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

/// A lift.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteLift {
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

/// A competition the athlete entered.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteCompetitionSummary {
    /// Unique identifier for the competition.
    pub competition_id: Uuid,
    /// Competition name.
    pub competition_name: String,
    /// Competition URL slug.
    pub competition_slug: String,
    /// Competition start date.
    pub competition_date: chrono::NaiveDate,
    /// Category, such as `Men -80kg`.
    pub category_name: String,
    /// Division.
    #[schema(required = true)]
    pub division: Option<String>,
    /// Place in the category.
    #[schema(required = true)]
    pub rank: Option<i32>,
    /// Total in kilograms.
    #[schema(required = true)]
    pub total: Option<rust_decimal::Decimal>,
    /// `WR` or `NR` when this total is the current world or national record for its weight class.
    #[schema(required = true)]
    pub total_record: Option<RecordMark>,
    /// RIS points.
    #[schema(required = true)]
    pub ris_score: Option<rust_decimal::Decimal>,
    /// Origin of the RIS.
    #[schema(required = true)]
    pub ris_source: Option<RisSource>,
    /// Participation status.
    pub status: AthleteStatus,
    /// Movements contested, such as `MPDS`.
    #[schema(required = true)]
    pub event: Option<String>,
    /// One entry per movement.
    pub lifts: Vec<AthleteLift>,
}

/// Best successful lift for one movement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PersonalRecord {
    /// Movement.
    pub movement_name: Movement,
    /// Weight in kilograms.
    pub max_weight: rust_decimal::Decimal,
    /// `WR` or `NR` when this best is the current world or national record for its weight class.
    #[schema(required = true)]
    pub record: Option<RecordMark>,
    /// Competition name.
    pub competition_name: String,
    /// Competition URL slug.
    pub competition_slug: String,
    /// Competition start date.
    pub date: chrono::NaiveDate,
}

/// The athlete's highest total at an MPDS competition.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BestTotal {
    /// Total in kilograms.
    pub total: rust_decimal::Decimal,
    /// `WR` or `NR` when this total is the current world or national record for its weight class.
    #[schema(required = true)]
    pub record: Option<RecordMark>,
    /// Competition name.
    pub competition_name: String,
    /// Competition URL slug.
    pub competition_slug: String,
    /// Competition start date.
    pub date: chrono::NaiveDate,
}

/// The athlete's highest RIS at an MPDS competition.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BestRis {
    /// RIS points.
    pub ris_score: rust_decimal::Decimal,
    /// Origin of the score.
    pub ris_source: RisSource,
    /// Competition name.
    pub competition_name: String,
    /// Competition URL slug.
    pub competition_slug: String,
    /// Competition start date.
    pub date: chrono::NaiveDate,
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
            best_total: None,
            best_ris: None,
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
            total_record: row.total_record,
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

impl From<BestTotalRow> for BestTotal {
    fn from(row: BestTotalRow) -> Self {
        Self {
            total: row.total,
            record: row.record,
            competition_name: row.competition_name,
            competition_slug: row.competition_slug,
            date: row.date,
        }
    }
}

impl From<BestRisRow> for BestRis {
    fn from(row: BestRisRow) -> Self {
        Self {
            ris_score: row.ris_score,
            ris_source: row.ris_source,
            competition_name: row.competition_name,
            competition_slug: row.competition_slug,
            date: row.date,
        }
    }
}

#[derive(Debug)]
pub struct AthleteHistorySection {
    competitions: Vec<AthleteCompetitionSummary>,
    total_competitions: i64,
}

impl From<Vec<AthleteCompetitionRow>> for AthleteHistorySection {
    fn from(history: Vec<AthleteCompetitionRow>) -> Self {
        Self {
            total_competitions: count_competitions(&history),
            competitions: history.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug)]
pub struct AthleteRecordsSection {
    personal_records: Vec<PersonalRecord>,
    best_total: Option<BestTotal>,
    best_ris: Option<BestRis>,
}

impl From<AthleteRecords> for AthleteRecordsSection {
    fn from(records: AthleteRecords) -> Self {
        Self {
            personal_records: records
                .personal_records
                .into_iter()
                .map(Into::into)
                .collect(),
            best_total: records.best_total.map(Into::into),
            best_ris: records.best_ris.map(Into::into),
        }
    }
}

impl AthleteResponse {
    pub fn set_history(&mut self, section: AthleteHistorySection) {
        self.competitions = Some(section.competitions);
        self.total_competitions = Some(section.total_competitions);
    }

    pub fn set_records(&mut self, section: AthleteRecordsSection) {
        self.personal_records = Some(section.personal_records);
        self.best_total = Some(section.best_total);
        self.best_ris = Some(section.best_ris);
    }
}

/// An athlete whose name matches a search.
#[derive(Debug, Serialize, ToSchema)]
pub struct AthleteSearchHit {
    /// URL slug.
    pub slug: String,
    /// First name. Empty when redacted.
    pub first_name: String,
    /// Last name, or a placeholder when redacted.
    pub last_name: String,
    /// Gender the athlete competes in.
    pub gender: Gender,
    /// Two-letter country code (ISO 3166-1 alpha-2).
    #[schema(required = true)]
    pub country: Option<String>,
    /// Rankings the athlete appears in. Empty when the athlete has no valid lift.
    pub rankings: Vec<RankingMetric>,
    /// The athlete's best result: the entry whose best lifts add up to the most, then the latest.
    #[schema(required = true)]
    pub best_entry: Option<AthleteSearchEntry>,
}

/// One competition entry of a searched athlete.
#[derive(Debug, Serialize, ToSchema)]
pub struct AthleteSearchEntry {
    /// Competition.
    pub competition: CompetitionInfo,
    /// Federation.
    pub federation: FederationInfo,
    /// Event code, such as `PD`.
    #[schema(required = true)]
    pub event: Option<String>,
    /// Participation status.
    pub status: AthleteStatus,
    /// Published reason for the status.
    #[schema(required = true)]
    pub status_reason: Option<String>,
    /// Gender of the category.
    pub gender: Gender,
    /// Weight class, such as `-80kg`. Empty without a weight class.
    pub category: String,
    /// Muscle-up in kilograms. `null` without a successful lift.
    #[schema(required = true)]
    pub muscleup: Option<f64>,
    /// Pull-up in kilograms. `null` without a successful lift.
    #[schema(required = true)]
    pub pullup: Option<f64>,
    /// Dips in kilograms. `null` without a successful lift.
    #[schema(required = true)]
    pub dips: Option<f64>,
    /// Squat in kilograms. `null` without a successful lift.
    #[schema(required = true)]
    pub squat: Option<f64>,
}

/// Athletes whose name matches a search, ranked athletes first.
#[derive(Debug, Serialize, ToSchema)]
pub struct AthleteSearchResponse {
    /// Matching athletes, up to `limit`.
    pub data: Vec<AthleteSearchHit>,
    /// Number of matching athletes.
    pub total_items: i64,
}

impl From<AthleteSearchRow> for AthleteSearchHit {
    fn from(row: AthleteSearchRow) -> Self {
        let best_entry = AthleteSearchEntry::from_row(&row);
        Self {
            best_entry,
            slug: row.slug,
            first_name: row.first_name,
            last_name: row.last_name,
            gender: row.gender,
            country: row.country,
            rankings: row
                .rankings
                .iter()
                .filter_map(|metric| metric.parse::<RankingMovement>().ok())
                .map(RankingMetric::from)
                .collect(),
        }
    }
}

impl AthleteSearchEntry {
    fn from_row(row: &AthleteSearchRow) -> Option<Self> {
        Some(Self {
            competition: CompetitionInfo {
                competition_id: row.competition_id?,
                name: row.competition_name.clone()?,
                slug: row.competition_slug.clone()?,
                date: row.competition_date,
            },
            federation: FederationInfo {
                name: row.federation_name.clone()?,
                abbreviation: row.federation_abbreviation.clone(),
            },
            event: row.event_code.clone(),
            status: row.entry_status?,
            status_reason: row.entry_status_reason.clone(),
            gender: row.entry_gender?,
            category: osl_domain::WeightClass::label(row.weight_class_min, row.weight_class_max),
            muscleup: row.muscleup.map(decimal_to_f64),
            pullup: row.pullup.map(decimal_to_f64),
            dips: row.dips.map(decimal_to_f64),
            squat: row.squat.map(decimal_to_f64),
        })
    }
}
