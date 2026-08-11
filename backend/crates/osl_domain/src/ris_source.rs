use serde::{Deserialize, Serialize};

/// Where a RIS score came from.
///
/// A computed score can be reproduced and recomputed when a new formula
/// version lands. A reported one cannot: the source stated a number without
/// the bodyweight, so neither the formula nor the year behind it is known.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RisSource {
    Computed,
    Reported,
}

impl RisSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Computed => "computed",
            Self::Reported => "reported",
        }
    }

    pub fn is_reported(&self) -> bool {
        matches!(self, Self::Reported)
    }
}

impl std::fmt::Display for RisSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for RisSource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "computed" => Ok(Self::Computed),
            "reported" => Ok(Self::Reported),
            other => Err(format!(
                "unknown ris source: {}, expected computed or reported",
                other
            )),
        }
    }
}
