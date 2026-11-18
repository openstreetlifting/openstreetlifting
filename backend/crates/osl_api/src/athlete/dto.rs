use chrono::NaiveDateTime;
use osl_db::params::RankingMovement;
use osl_db::projections::athlete::{
    AthleteBalanceRow, AthleteCompetitionRow, AthleteLiftRow, AthleteRecords, BestRisRow,
    BestTotalRow, PersonalRecordRow, count_competitions,
};
use osl_db::projections::ranking::{AthleteMetricStandingRow, AthleteNeighborRow};
use osl_db::rows::athlete::AthleteRow;
use osl_domain::{AthleteStatus, Gender, Movement, RecordMark, RisSource};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::competition::dto::AttemptInfo;

/// Optional athlete section.
///
/// `competitions` adds `competitions` and `total_competitions`, `records` adds `personal_records`,
/// `best_total` and `best_ris`, `standing` adds `standing`, `lift_balance` adds `lift_balance`, and
/// `neighbors` adds `neighbors`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AthleteInclude {
    Competitions,
    Records,
    Standing,
    LiftBalance,
    Neighbors,
}

/// An athlete. Sections appear only when requested with `include`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteResponse {
    /// Stable identifier of the athlete.
    pub athlete_id: Uuid,
    /// First name. Empty for redacted athletes.
    pub first_name: String,
    /// Last name, or a placeholder such as `Redacted Athlete #1` for redacted athletes.
    pub last_name: String,
    /// Name in the athlete's native script, when known.
    #[schema(required = true)]
    pub native_name: Option<String>,
    /// URL identifier of the athlete.
    pub slug: String,
    /// Sex the athlete competes under.
    pub gender: Gender,
    /// Athlete's ISO 3166-1 alpha-2 country code, such as `FR`.
    #[schema(required = true)]
    pub country: Option<String>,
    /// URL of the profile picture, when available.
    #[schema(required = true)]
    pub profile_picture_url: Option<String>,
    /// Instagram handle without the `@`, when known.
    #[schema(required = true)]
    pub instagram_handle: Option<String>,
    /// When the athlete was added to the archive.
    pub created_at: NaiveDateTime,
    /// Competition history, newest first. Requires `include=competitions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub competitions: Option<Vec<AthleteCompetitionSummary>>,
    /// Best successful lift for each movement. Requires `include=records`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub personal_records: Option<Vec<PersonalRecord>>,
    /// Highest total at an MPDS competition, or `null` when there is none. Requires
    /// `include=records`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_total: Option<Option<BestTotal>>,
    /// Highest RIS at an MPDS competition, or `null` when there is none. Requires
    /// `include=records`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_ris: Option<Option<BestRis>>,
    /// Distinct competitions entered. Several divisions at one meet count once. Requires
    /// `include=competitions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub total_competitions: Option<i64>,
    /// Places in the rankings, or `null` when the athlete is unranked. Requires `include=standing`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standing: Option<Option<AthleteStanding>>,
    /// How the best MPDS total divides across the lifts, or `null` when there is none. Requires
    /// `include=lift_balance`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lift_balance: Option<Option<LiftBalance>>,
    /// Athletes ranked around this one on MPDS total, or `null` when there is no eligible total.
    /// Requires `include=neighbors`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbors: Option<Option<AthleteNeighborhood>>,
}

/// An athlete near the requested athlete on the MPDS total ranking.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteNeighbor {
    /// Stable identifier of the athlete.
    pub athlete_id: Uuid,
    /// First name. Empty for redacted athletes.
    pub first_name: String,
    /// Last name, or a placeholder such as `Redacted Athlete #1` for redacted athletes.
    pub last_name: String,
    /// URL identifier of the athlete.
    pub slug: String,
    /// Athlete's ISO 3166-1 alpha-2 country code, such as `FR`.
    #[schema(required = true)]
    pub country: Option<String>,
    /// Best MPDS total in kilograms.
    pub value: rust_decimal::Decimal,
    /// This total minus the requested athlete's total. Zero for the requested athlete.
    pub gap: rust_decimal::Decimal,
    /// Place in this ranking.
    pub place: i64,
    /// Athletes in this ranking.
    pub field: i64,
    /// Name of the competition.
    pub competition_name: String,
    /// URL identifier of the competition.
    pub competition_slug: String,
    /// Start date of the competition.
    #[schema(required = true)]
    pub competition_date: Option<chrono::NaiveDate>,
}

/// Athletes ranked around the requested athlete in their latest MPDS weight class.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteNeighborhood {
    /// Sex and weight class compared, such as `Men -80kg`.
    pub category: String,
    /// Nearby athletes across all countries, including the requested athlete.
    pub global: Vec<AthleteNeighbor>,
    /// Nearby athletes from the same country, or `null` without a nationality.
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

