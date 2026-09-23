//! Redaction rewrites the canonical files, because those are what is published.

mod common;

use std::path::{Path, PathBuf};

use common::{athlete, competition, from, lifting, men_80, numbered, weighing};
use osl_domain::CountryCode;
use osl_importer::canonical::models::{AthleteData, CanonicalFormat, LiftData};
use osl_importer::canonical::store;
use osl_importer::identity::AthleteQuery;
use osl_importer::privacy::{self, Lookup, PrivacyList};
use osl_importer::redact;
use rust_decimal::Decimal;
use uuid::Uuid;

const KEY: &str = "test-key";

struct Workspace(PathBuf);

impl Workspace {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!("osl-redaction-{}", Uuid::new_v4()));
        std::fs::create_dir_all(root.join("competitions")).unwrap();
        std::fs::create_dir_all(root.join("athletes")).unwrap();
        Self(root)
    }

    fn competitions(&self) -> PathBuf {
        self.0.join("competitions")
    }

    fn instagram(&self) -> PathBuf {
        self.0.join("athletes/instagram.csv")
    }

    fn privacy(&self) -> PathBuf {
        self.0.join("athletes/privacy.csv")
    }

    fn list(&self) -> PrivacyList {
        PrivacyList::load_with_key(&self.privacy(), Some(KEY.to_string())).unwrap()
    }

    fn unkeyed_list(&self) -> PrivacyList {
        PrivacyList::load_with_key(&self.privacy(), None).unwrap()
    }

    fn write_competition(&self, canonical: &CanonicalFormat) -> PathBuf {
        let directory = self
            .competitions()
            .join("test-federation")
            .join(canonical.competition.start_date.format("%Y").to_string())
            .join(&canonical.competition.slug);

        std::fs::create_dir_all(&directory).unwrap();
        let mut canonical = canonical.clone();
        for category in &mut canonical.categories {
            for athlete in &mut category.athletes {
                athlete.total = athlete.total_from_lifts(&canonical.movements);
            }
        }
        store::write(&directory, &canonical).unwrap();
        directory
    }

    fn write_handles(&self, contents: &str) {
        std::fs::write(self.instagram(), contents).unwrap();
    }

    fn read_handles(&self) -> String {
        std::fs::read_to_string(self.instagram()).unwrap_or_default()
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn lifter(first: &str, last: &str) -> AthleteData {
    lifting(
        weighing(athlete(first, last), "78.5"),
        ["45", "65", "75", "130"],
    )
}

fn weights(lift: &LiftData) -> Vec<(i16, Decimal, bool)> {
    lift.attempts
        .iter()
        .flatten()
        .map(|attempt| {
            (
                attempt.attempt_number,
                attempt.weight,
                attempt.is_successful,
            )
        })
        .collect()
}

fn only_athlete(directory: &Path) -> AthleteData {
    let canonical = store::read(directory).unwrap();
    canonical.categories[0].athletes[0].clone()
}

fn query(name: &str, country: Option<&str>) -> AthleteQuery {
    AthleteQuery::new(name, None, country.map(str::to_string), None)
}

fn redact(workspace: &Workspace, name: &str, country: Option<&str>) -> redact::RedactionPlan {
    let mut list = workspace.list();
    let query = query(name, country);
    let plan = redact::plan(
        &workspace.competitions(),
        &workspace.instagram(),
        &list,
        &query,
        name,
    )
    .unwrap();

    redact::apply(&workspace.instagram(), &mut list, &query, &plan).unwrap();
    plan
}

#[test]
fn the_name_goes_and_the_result_stays() {
    let workspace = Workspace::new();
    let mut entry = lifter("Alina", "Riyaz");
    entry.native_name = Some("Алина Рияз".to_string());

    let directory = workspace.write_competition(&competition("meet", vec![men_80(vec![entry])]));

    let before = only_athlete(&directory);
    redact(&workspace, "Alina Riyaz", None);
    let after = only_athlete(&directory);

    assert_eq!(after.first_name, "");
    assert_eq!(after.last_name, "Redacted Athlete #1");
    assert_eq!(after.native_name, None);

    assert_eq!(after.bodyweight, before.bodyweight);
    assert_eq!(after.country, before.country);
    assert_eq!(after.status, before.status);
    assert_eq!(after.lifts.len(), before.lifts.len());

    for (after, before) in after.lifts.iter().zip(&before.lifts) {
        assert_eq!(after.movement, before.movement);
        assert_eq!(weights(after), weights(before));
    }
}

#[test]
fn the_file_no_longer_holds_the_name_anywhere() {
    let workspace = Workspace::new();
    let directory = workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));

    redact(&workspace, "Alina Riyaz", None);

    let written = std::fs::read_to_string(directory.join("entries.csv")).unwrap();
    assert!(!written.contains("Alina"), "{written}");
    assert!(!written.contains("Riyaz"), "{written}");
}

