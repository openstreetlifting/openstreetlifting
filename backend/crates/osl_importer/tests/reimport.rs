//! A canonical file is built up over several passes, so importing a corrected
//! file has to land the correction rather than stop at the row already there.

use chrono::NaiveDate;
use osl_domain::{CompetitionStatus, Gender, Movement, WeightClassSlug};
use osl_importer::canonical::models::{CanonicalFormat, CategoryData};
use osl_importer::canonical::transformer::CanonicalTransformer;
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::str::FromStr;

mod common;

use common::{attempts, decimal};

enum Class {
    Slug(WeightClassSlug),
    Open(&'static str),
}

fn canonical(class: Class) -> CanonicalFormat {
    fixture(class, None)
}

fn fixture(class: Class, ris: Option<&str>) -> CanonicalFormat {
    let mut lifter = common::athlete("John", "Doe");
    lifter.bodyweight = Some(decimal("72.5"));
    lifter.total = Some(decimal("310"));

    if let Some(ris) = ris {
        lifter.bodyweight = None;
        lifter.reported_ris = Some(decimal(ris));
    }

    let lifter = attempts(lifter, Movement::MuscleUp, &[("50", true)]);
    let lifter = attempts(lifter, Movement::PullUp, &[("60", true)]);
    let lifter = attempts(lifter, Movement::Dips, &[("80", true)]);
    let lifter = attempts(lifter, Movement::Squat, &[("120", true)]);

    let (weight_class_slug, weight_class_min) = match class {
        Class::Slug(slug) => (Some(slug), None),
        Class::Open(min) => (None, Some(decimal(min))),
    };

    let category = CategoryData {
        division: None,
        gender: Gender::M,
        weight_class_slug,
        weight_class_min,
        weight_class_max: None,
        athletes: vec![lifter],
    };

    let mut canonical = common::competition("test-open", vec![category]);
    canonical.competition.name = "Test Open".to_string();
    canonical.competition.status = None;
    canonical
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn defaults_missing_status_to_completed(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(canonical(Class::Slug(WeightClassSlug::M73)))
        .await
        .unwrap();

    let status: String =
        sqlx::query_scalar!(r#"SELECT status FROM competitions WHERE slug = 'test-open'"#)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(status, "completed");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn preserves_status_when_source_omits_it(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    let mut cancelled = canonical(Class::Slug(WeightClassSlug::M73));
    cancelled.competition.status = Some(CompetitionStatus::Cancelled);
    transformer.import_to_database(cancelled).await.unwrap();

    transformer
        .import_to_database(canonical(Class::Slug(WeightClassSlug::M73)))
        .await
        .unwrap();

    let status: String =
        sqlx::query_scalar!(r#"SELECT status FROM competitions WHERE slug = 'test-open'"#)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(status, "cancelled");
}

/// The class is part of what a result is, so correcting it refiles the lifter
/// rather than editing the class they were in.
#[sqlx::test(migrations = "../osl_db/migrations")]
async fn updates_corrected_weight_classes(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(canonical(Class::Slug(WeightClassSlug::M73)))
        .await
        .unwrap();
    transformer
        .import_to_database(canonical(Class::Slug(WeightClassSlug::M80)))
        .await
        .unwrap();

    let classes = participant_classes(&pool).await;
    assert_eq!(
        classes,
        vec![(Some(Decimal::from(73)), Some(Decimal::from(80)))]
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn stores_open_class_lower_bounds(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(canonical(Class::Open("87")))
        .await
        .unwrap();

    let classes = participant_classes(&pool).await;
    assert_eq!(classes, vec![(Some(Decimal::from(87)), None)]);
}

/// A competition running two divisions in one class is two contests, so each keeps its
/// own winner instead of the second collapsing onto the first.
#[sqlx::test(migrations = "../osl_db/migrations")]
async fn ranks_divisions_separately(pool: PgPool) {
    let mut elite = fixture(Class::Slug(WeightClassSlug::M73), None);
    elite.categories[0].division = Some("Elite".to_string());

    let mut open = elite.categories[0].clone();
    open.division = Some("Open".to_string());
    open.athletes[0].first_name = "Jane".to_string();
    elite.categories.push(open);

    CanonicalTransformer::new(&pool)
        .import_to_database(elite)
        .await
        .unwrap();

    let divisions = sqlx::query_scalar!(
        r#"SELECT d.name FROM competition_participants cp
           JOIN divisions d ON d.division_id = cp.division_id
           ORDER BY d.name"#
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(divisions, vec!["Elite", "Open"]);

    let detail = osl_db::repository::competition::CompetitionRepository::new(&pool)
        .find_by_slug_detailed("test-open")
        .await
        .unwrap();

    assert_eq!(detail.categories.len(), 2);
    for category in &detail.categories {
        assert_eq!(category.participants[0].rank, Some(1));
    }
}

async fn participant_classes(pool: &PgPool) -> Vec<(Option<Decimal>, Option<Decimal>)> {
    sqlx::query!(
        r#"SELECT wc.min_kg, wc.max_kg
           FROM competition_participants cp
           JOIN weight_classes wc USING (weight_class_id)"#
    )
    .fetch_all(pool)
    .await
    .unwrap()
    .into_iter()
    .map(|row| (row.min_kg, row.max_kg))
    .collect()
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn preserves_reported_scores_and_provenance(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(fixture(Class::Slug(WeightClassSlug::M73), Some("84.21")))
        .await
        .unwrap();

    let row =
        sqlx::query!(r#"SELECT ris_score, ris_source, bodyweight FROM competition_participants"#)
            .fetch_one(&pool)
            .await
            .unwrap();

    assert_eq!(row.ris_score, Some(Decimal::from_str("84.21").unwrap()));
    assert_eq!(row.ris_source.as_deref(), Some("reported"));
    assert_eq!(row.bodyweight, None);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn marks_computed_scores(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(canonical(Class::Slug(WeightClassSlug::M73)))
        .await
        .unwrap();

    let row = sqlx::query!(r#"SELECT ris_score, ris_source FROM competition_participants"#)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert!(row.ris_score.is_some());
    assert_eq!(row.ris_source.as_deref(), Some("computed"));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn preserves_reported_scores_when_recomputing(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    let file = fixture(Class::Slug(WeightClassSlug::M73), Some("84.21"));
    transformer.import_to_database(file.clone()).await.unwrap();
    transformer.import_to_database(file).await.unwrap();

    let row = sqlx::query!(r#"SELECT ris_score, ris_source FROM competition_participants"#)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(row.ris_score, Some(Decimal::from_str("84.21").unwrap()));
    assert_eq!(row.ris_source.as_deref(), Some("reported"));
}

/// Correcting a federation name or a date in the file has to reach the database,
/// or the file stops being the truth about its own competition the moment one exists.
#[sqlx::test(migrations = "../osl_db/migrations")]
async fn updates_corrected_competition_metadata(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(canonical(Class::Slug(WeightClassSlug::M73)))
        .await
        .unwrap();

    let mut corrected = canonical(Class::Slug(WeightClassSlug::M73));
    corrected.competition.federation.name = "Corrected Federation".to_string();
    corrected.competition.venue = Some("Main Hall".into());
    corrected.competition.start_date = NaiveDate::from_ymd_opt(2026, 3, 7).unwrap();
    corrected.competition.end_date = NaiveDate::from_ymd_opt(2026, 3, 8).unwrap();
    transformer.import_to_database(corrected).await.unwrap();

    let row = sqlx::query!(
        r#"
        SELECT f.name as "federation!", c.start_date as "start_date!", c.end_date as "end_date!", c.venue
        FROM competitions c JOIN federations f USING (federation_id)
        WHERE c.slug = 'test-open'
        "#
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(row.federation, "Corrected Federation");
    assert_eq!(row.venue.as_deref(), Some("Main Hall"));
    assert_eq!(row.start_date, NaiveDate::from_ymd_opt(2026, 3, 7).unwrap());
    assert_eq!(row.end_date, NaiveDate::from_ymd_opt(2026, 3, 8).unwrap());
}
