use chrono::NaiveDate;
use osl_db::params::{RankingFilter, RankingMovement};
use osl_db::projections::ranking::RankingRow;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Movement {
    Muscleup,
    Pullup,
    Dips,
    Squat,
    #[default]
    Total,
}

impl From<Movement> for RankingMovement {
    fn from(movement: Movement) -> Self {
        match movement {
            Movement::Muscleup => Self::Muscleup,
            Movement::Pullup => Self::Pullup,
            Movement::Dips => Self::Dips,
            Movement::Squat => Self::Squat,
            Movement::Total => Self::Total,
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GlobalRankingFilter {
    #[serde(flatten)]
    pub pagination: crate::shared::dto::PaginationParams,
    pub gender: Option<String>,
    pub country: Option<String>,
    #[serde(default)]
    pub movement: Movement,
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
            movement: self.movement.into(),
            offset: self.pagination.offset() as i64,
            limit: self.pagination.limit() as i64,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GlobalRankingEntry {
    pub rank: i64,
    pub athlete: AthleteInfo,
    /// Absent when no score could be established, rather than zero.
    pub ris: Option<f64>,
    pub ris_source: Option<String>,
    pub total: f64,
    pub muscleup: f64,
    pub pullup: f64,
    pub dips: f64,
    pub squat: f64,
    pub competition: CompetitionInfo,
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
    pub date: Option<NaiveDate>,
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
            ris: row.ris_score.map(decimal_to_f64),
            ris_source: row.ris_source,
            total: decimal_to_f64(row.total),
            muscleup: decimal_to_f64(row.muscleup),
            pullup: decimal_to_f64(row.pullup),
            dips: decimal_to_f64(row.dips),
            squat: decimal_to_f64(row.squat),
            competition: CompetitionInfo {
                competition_id: row.competition_id,
                name: row.competition_name,
                date: row.start_date,
            },
        }
    }
}

fn decimal_to_f64(decimal: Decimal) -> f64 {
    decimal.to_string().parse().unwrap_or(0.0)
}
