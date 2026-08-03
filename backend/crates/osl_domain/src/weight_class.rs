use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WeightClassSlug {
    #[serde(rename = "F-52")]
    F52,
    #[serde(rename = "F-57")]
    F57,
    #[serde(rename = "F-63")]
    F63,
    #[serde(rename = "F-70")]
    F70,
    #[serde(rename = "F+70")]
    FPlus70,
    #[serde(rename = "M-66")]
    M66,
    #[serde(rename = "M-73")]
    M73,
    #[serde(rename = "M-80")]
    M80,
    #[serde(rename = "M-87")]
    M87,
    #[serde(rename = "M-94")]
    M94,
    #[serde(rename = "M-101")]
    M101,
    #[serde(rename = "M+101")]
    MPlus101,
}

impl WeightClassSlug {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::F52 => "F-52",
            Self::F57 => "F-57",
            Self::F63 => "F-63",
            Self::F70 => "F-70",
            Self::FPlus70 => "F+70",
            Self::M66 => "M-66",
            Self::M73 => "M-73",
            Self::M80 => "M-80",
            Self::M87 => "M-87",
            Self::M94 => "M-94",
            Self::M101 => "M-101",
            Self::MPlus101 => "M+101",
        }
    }
}

impl std::fmt::Display for WeightClassSlug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for WeightClassSlug {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "F-52" => Ok(Self::F52),
            "F-57" => Ok(Self::F57),
            "F-63" => Ok(Self::F63),
            "F-70" => Ok(Self::F70),
            "F+70" => Ok(Self::FPlus70),
            "M-66" => Ok(Self::M66),
            "M-73" => Ok(Self::M73),
            "M-80" => Ok(Self::M80),
            "M-87" => Ok(Self::M87),
            "M-94" => Ok(Self::M94),
            "M-101" => Ok(Self::M101),
            "M+101" => Ok(Self::MPlus101),
            other => Err(format!("unknown weight class slug: {}", other)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightClass {
    pub id: Uuid,
    pub slug: WeightClassSlug,
    pub gender: String,
    pub limit_kg: Option<Decimal>,
    pub is_plus: bool,
}
