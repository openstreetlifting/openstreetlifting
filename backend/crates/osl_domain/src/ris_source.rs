use serde::{Deserialize, Serialize};

/// Where a RIS score came from. `computed` was worked out from the athlete's
/// bodyweight and total. `reported` was stated by the source, which gave no
/// bodyweight, so it cannot be restated on the formula everything else uses.
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
