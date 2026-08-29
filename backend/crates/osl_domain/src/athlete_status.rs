use serde::{Deserialize, Serialize};

/// Outcome of an athlete's participation in a competition.
///
/// `Competed` counts for rankings and records.
/// `Disqualified` covers a athlete who took attempts and was disqualified, either by 'bombing'
/// movement or by a judge call.
/// `NoShow` covers an athlete that has not competed
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AthleteStatus {
    Competed,
    Disqualified,
    NoShow,
}

impl AthleteStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Competed => "competed",
            Self::Disqualified => "disqualified",
            Self::NoShow => "no_show",
        }
    }

    pub fn competed(&self) -> bool {
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
            other => Err(format!("unknown athlete status: {}", other)),
        }
    }
}
