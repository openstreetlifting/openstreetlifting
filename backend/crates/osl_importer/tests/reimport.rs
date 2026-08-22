//! A canonical file is built up over several passes, so importing a corrected
//! file has to land the correction rather than stop at the row already there.

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

    if let Some(ris) = ris {
        lifter.bodyweight = None;
        lifter.ris = Some(decimal(ris));
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
        name: "Test category".to_string(),
        gender: Gender::M,
        weight_class_slug,
        weight_class_min,
        weight_class_max: None,
        athletes: vec![lifter],
    };

    let mut canonical = common::meet("test-open", vec![category]);
    canonical.competition.name = "Test Open".to_string();
    canonical.competition.status = None;
    canonical
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_file_without_a_status_imports_as_completed(pool: PgPool) {
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
async fn reimport_keeps_a_status_the_file_does_not_state(pool: PgPool) {
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

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn reimport_corrects_category_bounds(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(canonical(Class::Slug(WeightClassSlug::M73)))
        .await
        .unwrap();
    transformer
        .import_to_database(canonical(Class::Slug(WeightClassSlug::M80)))
        .await
        .unwrap();

    let category = sqlx::query!(
        r#"SELECT wc.min_kg AS weight_class_min, wc.max_kg AS weight_class_max
           FROM categories c
           JOIN weight_classes wc USING (weight_class_id)
           WHERE c.name = 'Test category'"#
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(category.weight_class_min, Some(Decimal::from(73)));
    assert_eq!(category.weight_class_max, Some(Decimal::from(80)));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn open_class_is_stored_as_a_lower_bound(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(canonical(Class::Open("87")))
        .await
        .unwrap();

    let category = sqlx::query!(
        r#"SELECT wc.min_kg AS weight_class_min, wc.max_kg AS weight_class_max
           FROM categories c
           JOIN weight_classes wc USING (weight_class_id)
           WHERE c.name = 'Test category'"#
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(category.weight_class_min, Some(Decimal::from(87)));
    assert_eq!(category.weight_class_max, None);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_reported_score_is_kept_and_marked(pool: PgPool) {
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
async fn a_computed_score_is_marked_as_ours(pool: PgPool) {
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
async fn a_recompute_leaves_a_reported_score_alone(pool: PgPool) {
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