#[test]
fn a_name_that_names_two_people_is_refused() {
    let workspace = Workspace::new();
    let directory = workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![
            from(lifter("Tony", "Nguyen"), "FR"),
            numbered(from(lifter("Tony", "Nguyen"), "US"), 2),
        ])],
    ));

    let list = workspace.list();
    let problem = redact::plan(
        &workspace.competitions(),
        &workspace.instagram(),
        &list,
        &query("Tony Nguyen", None),
        "Tony Nguyen",
    )
    .unwrap_err()
    .to_string();

    assert!(problem.contains("names 2 athletes"), "{problem}");
    assert!(problem.contains("--country"), "{problem}");

    let untouched = store::read(&directory).unwrap();
    assert_eq!(untouched.categories[0].athletes[0].first_name, "Tony");
}

#[test]
fn narrowing_picks_the_one_that_was_meant() {
    let workspace = Workspace::new();
    let directory = workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![
            from(lifter("Tony", "Nguyen"), "FR"),
            numbered(from(lifter("Tony", "Nguyen"), "US"), 2),
        ])],
    ));

    redact(&workspace, "Tony Nguyen", Some("US"));

    let canonical = store::read(&directory).unwrap();
    let athletes = &canonical.categories[0].athletes;

    let french = athletes
        .iter()
        .find(|a| a.country == CountryCode::parse("FR").unwrap())
        .unwrap();
    let american = athletes
        .iter()
        .find(|a| a.country == CountryCode::parse("US").unwrap())
        .unwrap();

    assert_eq!(french.display_name(), "Tony Nguyen");
    assert_eq!(american.last_name, "Redacted Athlete #1");
}

#[test]
fn a_name_nobody_carries_is_refused() {
    let workspace = Workspace::new();
    workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));

    let list = workspace.list();
    let problem = redact::plan(
        &workspace.competitions(),
        &workspace.instagram(),
        &list,
        &query("Nobody Here", None),
        "Nobody Here",
    )
    .unwrap_err()
    .to_string();

    assert!(
        problem.contains("No athlete named 'Nobody Here'"),
        "{problem}"
    );
}

#[test]
fn every_competition_they_entered_is_rewritten() {
    let workspace = Workspace::new();
    let first = workspace.write_competition(&competition(
        "meet-one",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));
    let second = workspace.write_competition(&competition(
        "meet-two",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));

    let plan = redact(&workspace, "Alina Riyaz", None);

    assert_eq!(plan.competitions.len(), 2);
    assert_eq!(plan.entries, 2);
    assert_eq!(only_athlete(&first).last_name, "Redacted Athlete #1");
    assert_eq!(only_athlete(&second).last_name, "Redacted Athlete #1");
}

#[test]
fn the_handle_goes_with_the_name() {
    let workspace = Workspace::new();
    workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));
    workspace.write_handles(
        "Name,Sex,Country,Disambiguation,Instagram\n\
         Alina Riyaz,,,,alina_lifts\n\
         Someone Else,,,,someone\n",
    );

    let plan = redact(&workspace, "Alina Riyaz", None);
    let handles = workspace.read_handles();

    assert_eq!(plan.handles, 1);
    assert!(!handles.contains("alina_lifts"), "{handles}");
    assert!(handles.contains("Someone Else"), "{handles}");
}

#[test]
fn a_handle_for_someone_who_shares_the_name_is_left_alone() {
    let workspace = Workspace::new();
    workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![
            from(lifter("Tony", "Nguyen"), "FR"),
            numbered(from(lifter("Tony", "Nguyen"), "US"), 2),
        ])],
    ));
    workspace.write_handles(
        "Name,Sex,Country,Disambiguation,Instagram\n\
         Tony Nguyen,,FR,,tony_fr\n\
         Tony Nguyen,,US,2,tony_us\n",
    );

    redact(&workspace, "Tony Nguyen", Some("US"));
    let handles = workspace.read_handles();

    assert!(handles.contains("tony_fr"), "{handles}");
    assert!(!handles.contains("tony_us"), "{handles}");
}

