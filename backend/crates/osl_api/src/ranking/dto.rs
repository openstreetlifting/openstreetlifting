use chrono::NaiveDate;
use osl_db::params::{RankingFilter, RankingMovement, SortDirection};
use osl_db::projections::ranking::RankingRow;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Deserialize, IntoParams)]
pub struct ClassesFilter {
    pub gender: Option<String>,
    pub competition_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct CountriesFilter {
    pub competition_id: Option<Uuid>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Movement {
    Muscleup,
    Pullup,
    Dips,
    Squat,
    Total,
    #[default]
    Ris,
}

impl From<Movement> for RankingMovement {
    fn from(movement: Movement) -> Self {
        match movement {
            Movement::Muscleup => Self::Muscleup,
            Movement::Pullup => Self::Pullup,
            Movement::Dips => Self::Dips,
            Movement::Squat => Self::Squat,
            Movement::Total => Self::Total,
            Movement::Ris => Self::Ris,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    #[default]
    Desc,
    Asc,
}

impl From<Direction> for SortDirection {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Desc => Self::Desc,
            Direction::Asc => Self::Asc,
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GlobalRankingFilter {
    #[serde(flatten)]
    pub pagination: crate::shared::dto::PaginationParams,
    pub gender: Option<String>,
    pub country: Option<String>,
    /// Case insensitive substring of the athlete's full name.
    pub q: Option<String>,
    #[serde(default)]
    pub movement: Movement,
    #[serde(default)]
    pub direction: Direction,
    /// Which event to rank totals within, defaulting to the four movements.
    /// Ignored when ranking by a single movement.
    pub event: Option<String>,
    /// Weight class suffix, e.g. `-73kg`, matched regardless of gender.
    pub category: Option<String>,
    pub year: Option<i32>,
    /// Narrows the ranking to one competition, e.g. for a per-meet leaderboard.
    pub competition_id: Option<Uuid>,
}

impl GlobalRankingFilter {
    pub fn validate(&self) -> Result<(), String> {
        self.pagination.validate()?;

        if let Some(ref gender) = self.gender
            && gender != "M"
            && gender != "F"
        {
            return Err("gender must be 'M' or 'F'".to_string());
        }

        Ok(())
    }

    pub fn to_db_filter(&self) -> RankingFilter {
        RankingFilter {
            gender: self.gender.clone(),
            country: self.country.clone(),
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
            category: self.category.clone(),
            year: self.year,
            competition_id: self.competition_id,
            offset: self.pagination.offset() as i64,
            limit: self.pagination.limit() as i64,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GlobalRankingEntry {
    pub rank: i64,
    pub athlete: AthleteInfo,
    pub category: String,
    /// Absent when no score could be established, rather than zero.
    pub ris: Option<f64>,
    pub ris_source: Option<String>,
    /// Absent outside the four-movement event, where a total is not
    /// comparable and no RIS is computed.
    pub total: Option<f64>,
    /// Absent when the competition did not contest the movement.
    pub muscleup: Option<f64>,
    pub pullup: Option<f64>,
    pub dips: Option<f64>,
    pub squat: Option<f64>,
    /// Which movements the competition contested, e.g. `MPDS` for all four.
    pub event: Option<String>,
    pub competition: CompetitionInfo,
    pub federation: FederationInfo,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AthleteInfo {
    pub athlete_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub slug: String,
    pub country: String,
    pub gender: String,
    pub bodyweight: Option<f64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CompetitionInfo {
    pub competition_id: Uuid,
    pub name: String,
    pub slug: String,
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FederationInfo {
    pub name: String,
    pub abbreviation: Option<String>,
}

impl From<RankingRow> for GlobalRankingEntry {
    fn from(row: RankingRow) -> Self {
        Self {
            rank: row.rank,
            athlete: AthleteInfo {
                athlete_id: row.athlete_id,
                first_name: row.first_name,
                last_name: row.last_name,
                slug: row.slug,
                country: row.country,
                gender: row.gender,
                bodyweight: row.bodyweight.map(decimal_to_f64),
            },
            category: weight_class(&row.category_name),
            ris: row.ris_score.map(decimal_to_f64),
            ris_source: row.ris_source,
            total: row.total.map(decimal_to_f64),
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
        }
    }
}

fn decimal_to_f64(decimal: Decimal) -> f64 {
    decimal.to_string().parse().unwrap_or(0.0)
}

/// Category names are stored as `"<Gender> <weight class>"` (e.g. `"Men -73kg"`).
/// Gender already has its own field on the entry, so drop that leading word here
/// rather than making every caller strip it again.
fn weight_class(category_name: &str) -> String {
    category_name
        .split_once(' ')
        .map(|(_, class)| class)
        .unwrap_or(category_name)
        .to_string()
}
