use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Gender {
    #[serde(rename = "M")]
    M,
    #[serde(rename = "F")]
    F,
    #[serde(rename = "MX")]
    Mx,
}

impl Gender {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::M => "M",
            Self::F => "F",
            Self::Mx => "MX",
        }
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Gender {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_uppercase().as_str() {
            "M" => Ok(Self::M),
            "F" => Ok(Self::F),
            "MX" => Ok(Self::Mx),
            other => Err(format!("unknown gender: {}, expected M, F or MX", other)),
        }
    }
}
