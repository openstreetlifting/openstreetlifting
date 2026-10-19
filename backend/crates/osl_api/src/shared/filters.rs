use osl_domain::Gender;
use serde::{Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum RankedGender {
    M,
    F,
    Mx,
}

impl From<RankedGender> for Gender {
    fn from(gender: RankedGender) -> Self {
        match gender {
            RankedGender::M => Self::M,
            RankedGender::F => Self::F,
            RankedGender::Mx => Self::Mx,
        }
    }
}

impl<'de> Deserialize<'de> for RankedGender {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match crate::shared::dto::from_str::<D, Gender>(deserializer)? {
            Gender::M => Ok(Self::M),
            Gender::F => Ok(Self::F),
            Gender::Mx => Ok(Self::Mx),
        }
    }
}
