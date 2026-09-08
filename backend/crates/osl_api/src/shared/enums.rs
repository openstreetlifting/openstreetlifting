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

/// The genders a ranking can be drawn for.
///
/// Narrower than `Gender` on purpose: weight classes are only drawn for men and
/// women, so a mixed board would compare an athlete against an empty field.
/// Keeping that in the type rather than in a `validate` call is what makes the
/// schema advertise `M` and `F` for the parameter instead of advertising `MX`
/// and then refusing it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum RankedGender {
    M,
    F,
}

impl From<RankedGender> for Gender {
    fn from(gender: RankedGender) -> Self {
        match gender {
            RankedGender::M => Self::M,
            RankedGender::F => Self::F,
        }
    }
}

impl From<RankedGender> for osl_domain::Gender {
    fn from(gender: RankedGender) -> Self {
        Gender::from(gender).into()
    }
}

impl<'de> Deserialize<'de> for RankedGender {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match parse::<D, osl_domain::Gender>(deserializer)? {
            osl_domain::Gender::M => Ok(Self::M),
            osl_domain::Gender::F => Ok(Self::F),
            osl_domain::Gender::Mx => Err(serde::de::Error::custom("gender must be 'M' or 'F'")),
        }
    }
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

    fn wire(value: impl serde::Serialize) -> String {
        serde_json::to_string(&value).unwrap()
    }

    /// The spellings are the published contract, so they are asserted rather
    /// than left to whatever `rename_all` happens to produce. A change that
    /// breaks every client should have to break this test first.
    #[test]
    fn the_wire_spellings_are_fixed() {
        assert_eq!(wire(Gender::M), "\"M\"");
        assert_eq!(wire(Gender::F), "\"F\"");
        assert_eq!(wire(Gender::Mx), "\"MX\"");

        assert_eq!(wire(AthleteStatus::Competed), "\"competed\"");
        assert_eq!(wire(AthleteStatus::Disqualified), "\"disqualified\"");
        assert_eq!(wire(AthleteStatus::NoShow), "\"no_show\"");

        assert_eq!(wire(CompetitionStatus::Draft), "\"draft\"");
        assert_eq!(wire(CompetitionStatus::Upcoming), "\"upcoming\"");
        assert_eq!(wire(CompetitionStatus::Live), "\"live\"");
        assert_eq!(wire(CompetitionStatus::Completed), "\"completed\"");
        assert_eq!(wire(CompetitionStatus::Cancelled), "\"cancelled\"");

        assert_eq!(wire(RisSource::Computed), "\"computed\"");
        assert_eq!(wire(RisSource::Reported), "\"reported\"");
    }

    /// The mirror is what clients read; the domain's `as_str` is what gets
    /// bound into Postgres. Nothing in the type system ties the two together,
    /// so without this a rename on one side would leave the API answering `MX`
    /// while the ranking filter searched the column for something else.
    #[test]
    fn the_database_spells_them_the_same_way() {
        for gender in [
            osl_domain::Gender::M,
            osl_domain::Gender::F,
            osl_domain::Gender::Mx,
        ] {
            assert_eq!(
                wire(Gender::from(gender)),
                format!("\"{}\"", gender.as_str())
            );
        }

        for status in [
            osl_domain::AthleteStatus::Competed,
            osl_domain::AthleteStatus::Disqualified,
            osl_domain::AthleteStatus::NoShow,
        ] {
            assert_eq!(
                wire(AthleteStatus::from(status)),
                format!("\"{}\"", status.as_str())
            );
        }

        for status in [
            osl_domain::CompetitionStatus::Draft,
            osl_domain::CompetitionStatus::Upcoming,
            osl_domain::CompetitionStatus::Live,
            osl_domain::CompetitionStatus::Completed,
            osl_domain::CompetitionStatus::Cancelled,
        ] {
            assert_eq!(
                wire(CompetitionStatus::from(status)),
                format!("\"{}\"", status.as_str())
            );
        }

        for source in [
            osl_domain::RisSource::Computed,
            osl_domain::RisSource::Reported,
        ] {
            assert_eq!(
                wire(RisSource::from(source)),
                format!("\"{}\"", source.as_str())
            );
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