#[test]
fn a_later_competition_naming_them_is_refused() {
    let workspace = Workspace::new();
    workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));
    redact(&workspace, "Alina Riyaz", None);

    let next = competition("next-year", vec![men_80(vec![lifter("Alina", "Riyaz")])]);
    let problem = privacy::check_competition(&next, &workspace.list())
        .unwrap_err()
        .to_string();

    assert!(
        problem.contains("asked to be taken off the site"),
        "{problem}"
    );
    assert!(problem.contains("Redacted Athlete #1"), "{problem}");
}

#[test]
fn everybody_else_still_imports() {
    let workspace = Workspace::new();
    workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));
    redact(&workspace, "Alina Riyaz", None);

    let other = competition("other", vec![men_80(vec![lifter("Lea", "Merandon")])]);

    assert!(privacy::check_competition(&other, &workspace.list()).is_ok());
}

#[test]
fn the_redaction_is_recorded_without_the_name() {
    let workspace = Workspace::new();
    workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));
    redact(&workspace, "Alina Riyaz", None);

    let recorded = std::fs::read_to_string(workspace.privacy()).unwrap();

    assert!(!recorded.to_lowercase().contains("alina"), "{recorded}");
    assert!(!recorded.to_lowercase().contains("riyaz"), "{recorded}");
    assert!(
        recorded.contains("Hash,Sex,Country,Disambiguation,RedactedId"),
        "{recorded}"
    );
}

#[test]
fn the_list_recognises_the_name_it_cannot_show() {
    let workspace = Workspace::new();
    workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));
    redact(&workspace, "Alina Riyaz", None);

    let list = workspace.list();

    assert!(matches!(
        list.lookup("Alina Riyaz", "M", "FR", None),
        Lookup::Listed(_)
    ));
    assert!(matches!(
        list.lookup("ALINA RIYAZ", "M", "FR", None),
        Lookup::Listed(_)
    ));
    assert_eq!(
        list.lookup("Lea Merandon", "M", "FR", None),
        Lookup::NotListed
    );
}

#[test]
fn without_the_key_the_list_says_so_rather_than_passing() {
    let workspace = Workspace::new();
    workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));
    redact(&workspace, "Alina Riyaz", None);

    let list = workspace.unkeyed_list();

    assert!(!list.has_key());
    assert_eq!(list.len(), 1);
    assert_eq!(
        list.lookup("Alina Riyaz", "M", "FR", None),
        Lookup::MissingKey
    );
}

#[test]
fn without_the_key_nothing_is_redacted() {
    let workspace = Workspace::new();
    workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));

    let list = workspace.unkeyed_list();
    let problem = redact::plan(
        &workspace.competitions(),
        &workspace.instagram(),
        &list,
        &query("Alina Riyaz", None),
        "Alina Riyaz",
    )
    .unwrap_err()
    .to_string();

    assert!(problem.contains(privacy::KEY_ENV), "{problem}");
}

#[test]
fn two_redactions_get_two_numbers() {
    let workspace = Workspace::new();
    workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![
            lifter("Alina", "Riyaz"),
            lifter("Lea", "Merandon"),
        ])],
    ));

    assert_eq!(redact(&workspace, "Alina Riyaz", None).redacted.id(), 1);
    assert_eq!(redact(&workspace, "Lea Merandon", None).redacted.id(), 2);
    assert_eq!(workspace.list().len(), 2);
}

#[test]
fn allocation_has_no_reserved_range() {
    let workspace = Workspace::new();
    let mut list = workspace.list();
    list.append(privacy::PrivacyEntry {
        hash: list.hash("Fixture Person").unwrap(),
        query: query("", None),
        redacted: osl_domain::redaction::RedactedAthlete::new(9000).unwrap(),
    })
    .unwrap();
    workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![
            lifter("Alina", "Riyaz"),
            lifter("Lea", "Merandon"),
        ])],
    ));
    assert_eq!(redact(&workspace, "Alina Riyaz", None).redacted.id(), 9001);
    assert_eq!(redact(&workspace, "Lea Merandon", None).redacted.id(), 9002);
}

