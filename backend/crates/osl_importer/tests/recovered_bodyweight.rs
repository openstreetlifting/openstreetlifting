use osl_db::params::{RankingFilter, RankingMovement, SortDirection};
use osl_db::repository::ranking::RankingRepository;
use osl_db::services::ris_computation::recompute_all_ris;
use osl_domain::{Edition, Gender, RisSource, WeightClassSlug};
use osl_importer::canonical::models::{BodyweightSource, CanonicalFormat};
use osl_importer::canonical::{
    store, transformer::CanonicalTransformer, validator::CanonicalValidator,
};
use rust_decimal::Decimal;
use sqlx::PgPool;

mod common;

fn recovered() -> CanonicalFormat {
    let mut athlete = common::lifting(
        common::athlete("Xavier", "Macias"),
        ["50", "110", "175", "251.5"],
    );
    athlete.bodyweight = Some(common::decimal("91.7"));
    athlete.bodyweight_source = Some(BodyweightSource::Recovered);
    athlete.ris = Some(common::decimal("113.43"));
    athlete.reported_ris_edition = Some(Edition::V2024);
    common::competition(
        "recovered-worlds",
        vec![common::category(WeightClassSlug::M94, vec![athlete])],
    )
}

#[test]
fn source_evidence_is_required_and_checked() {
    let valid = recovered();
    CanonicalValidator::validate(&valid).unwrap();
    let mut missing_edition = valid.clone();
    missing_edition.categories[0].athletes[0].reported_ris_edition = None;
    assert!(
        CanonicalValidator::validate(&missing_edition)
            .unwrap_err()
            .to_string()
            .contains("ReportedRisEdition")
    );
    let mut missing_score = valid.clone();
    missing_score.categories[0].athletes[0].ris = None;
    assert!(CanonicalValidator::validate(&missing_score).is_err());
    let mut altered_weight = valid.clone();
    altered_weight.categories[0].athletes[0].bodyweight = Some(Decimal::from(90));
    assert!(
        CanonicalValidator::validate(&altered_weight)
            .unwrap_err()
            .to_string()
            .contains("reproduces RIS")
    );
    let mut reported_weight = valid;
    reported_weight.categories[0].athletes[0].bodyweight_source = Some(BodyweightSource::Reported);
    assert!(CanonicalValidator::validate(&reported_weight).is_err());
}

#[test]
fn total_requires_each_movement_and_consistent_attempts_but_accepts_zero() {
    let mut athlete = recovered().categories.remove(0).athletes.remove(0);
    assert_eq!(athlete.complete_total().unwrap(), common::decimal("586.5"));
    let squat = athlete.lifts.pop().unwrap();
    assert!(athlete.complete_total().unwrap_err().contains("Squat"));
    athlete.lifts.push(squat);
    athlete.lifts[0].attempts.as_mut().unwrap()[0].is_successful = false;
    assert!(
        athlete
            .complete_total()
            .unwrap_err()
            .contains("no successful")
    );
    athlete.lifts[0].attempts.as_mut().unwrap()[0].is_successful = true;
    athlete.lifts[0].best_lift = Some(Decimal::from(60));
    assert!(
        athlete
            .complete_total()
            .unwrap_err()
            .contains("contradicts")
    );
    athlete.lifts[0].best_lift = None;
    athlete.lifts[0].attempts.as_mut().unwrap()[0].weight = Decimal::ZERO;
    assert_eq!(athlete.complete_total().unwrap(), common::decimal("536.5"));
    athlete.lifts.push(athlete.lifts[0].clone());
    assert!(athlete.complete_total().unwrap_err().contains("duplicate"));
}

