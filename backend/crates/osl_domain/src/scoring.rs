use serde::{Deserialize, Serialize};

/// The metric used to place athletes within a contest.
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scoring {
    Total,
    Ris,
}

impl Scoring {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Total => "total",
            Self::Ris => "ris",
        }
    }
}

impl std::str::FromStr for Scoring {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "total" => Ok(Self::Total),
            "ris" => Ok(Self::Ris),
            _ => Err(format!("unknown scoring '{value}', expected total or ris")),
        }
    }
}