#[test]
fn a_returning_athlete_keeps_their_replacement_and_retry_is_a_noop() {
    let workspace = Workspace::new();
    let first = workspace.write_competition(&competition(
        "first",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));
    redact(&workspace, "Alina Riyaz", None);
    let later = workspace.write_competition(&competition(
        "later",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));
    assert_eq!(redact(&workspace, "Alina Riyaz", None).redacted.id(), 1);
    assert_eq!(
        only_athlete(&first).last_name,
        only_athlete(&later).last_name
    );
    let recorded = std::fs::read(workspace.privacy()).unwrap();
    let retry = redact(&workspace, "Alina Riyaz", None);
    assert_eq!(retry.entries, 0);
    assert_eq!(workspace.list().len(), 1);
    assert_eq!(std::fs::read(workspace.privacy()).unwrap(), recorded);
}

#[test]
fn missing_and_incorrect_keys_cannot_approve_an_import() {
    let workspace = Workspace::new();
    let original = competition("meet", vec![men_80(vec![lifter("Alina", "Riyaz")])]);
    workspace.write_competition(&original);
    redact(&workspace, "Alina Riyaz", None);
    assert!(privacy::check_competition(&original, &workspace.unkeyed_list()).is_err());
    for key in ["wrong-key", ""] {
        let result = PrivacyList::load_with_key(&workspace.privacy(), Some(key.into()));
        assert!(result.is_err() || result.unwrap().require_key().is_err());
    }
}

#[test]
fn instagram_cleanup_respects_headers_and_preserves_remaining_columns() {
    for contents in [
        "Name,Instagram\nAlina Riyaz,alina\nSomeone Else,someone\n",
        "Instagram,Country,Name,Sex\nalina,fr,Alina Riyaz,m\nsomeone,FR,Someone Else,M\n",
    ] {
        let workspace = Workspace::new();
        workspace.write_competition(&competition(
            "meet",
            vec![men_80(vec![lifter("Alina", "Riyaz")])],
        ));
        workspace.write_handles(contents);
        assert_eq!(redact(&workspace, "Alina Riyaz", None).handles, 1);
        let remaining = workspace.read_handles();
        assert!(!remaining.contains("Alina"));
        assert!(remaining.contains("Someone Else"));
        assert_eq!(remaining.lines().next(), contents.lines().next());
        assert_eq!(
            osl_importer::social::validate_file(&workspace.instagram()).unwrap(),
            1
        );
    }
}

#[test]
fn malformed_handles_leave_competitions_and_suppression_untouched() {
    let workspace = Workspace::new();
    let directory = workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));
    workspace.write_handles("Name,Instagram\nSomeone Else,not a handle\n");
    assert!(
        redact::plan(
            &workspace.competitions(),
            &workspace.instagram(),
            &workspace.list(),
            &query("Alina Riyaz", None),
            "Alina Riyaz"
        )
        .is_err()
    );
    assert_eq!(only_athlete(&directory).display_name(), "Alina Riyaz");
    assert!(!workspace.privacy().exists());
}

#[test]
fn failure_to_record_suppression_does_not_rewrite_results_or_handles() {
    let workspace = Workspace::new();
    let directory = workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));
    workspace.write_handles("Name,Instagram\nAlina Riyaz,alina\n");
    let mut list = workspace.list();
    let query = query("Alina Riyaz", None);
    let plan = redact::plan(
        &workspace.competitions(),
        &workspace.instagram(),
        &list,
        &query,
        "Alina Riyaz",
    )
    .unwrap();
    std::fs::create_dir(workspace.privacy()).unwrap();
    assert!(redact::apply(&workspace.instagram(), &mut list, &query, &plan).is_err());
    assert_eq!(only_athlete(&directory).display_name(), "Alina Riyaz");
    assert!(workspace.read_handles().contains("Alina Riyaz"));
    assert!(list.is_empty());
}

