//! The domain's closed vocabularies, as they appear on the wire.
//!
//! These mirror `osl_domain` rather than reusing it. The spellings here are a
//! published contract that clients and the OpenAPI document both depend on, so
//! they should not be able to change because someone renamed a variant inside
//! the domain for a reason of its own. Mirroring turns that into a compile error
//! in the `From` impl below, which is the whole point of paying for the
//! duplication. It also keeps `utoipa`, which is purely an HTTP concern, out of
//! `osl_domain`.
//!
//! Add a variant on both sides or neither: the `From` impls are exhaustive, so
//! the compiler will not let one drift ahead of the other.

use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;

/// Which category an athlete competes in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum Gender {
    M,
    F,
    Mx,
}

/// Where a competition is in its life.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum CompetitionStatus {
    Draft,
    Upcoming,
    Live,
    Completed,
    Cancelled,
}

/// Outcome of an athlete's participation in a competition. `competed` is the
/// only one whose result stands for rankings and records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AthleteStatus {
    Competed,
    Disqualified,
    NoShow,
}

/// Where a RIS score came from. `computed` was worked out from the athlete's
/// bodyweight and total. `reported` was stated by the source, which gave no
/// bodyweight, so it cannot be restated on the formula everything else uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum RisSource {
    Computed,
    Reported,
}

/// Reads a query parameter or request field through the domain's own parser.
///
/// `Gender` and `CompetitionStatus` arrive from callers, and the domain accepts
/// them trimmed and in any case. Going through `FromStr` rather than serde's
/// exact variant match keeps that, so no request that worked before this type
/// existed starts failing. The response is still only ever the canonical
/// spelling, which is what `Serialize` and the schema describe.
fn parse<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr<Err = String>,
{
    let raw = String::deserialize(deserializer)?;

    T::from_str(&raw).map_err(serde::de::Error::custom)
}

impl<'de> Deserialize<'de> for Gender {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        parse::<D, osl_domain::Gender>(deserializer).map(Self::from)
    }
}

impl<'de> Deserialize<'de> for CompetitionStatus {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        parse::<D, osl_domain::CompetitionStatus>(deserializer).map(Self::from)
    }
}

impl From<osl_domain::Gender> for Gender {
    fn from(gender: osl_domain::Gender) -> Self {
        match gender {
            osl_domain::Gender::M => Self::M,
            osl_domain::Gender::F => Self::F,
            osl_domain::Gender::Mx => Self::Mx,
        }
    }
}

impl From<Gender> for osl_domain::Gender {
    fn from(gender: Gender) -> Self {
        match gender {
            Gender::M => Self::M,
            Gender::F => Self::F,
            Gender::Mx => Self::Mx,
        }
    }
}

impl From<osl_domain::CompetitionStatus> for CompetitionStatus {
    fn from(status: osl_domain::CompetitionStatus) -> Self {
        match status {
            osl_domain::CompetitionStatus::Draft => Self::Draft,
            osl_domain::CompetitionStatus::Upcoming => Self::Upcoming,
            osl_domain::CompetitionStatus::Live => Self::Live,
            osl_domain::CompetitionStatus::Completed => Self::Completed,
            osl_domain::CompetitionStatus::Cancelled => Self::Cancelled,
        }
    }
}

impl From<CompetitionStatus> for osl_domain::CompetitionStatus {
    fn from(status: CompetitionStatus) -> Self {
        match status {
            CompetitionStatus::Draft => Self::Draft,
            CompetitionStatus::Upcoming => Self::Upcoming,
            CompetitionStatus::Live => Self::Live,
            CompetitionStatus::Completed => Self::Completed,
            CompetitionStatus::Cancelled => Self::Cancelled,
        }
    }
}

impl From<osl_domain::AthleteStatus> for AthleteStatus {
    fn from(status: osl_domain::AthleteStatus) -> Self {
        match status {
            osl_domain::AthleteStatus::Competed => Self::Competed,
            osl_domain::AthleteStatus::Disqualified => Self::Disqualified,
            osl_domain::AthleteStatus::NoShow => Self::NoShow,
        }
    }
}

impl From<osl_domain::RisSource> for RisSource {
    fn from(source: osl_domain::RisSource) -> Self {
        match source {
            osl_domain::RisSource::Computed => Self::Computed,
            osl_domain::RisSource::Reported => Self::Reported,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The spellings are the published contract, so they are asserted here
    /// rather than left to whatever `rename_all` happens to produce. A change
    /// that breaks every client should have to break this test first.
    #[test]
    fn the_wire_spellings_are_fixed() {
        let cases = [
            (serde_json::to_string(&Gender::M).unwrap(), "\"M\""),
            (serde_json::to_string(&Gender::F).unwrap(), "\"F\""),
            (serde_json::to_string(&Gender::Mx).unwrap(), "\"MX\""),
            (
                serde_json::to_string(&AthleteStatus::Competed).unwrap(),
                "\"competed\"",
            ),
            (
                serde_json::to_string(&AthleteStatus::Disqualified).unwrap(),
                "\"disqualified\"",
            ),
            (
                serde_json::to_string(&AthleteStatus::NoShow).unwrap(),
                "\"no_show\"",
            ),
            (
                serde_json::to_string(&CompetitionStatus::Draft).unwrap(),
                "\"draft\"",
            ),
            (
                serde_json::to_string(&CompetitionStatus::Upcoming).unwrap(),
                "\"upcoming\"",
            ),
            (
                serde_json::to_string(&CompetitionStatus::Live).unwrap(),
                "\"live\"",
            ),
            (
                serde_json::to_string(&CompetitionStatus::Completed).unwrap(),
                "\"completed\"",
            ),
            (
                serde_json::to_string(&CompetitionStatus::Cancelled).unwrap(),
                "\"cancelled\"",
            ),
            (
                serde_json::to_string(&RisSource::Computed).unwrap(),
                "\"computed\"",
            ),
            (
                serde_json::to_string(&RisSource::Reported).unwrap(),
                "\"reported\"",
            ),
        ];

        for (actual, expected) in cases {
            assert_eq!(actual, expected);
        }
    }

    /// A mirror is only useful while it still matches, so every variant has to
    /// survive the round trip through the domain and back.
    #[test]
    fn every_variant_round_trips_through_the_domain() {
        for gender in [Gender::M, Gender::F, Gender::Mx] {
            assert_eq!(Gender::from(osl_domain::Gender::from(gender)), gender);
        }

        for status in [
            CompetitionStatus::Draft,
            CompetitionStatus::Upcoming,
            CompetitionStatus::Live,
            CompetitionStatus::Completed,
            CompetitionStatus::Cancelled,
        ] {
            assert_eq!(
                CompetitionStatus::from(osl_domain::CompetitionStatus::from(status)),
                status
            );
        }
    }

    /// The domain trims and ignores case, and dropping that would break
    /// requests that work today.
    #[test]
    fn input_is_read_as_leniently_as_the_domain_reads_it() {
        assert_eq!(
            serde_json::from_str::<Gender>("\" f \"").unwrap(),
            Gender::F
        );
        assert_eq!(
            serde_json::from_str::<CompetitionStatus>("\"COMPLETED\"").unwrap(),
            CompetitionStatus::Completed
        );
        assert!(serde_json::from_str::<Gender>("\"nonsense\"").is_err());
    }
}
