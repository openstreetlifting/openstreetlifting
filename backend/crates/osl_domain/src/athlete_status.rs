use serde::{Deserialize, Serialize};

/// Outcome of an athlete's participation in a competition.
///
/// Mirrors what the schema can express today: `competition_participants`
/// stores `is_disqualified` plus a free-text `disqualified_reason`.
/// Additional variants (did-not-start, did-not-finish) land with the
/// canonical format 1.1.0 migration, which introduces a real status column.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AthleteStatus {
    /// The athlete completed the competition and their result stands.
    Competed,
    /// The athlete was disqualified; the reason is carried alongside.
    Disqualified,
}

impl AthleteStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Competed => "competed",
            Self::Disqualified => "disqualified",
        }
    }

    pub fn is_disqualified(&self) -> bool {
        matches!(self, Self::Disqualified)
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
            other => Err(format!("unknown athlete status: {}", other)),
        }
    }
}
