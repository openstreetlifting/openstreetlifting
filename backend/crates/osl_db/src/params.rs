//! Input types owned by osl_db.
//!
//! Repositories take these rather than API request DTOs, so osl_db stays
//! independent of the HTTP layer. osl_api converts its validated request
//! bodies into these on the way in.

use rust_decimal::Decimal;
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
    pub gender: Option<String>,
    pub country: Option<String>,
    pub name: Option<String>,
    pub movement: RankingMovement,
    pub direction: SortDirection,
    /// Which event a total is ranked within. Ignored when ranking by a single
    /// movement, since those compare across events.
    pub event: String,
    /// Weight class suffix, e.g. `-73kg`, matched against the category name
    /// regardless of gender.
    pub category: Option<String>,
    pub year: Option<i32>,
    /// Narrows the ranking to one competition, e.g. for a per-meet leaderboard.
    pub competition_id: Option<Uuid>,
    pub offset: i64,
    pub limit: i64,
}

/// Score to upsert into `ris_scores_history`.
#[derive(Debug, Clone, Copy)]
pub struct RisScoreUpsert {
    pub participant_id: Uuid,
    pub formula_id: Uuid,
    pub ris_score: Decimal,
    pub bodyweight: Decimal,
    pub total_weight: Decimal,
}
