use osl_domain::Edition;
use osl_importer::canonical::{models::BodyweightSource, store};
use std::{
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

struct Fixture(PathBuf);

impl Fixture {
    fn new(rows: &str) -> Self {
        let directory = std::env::temp_dir().join(format!(
            "osl-recovery-cli-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&directory).unwrap();
        std::fs::write(directory.join("competition.toml"), "event = \"MPDS\"\nsources = []\n[competition]\nname = \"Recovery test\"\nstart_date = \"2023-09-30\"\nend_date = \"2023-09-30\"\ncountry = \"FR\"\nstatus = \"completed\"\n[federation]\nname = \"Test federation\"\n").unwrap();
        let header = "Sex,WeightClassKg,FirstName,LastName,Disambiguation,Country,BodyweightKg,Ris,Status,StatusReason,MuscleUp1Kg,MuscleUp2Kg,MuscleUp3Kg,BestMuscleUpKg,PullUp1Kg,PullUp2Kg,PullUp3Kg,BestPullUpKg,Dips1Kg,Dips2Kg,Dips3Kg,BestDipsKg,Squat1Kg,Squat2Kg,Squat3Kg,BestSquatKg\n";
        std::fs::write(directory.join("entries.csv"), format!("{header}{rows}")).unwrap();
        Self(directory)
    }

    fn run(&self, check: bool) -> String {
        let mut command = Command::new(env!("CARGO_BIN_EXE_recover-bodyweight"));
        command.args(["--edition", "2024"]).arg(&self.0);
        if check {
            command.arg("--check");
        }
        let output = command.output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

const VALID: &str = "M,94,Xavier,Macias,,FR,,113.43,competed,,,,,50,,,,110,,,,175,,,,251.5\n";

#[test]
fn one_candidate_recovers_and_keeps_original_score_while_check_is_read_only() {
    let fixture = Fixture::new(VALID);
    let csv = std::fs::read(fixture.0.join("entries.csv")).unwrap();
    let metadata = std::fs::read(fixture.0.join("competition.toml")).unwrap();
    assert!(fixture.run(true).contains("would recover 1 bodyweight(s)"));
    assert_eq!(std::fs::read(fixture.0.join("entries.csv")).unwrap(), csv);
    assert_eq!(
        std::fs::read(fixture.0.join("competition.toml")).unwrap(),
        metadata
    );
    assert!(fixture.run(false).contains("recovered 1 bodyweight(s)"));
    let canonical = store::read(&fixture.0).unwrap();
    let athlete = &canonical.categories[0].athletes[0];
    assert_eq!(athlete.bodyweight.unwrap().to_string(), "91.7");
    assert_eq!(athlete.ris.unwrap().to_string(), "113.43");
    assert_eq!(athlete.bodyweight_source, Some(BodyweightSource::Recovered));
    assert_eq!(athlete.reported_ris_edition, Some(Edition::V2024));
    let recovered_csv = std::fs::read(fixture.0.join("entries.csv")).unwrap();
    assert!(fixture.run(false).contains("1 ineligible"));
    assert_eq!(
        std::fs::read(fixture.0.join("entries.csv")).unwrap(),
        recovered_csv
    );
}

#[test]
fn incomplete_total_is_rejected_and_valid_rows_are_reported_as_withheld() {
    let incomplete = VALID
        .replace("Xavier,Macias", "John,Doe")
        .replace("251.5\n", "\n");
    let ineligible = VALID
        .replace("Xavier,Macias", "Jane,Doe")
        .replace("113.43", "");
    let fixture = Fixture::new(&format!("{VALID}{incomplete}{ineligible}"));
    let original = std::fs::read(fixture.0.join("entries.csv")).unwrap();
    let output = fixture.run(false);
    assert!(output.contains("missing Squat result"), "{output}");
    assert!(
        output.contains("1 rejected, 1 withheld, 1 ineligible"),
        "{output}"
    );
    assert_eq!(
        std::fs::read(fixture.0.join("entries.csv")).unwrap(),
        original
    );
}

#[test]
fn exactly_twenty_percent_rejected_does_not_withhold_valid_recoveries() {
    let mut rows = String::new();
    for name in ["Anna", "Ben", "Chris", "Dan"] {
        rows.push_str(&VALID.replace("Xavier", name));
    }
    rows.push_str(&VALID.replace("251.5\n", "\n"));
    let output = Fixture::new(&rows).run(true);
    assert!(output.contains("would recover 4 bodyweight(s)"), "{output}");
    assert!(output.contains("1 rejected, 0 withheld"), "{output}");
}
