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

/// Metric to rank on.
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum RankingMetric {
    Muscleup,
    Pullup,
    Dips,
    Squat,
    Total,
    #[serde(rename = "classic_total")]
    ClassicTotal,
    Name,
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
            RankingMetric::ClassicTotal => Self::ClassicTotal,
            RankingMetric::Name => Self::Name,
            RankingMetric::Ris => Self::Ris,
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GlobalRankingFilter {
    #[serde(flatten)]
    #[param(ignore)]
    pub pagination: crate::shared::dto::PaginationParams,
    /// Gender. `MX` requires `competition_id`.
    pub gender: Option<RankedGender>,
    /// Athlete country (ISO 3166-1 alpha-2).
    #[param(example = "FR")]
    pub country: Option<String>,
    /// Federation name.
    pub federation: Option<String>,
    /// Series identifier, from the facets endpoint.
    pub series: Option<String>,
    /// Search on athlete name.
    ///
    /// Narrows the selected board and keeps each athlete's rank on it. To find
    /// an athlete who only competed a shorter event, rank by `name`.
    pub q: Option<String>,
    /// Metric to rank on. Defaults to `ris`.
    #[serde(default)]
    pub movement: RankingMetric,
    /// Sort order. Defaults to `desc`.
    #[serde(default)]
    pub direction: Direction,
    /// Event code, such as `MPDS` or `PD`. Defaults to `MPDS`.
    #[param(example = "MPDS")]
    pub event: Option<String>,
    /// Weight class, such as `-80kg` or `+101kg`.
    #[param(example = "-80kg")]
    pub category: Option<String>,

    /// Competition year.
    pub year: Option<i32>,
    /// Rank one competition. Otherwise each athlete appears once.
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

        if let Some(event) = self.requested_event() {
            osl_domain::event::movements(&event)?;
        }

        Ok(())
    }

    fn requested_event(&self) -> Option<String> {
        self.event
            .as_deref()
            .map(str::trim)
            .filter(|event| !event.is_empty())
            .map(str::to_ascii_uppercase)
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
        let name = self
            .q
            .as_deref()
            .map(str::trim)
            .filter(|query| !query.is_empty())
            .map(str::to_string);
        let movement: RankingMovement = self.movement.into();

        let event = match movement {
            RankingMovement::ClassicTotal | RankingMovement::Name => None,
            _ => Some(
                self.requested_event()
                    .unwrap_or_else(|| osl_domain::FULL_EVENT.to_string()),
            ),
        };

        RankingFilter {
            by: self.filters(),
            name,
            movement,
            direction: self.direction.into(),
            event,
            offset: self.pagination.offset(),
            limit: i64::from(self.pagination.limit()),
        }
    }
}

/// Filter values that return ranked results.
#[derive(Debug, Serialize, ToSchema)]
pub struct RankingFacets {
    /// Weight classes.
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
    /// Series identifier.
    pub series: String,
    /// Series name.
    pub name: String,
    /// Federation name.
    pub federation: String,
}

/// A ranked result.
#[derive(Debug, Serialize, ToSchema)]
pub struct GlobalRankingEntry {
    /// Unique identifier for the entry.
    pub participant_id: Uuid,
    /// Position for the selected filters.
    pub rank: i64,
    /// Athlete.
    pub athlete: AthleteInfo,
    /// Category, such as `Men -80kg`.
    pub category: String,
    /// Division.
    #[schema(required = true)]
    pub division: Option<String>,
    /// RIS points.
    #[schema(required = true)]
    pub ris: Option<f64>,
    /// Origin of the RIS.
    #[schema(required = true)]
    pub ris_source: Option<RisSource>,
    /// Total in kilograms.
    #[schema(required = true)]
    pub total: Option<f64>,
    /// `WR` or `NR` when this total is the current world or national record for its weight class.
    #[schema(required = true)]
    pub total_record: Option<RecordMark>,
    /// Muscle-up in kilograms.
    #[schema(required = true)]
    pub muscleup: Option<f64>,
    /// Pull-up in kilograms.
    #[schema(required = true)]
    pub pullup: Option<f64>,
    /// Dips in kilograms.
    #[schema(required = true)]
    pub dips: Option<f64>,
    /// Squat in kilograms.
    #[schema(required = true)]
    pub squat: Option<f64>,
    /// Event code, such as `MPDS`.
    #[schema(required = true)]
    pub event: Option<String>,
    /// Competition.
    pub competition: CompetitionInfo,
    /// Federation.
    pub federation: FederationInfo,
    /// Lifts in this row that are the current world or national record for their weight class.
    pub records: Vec<LiftRecord>,
}

