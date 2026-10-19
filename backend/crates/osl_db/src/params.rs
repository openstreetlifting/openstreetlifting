use osl_domain::{CompetitionStatus, Gender, WeightClass};
use uuid::Uuid;

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
            Self::Total => "total",
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
            Self::Ris => "ris",
        }
    }
}

impl std::str::FromStr for RankingMovement {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
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
pub struct RankingFilter {
    pub gender: Option<Gender>,
    pub country: Option<String>,
    pub federation: Option<String>,
    pub name: Option<String>,
    pub movement: RankingMovement,
    pub direction: SortDirection,
    pub event: String,
    pub category: Option<WeightClass>,
    pub year: Option<i32>,
    pub competition_id: Option<Uuid>,
    pub offset: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Default)]
pub struct CompetitionFilter {
    pub status: Option<CompetitionStatus>,
    pub event: Option<String>,
    pub federation: Option<String>,
    pub country: Option<String>,
    pub year: Option<i32>,
    pub search: Option<String>,
    pub direction: SortDirection,
}
