use chrono::NaiveDate;
use osl_db::params::{RecordClass, RecordsFilter};
use osl_db::projections::records::RecordRow;
use osl_domain::{Gender, Movement, WeightClassSlug};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

const DEFAULT_LIMIT: i64 = 3;
const MAX_LIMIT: i64 = 50;
const OPEN: &str = "open";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum RecordMovement {
    Muscleup,
    Pullup,
    Dips,
    Squat,
}

impl From<RecordMovement> for Movement {
    fn from(movement: RecordMovement) -> Self {
        match movement {
            RecordMovement::Muscleup => Self::MuscleUp,
            RecordMovement::Pullup => Self::PullUp,
            RecordMovement::Dips => Self::Dips,
            RecordMovement::Squat => Self::Squat,
        }
    }
}

impl From<Movement> for RecordMovement {
    fn from(movement: Movement) -> Self {
        match movement {
            Movement::MuscleUp => Self::Muscleup,
            Movement::PullUp => Self::Pullup,
            Movement::Dips => Self::Dips,
            Movement::Squat => Self::Squat,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub enum RecordGender {
    M,
    F,
}

impl From<RecordGender> for Gender {
    fn from(gender: RecordGender) -> Self {
        match gender {
            RecordGender::M => Self::M,
            RecordGender::F => Self::F,
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct RecordsQuery {
    pub movement: Option<RecordMovement>,
    pub gender: Option<RecordGender>,
    /// A weight class such as `M-73` or `F+70`, or `open` for the best across all classes.
    pub weight_class: Option<String>,
    /// An ISO 3166-1 alpha-2 country code, such as `FR`.
    pub country: Option<String>,
    /// Entries per group, from 1 to 50. Defaults to 3.
    pub limit: Option<i64>,
}

impl RecordsQuery {
    pub fn to_db_filter(&self) -> Result<RecordsFilter, String> {
        let limit = self.limit.unwrap_or(DEFAULT_LIMIT);
        if !(1..=MAX_LIMIT).contains(&limit) {
            return Err(format!("limit must be between 1 and {MAX_LIMIT}"));
        }

        let weight_class = match self.weight_class.as_deref().map(str::trim) {
            None | Some("") => None,
            Some(OPEN) => Some(RecordClass::Open),
            Some(slug) => {
                let slug: WeightClassSlug = slug.parse()?;
                Some(RecordClass::Slug(slug.as_str().to_string()))
            }
        };

        Ok(RecordsFilter {
            movement: self.movement.map(Into::into),
            gender: self.gender.map(Into::into),
            weight_class,
            country: self
                .country
                .as_deref()
                .map(str::trim)
                .filter(|country| !country.is_empty())
                .map(str::to_ascii_uppercase),
            limit,
        })
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecordGroup {
    pub movement: RecordMovement,
    pub gender: RecordGender,
    /// The weight class, or `null` for the open record.
    #[schema(required = true)]
    pub weight_class: Option<String>,
    pub entries: Vec<RecordEntry>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecordEntry {
    pub rank: i64,
    pub athlete: RecordAthlete,
    pub weight: Decimal,
    pub date: NaiveDate,
    pub competition: RecordCompetition,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecordAthlete {
    pub first_name: String,
    pub last_name: String,
    pub slug: String,
    #[schema(required = true)]
    pub country: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecordCompetition {
    pub name: String,
    pub slug: String,
}

#[must_use]
pub fn group(rows: Vec<RecordRow>) -> Vec<RecordGroup> {
    let mut groups: Vec<RecordGroup> = Vec::new();
    for row in rows {
        let Some(movement) = Movement::from_name(&row.movement_name) else {
            continue;
        };
        let gender = match row.gender {
            Gender::M => RecordGender::M,
            Gender::F => RecordGender::F,
            Gender::Mx => continue,
        };
        let movement = RecordMovement::from(movement);
        let entry = RecordEntry {
            rank: row.rank,
            athlete: RecordAthlete {
                first_name: row.first_name,
                last_name: row.last_name,
                slug: row.slug,
                country: row.country,
            },
            weight: row.weight,
            date: row.date,
            competition: RecordCompetition {
                name: row.competition_name,
                slug: row.competition_slug,
            },
        };

        match groups.last_mut() {
            Some(last)
                if last.movement == movement
                    && last.gender == gender
                    && last.weight_class == row.weight_class =>
            {
                last.entries.push(entry);
            }
            _ => groups.push(RecordGroup {
                movement,
                gender,
                weight_class: row.weight_class,
                entries: vec![entry],
            }),
        }
    }
    groups
}