/// How the athlete's best MPDS total divides across the four lifts.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LiftBalance {
    /// Sex and weight class of that total, such as `Men -80kg`.
    pub category: String,
    /// The best MPDS total in kilograms.
    pub total: rust_decimal::Decimal,
    /// Name of the competition.
    pub competition_name: String,
    /// URL identifier of the competition.
    pub competition_slug: String,
    /// Start date of the competition.
    pub competition_date: chrono::NaiveDate,
    /// Other athletes with an MPDS result in the same sex and weight class.
    pub field: i64,
    /// Share of the total taken by each lift.
    pub lifts: Vec<LiftShare>,
}

/// One lift's part of a total.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LiftShare {
    /// The movement.
    pub movement_name: Movement,
    /// Weight lifted in kilograms.
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

/// A place within a ranking.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StandingPlace {
    /// Place, starting at 1.
    pub place: i64,
    /// Athletes in this ranking.
    pub field: i64,
}

/// A place among athletes from one country.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CountryStanding {
    /// ISO 3166-1 alpha-2 country code, such as `FR`.
    pub code: String,
    /// Place within the country, starting at 1.
    pub place: i64,
    /// Athletes from this country in this ranking.
    pub field: i64,
}

/// The athlete's position for one metric.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MetricStanding {
    /// Best value: kilograms for lifts and total, points for RIS.
    pub value: rust_decimal::Decimal,
    /// Weight class compared, such as `-80kg`, or `null` for RIS, which spans classes.
    #[schema(required = true)]
    pub class: Option<String>,
    /// Place across all countries.
    pub global: StandingPlace,
    /// Place within the athlete's country, or `null` without a nationality.
    #[schema(required = true)]
    pub country: Option<CountryStanding>,
}

/// The athlete's place for each metric. A metric is `null` when unranked.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteStanding {
    /// Standing on RIS, from MPDS competitions.
    #[schema(required = true)]
    pub ris: Option<MetricStanding>,
    /// Standing on total, from MPDS competitions, in the latest weight class.
    #[schema(required = true)]
    pub total: Option<MetricStanding>,
    /// Standing on muscle-up.
    #[schema(required = true)]
    pub muscleup: Option<MetricStanding>,
    /// Standing on pull-up.
    #[schema(required = true)]
    pub pullup: Option<MetricStanding>,
    /// Standing on dips.
    #[schema(required = true)]
    pub dips: Option<MetricStanding>,
    /// Standing on squat.
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

/// A lift in the athlete's competition history.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteLift {
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

/// One entry in the athlete's competition history.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AthleteCompetitionSummary {
    /// Stable identifier of the competition.
    pub competition_id: Uuid,
    /// Name of the competition.
    pub competition_name: String,
    /// URL identifier of the competition.
    pub competition_slug: String,
    /// Start date of the competition.
    pub competition_date: chrono::NaiveDate,
    /// Sex and weight class entered, such as `Men -80kg`.
    pub category_name: String,
    /// Division entered, when the competition has divisions.
    #[schema(required = true)]
    pub division: Option<String>,
    /// Place in the category, or `null` when unplaced.
    #[schema(required = true)]
    pub rank: Option<i32>,
    /// Total in kilograms, or `null` when incomplete or not competed.
    #[schema(required = true)]
    pub total: Option<rust_decimal::Decimal>,
    /// `WR` or `NR` when this total is the current world or national record for its weight class.
    #[schema(required = true)]
    pub total_record: Option<RecordMark>,
    /// RIS points, when available.
    #[schema(required = true)]
    pub ris_score: Option<rust_decimal::Decimal>,
    /// Where the RIS comes from, when there is one.
    #[schema(required = true)]
    pub ris_source: Option<RisSource>,
    /// Whether the athlete competed.
    pub status: AthleteStatus,
    /// Movements contested, such as `MPDS` or `PD`.
    #[schema(required = true)]
    pub event: Option<String>,
    /// Results for each movement.
    pub lifts: Vec<AthleteLift>,
}

/// The athlete's best successful result for one movement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PersonalRecord {
    /// The movement.
    pub movement_name: Movement,
    /// Best weight in kilograms.
    pub max_weight: rust_decimal::Decimal,
    /// `WR` or `NR` when this best is the current world or national record for its weight class.
    #[schema(required = true)]
    pub record: Option<RecordMark>,
    /// Name of the competition.
    pub competition_name: String,
    /// URL identifier of the competition.
    pub competition_slug: String,
    /// Start date of the competition.
    pub date: chrono::NaiveDate,
}

/// The athlete's highest total at an MPDS competition.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BestTotal {
    /// The total in kilograms.
    pub total: rust_decimal::Decimal,
    /// `WR` or `NR` when this total is the current world or national record for its weight class.
    #[schema(required = true)]
    pub record: Option<RecordMark>,
    /// Name of the competition.
    pub competition_name: String,
    /// URL identifier of the competition.
    pub competition_slug: String,
    /// Start date of the competition.
    pub date: chrono::NaiveDate,
}

/// The athlete's highest RIS at an MPDS competition.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BestRis {
    /// RIS points.
    pub ris_score: rust_decimal::Decimal,
    /// Where the score comes from.
    pub ris_source: RisSource,
    /// Name of the competition.
    pub competition_name: String,
    /// URL identifier of the competition.
    pub competition_slug: String,
    /// Start date of the competition.
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
