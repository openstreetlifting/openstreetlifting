use osl_domain::{AthleteStatus, CompetitionStatus, Gender, Movement, RisSource};

fn wire(value: impl serde::Serialize) -> String {
    serde_json::to_string(&value).unwrap()
}

/// The spellings are the published contract, so a change that breaks every
/// client has to break this test first.
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

    assert_eq!(wire(Movement::MuscleUp), "\"Muscle-up\"");
    assert_eq!(wire(Movement::PullUp), "\"Pull-up\"");
    assert_eq!(wire(Movement::Dips), "\"Dips\"");
    assert_eq!(wire(Movement::Squat), "\"Squat\"");
}

/// Serde is what clients read and `as_str` is what gets bound into Postgres.
#[test]
fn the_database_spells_them_the_same_way() {
    for gender in [
        osl_domain::Gender::M,
        osl_domain::Gender::F,
        osl_domain::Gender::Mx,
    ] {
        assert_eq!(wire(gender), format!("\"{}\"", gender.as_str()));
    }

    for status in [
        osl_domain::AthleteStatus::Competed,
        osl_domain::AthleteStatus::Disqualified,
        osl_domain::AthleteStatus::NoShow,
    ] {
        assert_eq!(wire(status), format!("\"{}\"", status.as_str()));
    }

    for status in [
        osl_domain::CompetitionStatus::Draft,
        osl_domain::CompetitionStatus::Upcoming,
        osl_domain::CompetitionStatus::Live,
        osl_domain::CompetitionStatus::Completed,
        osl_domain::CompetitionStatus::Cancelled,
    ] {
        assert_eq!(wire(status), format!("\"{}\"", status.as_str()));
    }

    for source in [
        osl_domain::RisSource::Computed,
        osl_domain::RisSource::Reported,
    ] {
        assert_eq!(wire(source), format!("\"{}\"", source.as_str()));
    }
}