/// A current record held by a lift.
#[derive(Debug, Serialize, ToSchema)]
pub struct LiftRecord {
    /// Movement.
    pub movement: Movement,
    /// `WR` or `NR`.
    pub mark: RecordMark,
}

/// An athlete.
#[derive(Debug, Serialize, ToSchema)]
#[schema(as = RankingAthlete)]
pub struct AthleteInfo {
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
    /// Gender the athlete competes in.
    pub gender: Gender,
    /// Bodyweight in kilograms.
    #[schema(required = true)]
    pub bodyweight: Option<f64>,
    /// Instagram handle, without `@`.
    #[schema(required = true)]
    pub instagram_handle: Option<String>,
}

/// A competition.
#[derive(Debug, Serialize, ToSchema)]
pub struct CompetitionInfo {
    /// Unique identifier for the competition.
    pub competition_id: Uuid,
    /// Competition name.
    pub name: String,
    /// URL slug.
    pub slug: String,
    /// Start date.
    #[schema(required = true)]
    pub date: Option<NaiveDate>,
}

/// A federation.
#[derive(Debug, Serialize, ToSchema)]
#[schema(as = RankingFederation)]
pub struct FederationInfo {
    /// Full name.
    pub name: String,
    /// Short name.
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

#[cfg(test)]
mod tests {
    use crate::ranking::dto::{GlobalRankingFilter, RankingMetric};
    use crate::shared::dto::{Direction, PaginationParams};
    use osl_db::params::{RankingFilter, RankingMovement};

    fn request(q: Option<&str>, movement: RankingMetric) -> RankingFilter {
        GlobalRankingFilter {
            pagination: PaginationParams {
                page: 1,
                page_size: 50,
            },
            gender: None,
            country: None,
            federation: None,
            series: None,
            q: q.map(str::to_string),
            movement,
            direction: Direction::default(),
            event: None,
            category: None,
            year: None,
            competition_id: None,
        }
        .to_db_filter()
    }

    #[test]
    fn keeps_the_selected_board_when_searching() {
        for (metric, movement) in [
            (RankingMetric::Ris, RankingMovement::Ris),
            (RankingMetric::Total, RankingMovement::Total),
        ] {
            let filter = request(Some("kowalski"), metric);
            assert_eq!(
                filter.event.as_deref(),
                Some(osl_domain::FULL_EVENT),
                "a search narrows the board it is shown on, so ranks stay comparable"
            );
            assert_eq!(filter.movement, movement);
            assert_eq!(filter.name.as_deref(), Some("kowalski"));
        }
    }

    #[test]
    fn keeps_the_requested_board_when_not_searching() {
        let filter = request(None, RankingMetric::Total);
        assert_eq!(filter.event.as_deref(), Some(osl_domain::FULL_EVENT));
        assert_eq!(filter.movement, RankingMovement::Total);
    }

    #[test]
    fn leaves_the_classic_board_unscoped_because_its_metric_names_the_event() {
        let filter = request(None, RankingMetric::ClassicTotal);
        assert_eq!(filter.event, None);
        assert_eq!(
            filter.event_scope(),
            Some(osl_domain::CLASSIC_EVENT),
            "Classic Total always means the PD event"
        );
    }

    #[test]
    fn the_name_directory_spans_every_event_so_shorter_events_are_reachable() {
        let filter = request(Some("kowalski"), RankingMetric::Name);
        assert_eq!(filter.movement, RankingMovement::Name);
        assert_eq!(
            filter.event, None,
            "an athlete who only competed a shorter event must still be listed"
        );
        assert_eq!(filter.event_scope(), None);
        assert_eq!(filter.name.as_deref(), Some("kowalski"));
    }
}
