//! Input types owned by osl_db.
//!
//! Repositories take these rather than API request DTOs, so osl_db stays
//! independent of the HTTP layer. osl_api converts its validated request
//! bodies into these on the way in.

use osl_domain::{CompetitionStatus, Gender, ParseError, WeightClass};
use uuid::Uuid;

/// A slice of a collection, already resolved to SQL `LIMIT` / `OFFSET`.
///
/// The page-number arithmetic stays in osl_api; repositories only ever see
/// the resolved bounds.
#[derive(Debug, Clone, Copy)]
pub struct Page {
    pub limit: i64,
    pub offset: i64,
}

/// Movement the global ranking is sorted by.
///
/// Lives here rather than in osl_api because the variants map directly
/// onto CTE column names in the ranking query.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
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
    /// Every metric a standing is worked out for. The athlete standings query
    /// builds one `UNION ALL` branch per entry, so adding a variant here is what
    /// puts it in the ranking rather than a new block of copied SQL.
    pub const ALL: [RankingMovement; 6] = [
        RankingMovement::Ris,
        RankingMovement::Total,
        RankingMovement::Muscleup,
        RankingMovement::Pullup,
        RankingMovement::Dips,
        RankingMovement::Squat,
    ];

    /// The column the value is read from in the `movement_weights` CTE.
    pub fn as_column(&self) -> &'static str {
        match self {
            Self::Muscleup => "muscleup",
            Self::Pullup => "pullup",
            Self::Dips => "dips",
            Self::Squat => "squat",
            Self::Total => "total",
            Self::Ris => "ris_score",
        }
    }

    /// What the metric is called once it has been unioned into one column, and
    /// so what comes back on `AthleteMetricStandingRow::metric`. Only RIS
    /// differs from its column, which is why the two are not the same method.
    pub fn as_str(&self) -> &'static str {
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

impl std::fmt::Display for RankingMovement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for RankingMovement {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|movement| movement.as_str() == s)
            .ok_or_else(|| ParseError::new(format!("unknown ranking metric: {s}")))
    }
}

impl sqlx::Type<sqlx::Postgres> for RankingMovement {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <str as sqlx::Type<sqlx::Postgres>>::type_info()
    }

    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        <str as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for RankingMovement {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        let raw = <&str as sqlx::Decode<'_, sqlx::Postgres>>::decode(value)?;

        Ok(raw.parse()?)
    }
}

/// Which way the ranking runs. Best-first is the natural reading of a
/// leaderboard, so it is the default; worst-first is there for anyone who
/// wants to see who has the most room to grow.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SortDirection {
    #[default]
    Desc,
    Asc,
}

impl SortDirection {
    pub fn as_sql(&self) -> &'static str {
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
    /// Which event a total is ranked within. Ignored when ranking by a single
    /// movement, since those compare across events.
    pub event: String,
    pub category: Option<WeightClass>,
    pub year: Option<i32>,
    /// Narrows the ranking to one competition, e.g. for a per-competition leaderboard.
    pub competition_id: Option<Uuid>,
    pub offset: i64,
    pub limit: i64,
}

/// Narrows the competition list. A `None` leaves that dimension alone, and
/// `search` matches the competition name, its federation and its city, since
/// those are the three things someone types into one box.
#[derive(Debug, Clone, Default)]
pub struct CompetitionFilter {
    pub status: Option<CompetitionStatus>,
    pub federation: Option<String>,
    pub country: Option<String>,
    pub year: Option<i32>,
    pub search: Option<String>,
    /// Results read newest first, an upcoming calendar reads soonest first.
    pub direction: SortDirection,
}
