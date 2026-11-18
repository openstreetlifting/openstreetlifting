use chrono::NaiveDate;
use osl_db::filter::Filters;
use osl_db::params::{RecordBy, RecordClass, RecordsFilter};
use osl_db::projections::records::RecordRow;
use osl_domain::{Gender, Movement, RecordLift, WeightClassSlug};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

const DEFAULT_LIMIT: i64 = 3;
const MAX_LIMIT: i64 = 50;
const OPEN: &str = "open";

/// A lift, or `total` for MPDS totals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum RecordMovement {
    Muscleup,
    Pullup,
    Dips,
    Squat,
    Total,
}

impl From<RecordMovement> for RecordLift {
    fn from(movement: RecordMovement) -> Self {
        match movement {
            RecordMovement::Muscleup => Movement::MuscleUp.into(),
            RecordMovement::Pullup => Movement::PullUp.into(),
            RecordMovement::Dips => Movement::Dips.into(),
            RecordMovement::Squat => Movement::Squat.into(),
            RecordMovement::Total => Self::Total,
        }
    }
}

impl From<RecordLift> for RecordMovement {
    fn from(lift: RecordLift) -> Self {
        match lift {
            RecordLift::Movement(Movement::MuscleUp) => Self::Muscleup,
            RecordLift::Movement(Movement::PullUp) => Self::Pullup,
            RecordLift::Movement(Movement::Dips) => Self::Dips,
            RecordLift::Movement(Movement::Squat) => Self::Squat,
            RecordLift::Total => Self::Total,
        }
    }
}

/// `M` for men, `F` for women.
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
    /// Only this lift, or `total`.
    pub movement: Option<RecordMovement>,
    /// Only this sex.
    pub gender: Option<RecordGender>,
    /// A weight class such as `M-73` or `F+70`, or `open` for the best across all classes.
    /// With a federation or series, any class it entered athletes in, such as `M-75`.
    pub weight_class: Option<String>,
    /// An ISO 3166-1 alpha-2 country code, such as `FR`.
    pub country: Option<String>,
    /// A federation name, as listed by `/api/v1/records/facets`.
    pub federation: Option<String>,
    /// A competition series, as listed by `/api/v1/records/facets`.
    pub series: Option<String>,
    /// Only lifts from competitions held in this year.
    pub year: Option<i32>,
    /// Entries per group, from 1 to 50. Defaults to 3.
    pub limit: Option<i64>,
}

impl RecordsQuery {
    pub fn to_db_filter(&self) -> Result<RecordsFilter, String> {
        let limit = self.limit.unwrap_or(DEFAULT_LIMIT);
        if !(1..=MAX_LIMIT).contains(&limit) {
            return Err(format!("limit must be between 1 and {MAX_LIMIT}"));
        }

        let by: Filters<RecordBy> = [
            self.movement
                .map(|movement| RecordBy::Movement(movement.into())),
            self.gender.map(|gender| RecordBy::Gender(gender.into())),
            non_empty(self.country.as_deref())
                .map(|country| RecordBy::Country(country.to_ascii_uppercase())),
            non_empty(self.federation.as_deref()).map(RecordBy::Federation),
            non_empty(self.series.as_deref()).map(RecordBy::Series),
            self.year.map(RecordBy::Year),
        ]
        .into_iter()
        .flatten()
        .collect();

        let weight_class = match self.weight_class.as_deref().map(str::trim) {
            None | Some("") => None,
            Some(OPEN) => Some(RecordClass::Open),
            Some(label) if RecordBy::within_organiser(&by) => {
                Some(RecordClass::Label(class_label(label)?))
            }
            Some(slug) => {
                let slug: WeightClassSlug = slug.parse()?;
                Some(RecordClass::Label(slug.as_str().to_string()))
            }
        };

        Ok(RecordsFilter {
            by,
            weight_class,
            limit,
        })
    }
}

fn class_label(raw: &str) -> Result<String, String> {
    let invalid = || format!("unknown weight class: {raw}");
    let bound = raw
        .strip_prefix(['M', 'F'])
        .and_then(|rest| rest.strip_prefix(['-', '+']))
        .ok_or_else(invalid)?;
    let bound: Decimal = bound.parse().map_err(|_| invalid())?;
    if bound.is_sign_negative() || bound.is_zero() {
        return Err(invalid());
    }

    Ok(format!("{}{}", &raw[..2], bound.normalize()))
}

fn non_empty(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

/// The best results for one movement, sex and weight class.
#[derive(Debug, Serialize, ToSchema)]
pub struct RecordGroup {
    /// The lift, or `total`.
    pub movement: RecordMovement,
    /// Sex of the group.
    pub gender: RecordGender,
    /// The weight class, or `null` for the open record. A standard class unless a federation
    /// or series is selected, which lists the classes it used.
    #[schema(required = true)]
    pub weight_class: Option<String>,
    /// Best results, one per athlete, heaviest first.
    pub entries: Vec<RecordEntry>,
}

/// One result in a records group.
#[derive(Debug, Serialize, ToSchema)]
pub struct RecordEntry {
    /// Position in the group, starting at 1.
    pub rank: i64,
    /// The athlete.
    pub athlete: RecordAthlete,
    /// Weight or total in kilograms.
    pub weight: Decimal,
    /// Start date of the competition.
    pub date: NaiveDate,
    /// The competition.
    pub competition: RecordCompetition,
}

/// The athlete behind a record.
#[derive(Debug, Serialize, ToSchema)]
pub struct RecordAthlete {
    /// First name. Empty for redacted athletes.
    pub first_name: String,
    /// Last name, or a placeholder such as `Redacted Athlete #1` for redacted athletes.
    pub last_name: String,
    /// URL identifier of the athlete.
    pub slug: String,
    /// Athlete's ISO 3166-1 alpha-2 country code, such as `FR`.
    #[schema(required = true)]
    pub country: Option<String>,
}

/// The competition behind a record.
#[derive(Debug, Serialize, ToSchema)]
pub struct RecordCompetition {
    /// Name of the competition.
    pub name: String,
    /// URL identifier of the competition.
    pub slug: String,
}

#[must_use]
pub fn group(rows: Vec<RecordRow>) -> Vec<RecordGroup> {
    let mut groups: Vec<RecordGroup> = Vec::new();
    for row in rows {
        let gender = match row.gender {
            Gender::M => RecordGender::M,
            Gender::F => RecordGender::F,
            Gender::Mx => continue,
        };
        let movement = RecordMovement::from(row.movement_name);
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

/// Filter values that still return records.
#[derive(Debug, Serialize, ToSchema)]
pub struct RecordFacets {
    /// Country codes.
    pub countries: Vec<String>,
    /// Federation names.
    pub federations: Vec<String>,
    /// Competition series.
    pub series: Vec<RecordSeries>,
    /// Years, newest first.
    pub years: Vec<i32>,
    /// True when the selected federation or series entered athletes in weight classes outside
    /// the standard ladder. Records are then grouped by the classes it used.
    pub custom_weight_classes: bool,
}

/// A competition series.
#[derive(Debug, Serialize, ToSchema)]
pub struct RecordSeries {
    /// Identifier to pass as the `series` filter.
    pub series: String,
    /// Display name.
    pub name: String,
    /// Name of the organising federation.
    pub federation: String,
}
