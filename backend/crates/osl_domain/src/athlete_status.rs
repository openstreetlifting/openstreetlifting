use serde::{Deserialize, Serialize};

/// Outcome of an athlete's participation in a competition.
///
/// Only `Competed` counts for rankings and records. `Disqualified` covers a
/// lifter who took attempts and was ruled out, whether by failing every attempt
/// in a movement or by a procedural call; `NoShow` covers an entrant who never
/// lifted at all, which has no bodyweight and no attempts behind it.
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

    /// Whether the result stands, which is what every ranking and record asks.
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
