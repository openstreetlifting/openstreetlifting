use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecordMark {
    #[serde(rename = "WR")]
    World,
    #[serde(rename = "NR")]
    National,
}

impl RecordMark {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::World => "WR",
            Self::National => "NR",
        }
    }
}

impl std::fmt::Display for RecordMark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for RecordMark {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "WR" => Ok(Self::World),
            "NR" => Ok(Self::National),
            other => Err(format!("unknown record mark: {other}, expected WR or NR")),
        }
    }
}