#[test]
fn an_interrupted_rewrite_can_resume_using_its_recorded_identity() {
    let workspace = Workspace::new();
    let first = workspace.write_competition(&competition(
        "first",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));
    redact(&workspace, "Alina Riyaz", None);
    let pending = workspace.write_competition(&competition(
        "pending",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));
    workspace.write_handles("Name,Instagram\nAlina Riyaz,alina\n");
    assert!(
        privacy::check_competition(&store::read(&pending).unwrap(), &workspace.list()).is_err()
    );
    redact(&workspace, "Alina Riyaz", None);
    assert_eq!(
        only_athlete(&first).last_name,
        only_athlete(&pending).last_name
    );
    assert!(!workspace.read_handles().contains("alina"));
    assert_eq!(workspace.list().len(), 1);
}

#[test]
fn an_interruption_after_all_results_still_allows_handle_cleanup() {
    let workspace = Workspace::new();
    workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));
    redact(&workspace, "Alina Riyaz", None);
    workspace.write_handles("Name,Instagram\nAlina Riyaz,alina\n");
    let resumed = redact(&workspace, "Alina Riyaz", None);
    assert_eq!(resumed.entries, 0);
    assert_eq!(resumed.handles, 1);
    assert!(!workspace.read_handles().contains("alina"));
}

#[test]
fn a_replacement_cannot_carry_a_native_name() {
    let mut replacement = lifter("", "Redacted Athlete #1");
    replacement.native_name = Some("Фикстур Альфа".into());
    let canonical = competition("meet", vec![men_80(vec![replacement])]);
    let error =
        osl_importer::canonical::validator::CanonicalValidator::validate(&canonical).unwrap_err();
    assert!(error.to_string().contains("NativeName"));
}

#[test]
fn equivalent_replacement_names_cannot_carry_social_handles() {
    let workspace = Workspace::new();
    for name in [
        "Redacted Athlete #1",
        "redacted athlete #1",
        "Redacted Athlete 1",
        "REDACTED ATHLETE #1",
    ] {
        workspace.write_handles(&format!("Name,Instagram\n{name},fixture_handle\n"));
        assert!(
            osl_importer::social::validate_file(&workspace.instagram()).is_err(),
            "{name}"
        );
    }
}

#[test]
fn an_unnumbered_suppression_does_not_match_another_numbered_person() {
    let workspace = Workspace::new();
    workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));
    redact(&workspace, "Alina Riyaz", None);
    let other = competition(
        "other",
        vec![men_80(vec![numbered(lifter("Alina", "Riyaz"), 2)])],
    );
    assert!(privacy::check_competition(&other, &workspace.list()).is_ok());
}

#[test]
fn the_last_available_id_does_not_overflow() {
    let workspace = Workspace::new();
    let mut list = workspace.list();
    list.append(privacy::PrivacyEntry {
        hash: list.hash("Fixture Person").unwrap(),
        query: query("", None),
        redacted: osl_domain::redaction::RedactedAthlete::new(u32::MAX).unwrap(),
    })
    .unwrap();
    assert!(list.next_id().is_err());
}

#[test]
fn cli_requires_verified_key_and_only_allows_bypass_for_validation() {
    let workspace = Workspace::new();
    let directory = workspace.write_competition(&competition(
        "meet",
        vec![men_80(vec![lifter("Alina", "Riyaz")])],
    ));
    redact(&workspace, "Alina Riyaz", None);
    let run = |key: Option<&str>, args: &[&str]| {
        let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_import"));
        command
            .current_dir(&workspace.0)
            .env_remove("DATABASE_URL")
            .env_remove(privacy::KEY_ENV);
        if let Some(key) = key {
            command.env(privacy::KEY_ENV, key);
        }
        command
            .arg("--privacy-file")
            .arg(workspace.privacy())
            .args(args)
            .output()
            .unwrap()
    };
    let directory = directory.to_str().unwrap();
    for key in [None, Some("wrong-key")] {
        assert!(
            !run(key, &["competitions", directory, "--dry-run"])
                .status
                .success()
        );
        assert!(
            !run(key, &["privacy", "--directory", directory])
                .status
                .success()
        );
    }
    assert!(
        run(Some(KEY), &["competitions", directory, "--dry-run"])
            .status
            .success()
    );
    assert!(
        run(
            None,
            &[
                "competitions",
                directory,
                "--dry-run",
                "--skip-privacy-check"
            ]
        )
        .status
        .success()
    );
    let refused = run(None, &["competitions", directory, "--skip-privacy-check"]);
    assert!(!refused.status.success());
    assert!(String::from_utf8_lossy(&refused.stderr).contains("--dry-run"));
}
