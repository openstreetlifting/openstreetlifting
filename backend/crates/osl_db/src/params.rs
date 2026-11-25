use osl_domain::{CompetitionStatus, Gender, RecordLift, WeightClass};
use uuid::Uuid;

use crate::filter::Filters;

#[derive(Debug, Clone, Copy)]
pub struct Page {
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RankingMovement {
    Muscleup,
    Pullup,
    Dips,
    Squat,
    Total,
    ClassicTotal,
    Name,
    #[default]
    Ris,
}

impl RankingMovement {
    pub const ALL: [Self; 6] = [
        Self::Ris,
        Self::Total,
        Self::Muscleup,
        Self::Pullup,
        Self::Dips,
        Self::Squat,
    ];

    #[must_use]
    pub const fn as_column(&self) -> &'static str {
        match self {
            Self::Muscleup => "muscleup",
            Self::Pullup => "pullup",
            Self::Dips => "dips",
            Self::Squat => "squat",
            Self::Total | Self::ClassicTotal => "total",
            Self::Name => "last_name",
            Self::Ris => "ris_score",
        }
    }

    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Muscleup => "muscleup",
            Self::Pullup => "pullup",
            Self::Dips => "dips",
            Self::Squat => "squat",
            Self::Total => "total",
            Self::ClassicTotal => "classic_total",
            Self::Name => "name",
            Self::Ris => "ris",
        }
    }

    #[must_use]
    pub const fn ranks_total(&self) -> bool {
        matches!(self, Self::Total | Self::ClassicTotal)
    }
}

impl std::str::FromStr for RankingMovement {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        for movement in [Self::ClassicTotal, Self::Name] {
            if s == movement.as_str() {
                return Ok(movement);
            }
        }

        Self::ALL
            .into_iter()
            .find(|movement| movement.as_str() == s)
            .ok_or_else(|| format!("unknown ranking metric: {s}"))
    }
}

osl_domain::text_enum!(RankingMovement);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SortDirection {
    #[default]
    Desc,
    Asc,
}

impl SortDirection {
    #[must_use]
    pub const fn as_sql(&self) -> &'static str {
        match self {
            Self::Desc => "DESC",
            Self::Asc => "ASC",
        }
    }
}

#[derive(Debug, Clone)]
pub enum RankingBy {
    Sex(Gender),
    CategoryGender(Gender),
    Country(String),
    Federation(String),
    Series(String),
    Class(WeightClass),
    Year(i32),
    Competition(Uuid),
}

#[derive(Debug, Clone)]
pub struct RankingFilter {
    pub by: Filters<RankingBy>,
    pub name: Option<String>,
    pub movement: RankingMovement,
    pub direction: SortDirection,
    pub event: Option<String>,
    pub offset: i64,
    pub limit: i64,
}

impl RankingFilter {
    #[must_use]
    pub fn in_competition(&self) -> bool {
        self.by.any(|by| matches!(by, RankingBy::Competition(_)))
    }

    #[must_use]
    pub fn event_scope(&self) -> Option<&str> {
        match self.movement {
            RankingMovement::ClassicTotal => Some(osl_domain::CLASSIC_EVENT),
            RankingMovement::Total | RankingMovement::Ris => self.event.as_deref(),
            RankingMovement::Muscleup
            | RankingMovement::Pullup
            | RankingMovement::Dips
            | RankingMovement::Squat
            | RankingMovement::Name => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CompetitionBy {
    Status(CompetitionStatus),
    Event(String),
    Federation(String),
    Country(String),
    Year(i32),
}

#[derive(Debug, Clone, Default)]
pub struct CompetitionFilter {
    pub by: Filters<CompetitionBy>,
    pub search: Option<String>,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordClass {
    Open,
    Label(String),
}

#[derive(Debug, Clone)]
pub enum RecordBy {
    Movement(RecordLift),
    Gender(Gender),
    Country(String),
    Federation(String),
    Series(String),
    Year(i32),
}

impl RecordBy {
    #[must_use]
    pub fn within_organiser(by: &Filters<Self>) -> bool {
        by.any(|by| matches!(by, Self::Federation(_) | Self::Series(_)))
    }
}

#[derive(Debug, Clone)]
pub struct RecordsFilter {
    pub by: Filters<RecordBy>,
    pub weight_class: Option<RecordClass>,
    pub limit: i64,
}
