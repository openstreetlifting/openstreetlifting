use serde::{Deserialize, Serialize};

/// Ranking metric: `total` or `ris`.
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RankBy {
    Total,
    Ris,
}

impl RankBy {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Total => "total",
            Self::Ris => "ris",
        }
    }
}

impl std::str::FromStr for RankBy {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "total" => Ok(Self::Total),
            "ris" => Ok(Self::Ris),
            _ => Err(format!("unknown rank_by '{value}', expected total or ris")),
        }
    }
}
