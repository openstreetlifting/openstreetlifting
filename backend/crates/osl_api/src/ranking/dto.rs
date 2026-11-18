use crate::shared::dto::Direction;
use crate::shared::filters::RankedGender;
use chrono::NaiveDate;
use osl_db::filter::Filters;
use osl_db::params::{RankingBy, RankingFilter, RankingMovement};
use osl_db::projections::ranking::RankingRow;
use osl_domain::{Gender, Movement, RecordMark, RisSource, WeightClass};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

/// Value athletes are ranked on: one lift, the total or RIS.
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum RankingMetric {
    Muscleup,
    Pullup,
    Dips,
    Squat,
    Total,
    #[default]
    Ris,
}

impl From<RankingMetric> for RankingMovement {
    fn from(metric: RankingMetric) -> Self {
        match metric {
            RankingMetric::Muscleup => Self::Muscleup,
            RankingMetric::Pullup => Self::Pullup,
            RankingMetric::Dips => Self::Dips,
            RankingMetric::Squat => Self::Squat,
            RankingMetric::Total => Self::Total,
            RankingMetric::Ris => Self::Ris,
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GlobalRankingFilter {
    #[serde(flatten)]
    #[param(ignore)]
    pub pagination: crate::shared::dto::PaginationParams,
    /// Only this sex. `MX` requires `competition_id`.
    pub gender: Option<RankedGender>,
    /// Only athletes from this ISO 3166-1 alpha-2 country code.
    #[param(example = "FR")]
    pub country: Option<String>,
    /// Only results from this federation, by exact name.
    pub federation: Option<String>,
    /// Only results from this competition series, as listed by the facets endpoint.
    pub series: Option<String>,
    /// Case-insensitive text found in the athlete's full name.
    pub q: Option<String>,
    /// Value to rank on. Defaults to `ris`.
    #[serde(default)]
    pub movement: RankingMetric,
    /// Sort order of the ranked value. Defaults to `desc`, best first.
    #[serde(default)]
    pub direction: Direction,
    /// Event to rank, as movement letters such as `PD`. Defaults to `MPDS`.
    #[param(example = "MPDS")]
    pub event: Option<String>,
    /// Only this weight class, written `-80kg` or `+101kg`.
    #[param(example = "-80kg")]
    pub category: Option<String>,

    /// Only results from competitions in this year.
    pub year: Option<i32>,
    /// Rank the entries of one competition. Without it, each athlete appears once with their best
    /// result.
    pub competition_id: Option<Uuid>,
}

impl GlobalRankingFilter {
    pub fn validate(&self) -> Result<(), String> {
        self.pagination.validate()?;
        if self.gender == Some(RankedGender::Mx) && self.competition_id.is_none() {
            return Err("MX requires competition_id; global rankings use athlete sex".into());
        }

        if let Some(ref category) = self.category {
            WeightClass::from_str(category)?;
        }

        Ok(())
    }

    pub fn filters(&self) -> Filters<RankingBy> {
        let gender = self.gender.map(Gender::from).map(|gender| {
            if self.competition_id.is_some() {
                RankingBy::CategoryGender(gender)
            } else {
                RankingBy::Sex(gender)
            }
        });
        let class = self
            .category
            .as_deref()
            .and_then(|label| WeightClass::from_str(label).ok());

        [
            gender,
            self.country.clone().map(RankingBy::Country),
            self.federation.clone().map(RankingBy::Federation),
            self.series.clone().map(RankingBy::Series),
            class.map(RankingBy::Class),
            self.year.map(RankingBy::Year),
            self.competition_id.map(RankingBy::Competition),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    pub fn to_db_filter(&self) -> RankingFilter {
        RankingFilter {
            by: self.filters(),
            name: self
                .q
                .as_deref()
                .map(str::trim)
                .filter(|query| !query.is_empty())
                .map(str::to_string),
            movement: self.movement.into(),
            direction: self.direction.into(),
            event: self
                .event
                .clone()
                .unwrap_or_else(|| osl_domain::FULL_EVENT.to_string()),
            offset: self.pagination.offset(),
            limit: i64::from(self.pagination.limit()),
        }
    }
}

/// Filter values that still return ranked results.
#[derive(Debug, Serialize, ToSchema)]
pub struct RankingFacets {
    /// Weight class labels.
    pub classes: Vec<String>,
    /// Country codes.
    pub countries: Vec<String>,
    /// Federation names.
    pub federations: Vec<String>,
    /// Competition series.
    pub series: Vec<RankingSeries>,
    /// Years, newest first.
    pub years: Vec<i32>,
}

impl From<osl_db::projections::ranking::RankingFacets> for RankingFacets {
    fn from(facets: osl_db::projections::ranking::RankingFacets) -> Self {
        Self {
            classes: facets.classes,
            countries: facets.countries,
            federations: facets.federations,
            series: facets
                .series
                .into_iter()
                .map(|option| RankingSeries {
                    series: option.series,
                    name: option.name,
                    federation: option.federation,
                })
                .collect(),
            years: facets.years,
        }
    }
}

/// A competition series.
#[derive(Debug, Serialize, ToSchema)]
pub struct RankingSeries {
    /// Identifier to pass as the `series` filter.
    pub series: String,
    /// Display name.
    pub name: String,
    /// Name of the organising federation.
    pub federation: String,
}

/// One ranked result.
#[derive(Debug, Serialize, ToSchema)]
pub struct GlobalRankingEntry {
    /// Identifier of the competition entry behind this result.
    pub participant_id: Uuid,
    /// Position for the selected metric and filters.
    pub rank: i64,
    /// The athlete.
    pub athlete: AthleteInfo,
    /// Category of the entry, such as `Men -80kg`.
    pub category: String,
    /// Division, when the competition has divisions.
    #[schema(required = true)]
    pub division: Option<String>,
    /// RIS points, when available.
    #[schema(required = true)]
    pub ris: Option<f64>,
    /// Where the RIS comes from, when there is one.
    #[schema(required = true)]
    pub ris_source: Option<RisSource>,
    /// Total in kilograms, when complete.
    #[schema(required = true)]
    pub total: Option<f64>,
    /// `WR` or `NR` when this total is the current world or national record for its weight class.
    #[schema(required = true)]
    pub total_record: Option<RecordMark>,
    /// Best muscle-up in kilograms, when contested.
    #[schema(required = true)]
    pub muscleup: Option<f64>,
    /// Best pull-up in kilograms, when contested.
    #[schema(required = true)]
    pub pullup: Option<f64>,
    /// Best dips in kilograms, when contested.
    #[schema(required = true)]
    pub dips: Option<f64>,
    /// Best squat in kilograms, when contested.
    #[schema(required = true)]
    pub squat: Option<f64>,
    /// Movements of the competition, such as `MPDS`.
    #[schema(required = true)]
    pub event: Option<String>,
    /// Competition of this result.
    pub competition: CompetitionInfo,
    /// Federation of this competition.
    pub federation: FederationInfo,
    /// Lifts in this row that are the current world or national record for their weight class.
    pub records: Vec<LiftRecord>,
}

/// A current record held by a lift in a ranking row.
#[derive(Debug, Serialize, ToSchema)]
pub struct LiftRecord {
    /// The movement.
    pub movement: Movement,
    /// `WR` or `NR`.
    pub mark: RecordMark,
}

/// The athlete behind a ranked result.
#[derive(Debug, Serialize, ToSchema)]
#[schema(as = RankingAthlete)]
pub struct AthleteInfo {
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
    /// Sex the athlete competes under.
    pub gender: Gender,
    /// Bodyweight at this competition in kilograms, when known.
    #[schema(required = true)]
    pub bodyweight: Option<f64>,
    /// Instagram handle without the `@`, when known.
    #[schema(required = true)]
    pub instagram_handle: Option<String>,
}

/// The competition behind a ranked result.
#[derive(Debug, Serialize, ToSchema)]
pub struct CompetitionInfo {
    /// Stable identifier of the competition.
    pub competition_id: Uuid,
    /// Name of the competition.
    pub name: String,
    /// URL identifier of the competition.
    pub slug: String,
    /// Start date, when known.
    #[schema(required = true)]
    pub date: Option<NaiveDate>,
}

/// The federation behind a ranked result.
#[derive(Debug, Serialize, ToSchema)]
#[schema(as = RankingFederation)]
pub struct FederationInfo {
    /// Full name.
    pub name: String,
    /// Short name, when known.
    #[schema(required = true)]
    pub abbreviation: Option<String>,
}

impl GlobalRankingEntry {
    #[must_use]
    pub fn new(
        row: RankingRow,
        records: Vec<LiftRecord>,
        total_record: Option<RecordMark>,
    ) -> Self {
        Self {
            participant_id: row.participant_id,
            rank: row.rank,
            athlete: AthleteInfo {
                athlete_id: row.athlete_id,
                first_name: row.first_name,
                last_name: row.last_name,
                slug: row.slug,
                country: row.country,
                gender: row.gender,
                bodyweight: row.bodyweight.map(decimal_to_f64),
                instagram_handle: row.instagram_handle,
            },
            category: WeightClass::label(row.weight_class_min, row.weight_class_max),
            division: row.division,
            ris: row.ris_score.map(decimal_to_f64),
            ris_source: row.ris_source,
            total: row.total.map(decimal_to_f64),
            total_record,
            muscleup: row.muscleup.map(decimal_to_f64),
            pullup: row.pullup.map(decimal_to_f64),
            dips: row.dips.map(decimal_to_f64),
            squat: row.squat.map(decimal_to_f64),
            event: row.event_code,
            competition: CompetitionInfo {
                competition_id: row.competition_id,
                name: row.competition_name,
                slug: row.competition_slug,
                date: row.start_date,
            },
            federation: FederationInfo {
                name: row.federation_name,
                abbreviation: row.federation_abbreviation,
            },
            records,
        }
    }
}

fn decimal_to_f64(decimal: Decimal) -> f64 {
    decimal.to_string().parse().unwrap_or(0.0)
}