#[test]
fn csv_round_trip_keeps_recovery_evidence_and_rejects_a_contradictory_best() {
    let directory = std::env::temp_dir().join(format!("osl-recovery-csv-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&directory).unwrap();
    store::write(&directory, &recovered()).unwrap();
    let read = store::read(&directory).unwrap();
    let athlete = &read.categories[0].athletes[0];
    assert_eq!(athlete.ris, Some(common::decimal("113.43")));
    assert_eq!(athlete.reported_ris_edition, Some(Edition::V2024));
    assert_eq!(
        athlete.bodyweight_source(),
        Some(BodyweightSource::Recovered)
    );
    CanonicalValidator::validate(&read).unwrap();

    let path = directory.join("entries.csv");
    let csv = std::fs::read_to_string(&path).unwrap();
    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let headers = reader.headers().unwrap().clone();
    let best = headers
        .iter()
        .position(|name| name == "BestSquatKg")
        .unwrap();
    let mut fields = reader
        .records()
        .next()
        .unwrap()
        .unwrap()
        .iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    fields[best] = "260".into();
    let mut writer = csv::Writer::from_path(&path).unwrap();
    writer.write_record(&headers).unwrap();
    writer.write_record(fields).unwrap();
    writer.flush().unwrap();
    let error = store::read(&directory).unwrap_err().to_string();
    assert!(error.contains("contradicts successful attempts"), "{error}");
    std::fs::remove_dir_all(directory).unwrap();
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn recovered_2024_score_is_ranked_using_2026_and_source_survives_recompute(pool: PgPool) {
    let canonical = recovered();
    let mut reported = canonical.clone();
    reported.categories[0].athletes[0].bodyweight = None;
    reported.categories[0].athletes[0].bodyweight_source = None;
    let importer = CanonicalTransformer::new(&pool);
    importer.import_to_database(reported).await.unwrap();
    importer
        .import_to_database(canonical.clone())
        .await
        .unwrap();
    // Re-importing a computed row must reset the active score and its edition together.
    importer.import_to_database(canonical).await.unwrap();

    let expected = osl_domain::ris::compute(
        common::decimal("91.7"),
        common::decimal("586.5"),
        Gender::M,
        Edition::V2026,
    );
    assert_ne!(expected, common::decimal("113.43"));
    assert_eq!(recompute_all_ris(&pool).await.unwrap(), 1);

    let row: (Decimal, String, i32, Decimal, i32, String) = sqlx::query_as(
        "SELECT ris_score, ris_source, ris_edition, reported_ris_score, reported_ris_edition, bodyweight_source FROM competition_participants"
    ).fetch_one(&pool).await.unwrap();
    assert_eq!(
        row,
        (
            expected,
            "computed".into(),
            2026,
            common::decimal("113.43"),
            2024,
            "recovered".into()
        )
    );

    let filter = RankingFilter {
        gender: None,
        country: None,
        federation: None,
        name: None,
        movement: RankingMovement::Ris,
        direction: SortDirection::Desc,
        event: osl_domain::FULL_EVENT.into(),
        category: None,
        year: None,
        competition_id: None,
        offset: 0,
        limit: 10,
    };
    let (ranking, total) = RankingRepository::new(&pool)
        .get_global_ranking(&filter)
        .await
        .unwrap();
    assert_eq!(total, 1);
    assert_eq!(ranking[0].ris_score, Some(expected));
    assert_eq!(ranking[0].ris_source, Some(RisSource::Computed));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn correcting_recovery_back_to_source_only_does_not_leave_stale_provenance(pool: PgPool) {
    let mut canonical = recovered();
    let importer = CanonicalTransformer::new(&pool);
    importer
        .import_to_database(canonical.clone())
        .await
        .unwrap();
    let athlete = &mut canonical.categories[0].athletes[0];
    athlete.bodyweight = None;
    athlete.bodyweight_source = None;
    importer.import_to_database(canonical).await.unwrap();
    assert_eq!(recompute_all_ris(&pool).await.unwrap(), 0);
    let row: (Option<Decimal>, Option<String>, Decimal, String, Option<i32>, Decimal, i32) = sqlx::query_as(
        "SELECT bodyweight, bodyweight_source, ris_score, ris_source, ris_edition, reported_ris_score, reported_ris_edition FROM competition_participants"
    ).fetch_one(&pool).await.unwrap();
    assert_eq!(
        row,
        (
            None,
            None,
            common::decimal("113.43"),
            "reported".into(),
            None,
            common::decimal("113.43"),
            2024
        )
    );
}
