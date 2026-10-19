use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AthleteStatus {
    Competed,
    // took attempts but was disqualified by bombing or a judge call.
    Disqualified,
    // did not compete.
    NoShow,
}

impl AthleteStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Competed => "competed",
            Self::Disqualified => "disqualified",
            Self::NoShow => "no_show",
        }
    }

    #[must_use]
    pub const fn competed(&self) -> bool {
        matches!(self, Self::Competed)
    }
}

impl std::fmt::Display for AthleteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for AthleteStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "competed" => Ok(Self::Competed),
            "disqualified" => Ok(Self::Disqualified),
            "no_show" => Ok(Self::NoShow),
            other => Err(format!("unknown athlete status: {other}")),
        }
    }
}
