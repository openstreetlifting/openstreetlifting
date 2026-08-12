//! Deleting a canonical file has to delete its competition, since the files are
//! the data. What it must not take with it is the reference data and the
//! athletes who lifted somewhere else too.

use osl_importer::canonical::{models::CanonicalFormat, transformer::CanonicalTransformer};
use osl_importer::sync::CompetitionSync;
use serde_json::{Value, json};
use sqlx::PgPool;

fn lift(movement: &str, weight: i32) -> Value {
    json!({
        "movement": movement,
        "attempts": [{ "attempt_number": 1, "weight": weight, "is_successful": true }],
    })
}

fn athlete(first: &str, last: &str) -> Value {
    json!({
        "first_name": first, "last_name": last, "country": "FR",
        "bodyweight": 80, "status": "competed",
        "lifts": [lift("Muscle-up", 40), lift("Pull-up", 90)],
    })
}

fn meet(slug: &str, athletes: Vec<Value>) -> CanonicalFormat {
    serde_json::from_value(json!({
        "format_version": "1.5.0",
        "source": { "type": "manual", "extracted_at": "2026-01-01T00:00:00Z", "extractor": "file-deletion-test" },
        "competition": {
            "name": slug, "slug": slug,
            "federation": { "name": "Test Federation" },
            "start_date": "2026-01-01", "end_date": "2026-01-01", "country": "FR",
        },
        "movements": [
            { "name": "Muscle-up", "order": 1 }, { "name": "Pull-up", "order": 2 },
        ],
        "categories": [{
            "name": "Men -80kg", "gender": "M", "weight_class_slug": "M-80",
            "athletes": athletes,
        }],
    }))
    .expect("test document should be a valid canonical file")
}

async fn import(pool: &PgPool, canonical: CanonicalFormat) {
    CanonicalTransformer::new(pool)
        .import_to_database(canonical)
        .await
        .expect("import should succeed");
}

/// One meet whose file is still there, one whose file was deleted, and a lifter
/// who was at both.
async fn two_meets(pool: &PgPool) {
    import(
        pool,
        meet(
            "kept-meet",
            vec![athlete("Clara", "Clean"), athlete("Sam", "Shared")],
        ),
    )
    .await;
    import(
        pool,
        meet(
            "gone-meet",
            vec![athlete("Sam", "Shared"), athlete("Gil", "Gone")],
        ),
    )
    .await;
}

async fn count(pool: &PgPool, query: &'static str) -> i64 {
    sqlx::query_scalar(query).fetch_one(pool).await.unwrap()
}

async fn slugs(pool: &PgPool) -> Vec<String> {
    sqlx::query_scalar("SELECT slug FROM competitions ORDER BY slug")
        .fetch_all(pool)
        .await
        .unwrap()
}

async fn last_names(pool: &PgPool) -> Vec<String> {
    sqlx::query_scalar("SELECT last_name FROM athletes ORDER BY last_name")
        .fetch_all(pool)
        .await
        .unwrap()
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_competition_no_file_claims_is_deleted(pool: PgPool) {
    two_meets(&pool).await;

    let plan = CompetitionSync::new(&pool)
        .apply(&["kept-meet".to_string()])
        .await
        .unwrap();

    assert_eq!(slugs(&pool).await, vec!["kept-meet"]);
    assert_eq!(
        plan.competitions
            .iter()
            .map(|c| c.slug.as_str())
            .collect::<Vec<_>>(),
        vec!["gone-meet"],
        "the plan has to name what it deleted"
    );

    assert_eq!(
        count(&pool, "SELECT COUNT(*) FROM competition_participants").await,
        2
    );
    assert_eq!(count(&pool, "SELECT COUNT(*) FROM lifts").await, 4);
    assert_eq!(
        count(&pool, "SELECT COUNT(*) FROM attempts").await,
        4,
        "the attempts of a deleted competition go with it"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn an_athlete_left_with_no_result_goes_too(pool: PgPool) {
    two_meets(&pool).await;

    CompetitionSync::new(&pool)
        .apply(&["kept-meet".to_string()])
        .await
        .unwrap();

    assert_eq!(
        last_names(&pool).await,
        vec!["Clean", "Shared"],
        "Gone only ever lifted at the deleted meet, Shared was at both"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn reference_data_is_left_alone(pool: PgPool) {
    two_meets(&pool).await;

    let movements = count(&pool, "SELECT COUNT(*) FROM movements").await;
    let weight_classes = count(&pool, "SELECT COUNT(*) FROM weight_classes").await;

    CompetitionSync::new(&pool)
        .apply(&["kept-meet".to_string()])
        .await
        .unwrap();

    assert_eq!(count(&pool, "SELECT COUNT(*) FROM federations").await, 1);
    assert_eq!(count(&pool, "SELECT COUNT(*) FROM categories").await, 1);
    assert_eq!(
        count(&pool, "SELECT COUNT(*) FROM movements").await,
        movements
    );
    assert_eq!(
        count(&pool, "SELECT COUNT(*) FROM weight_classes").await,
        weight_classes
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn claiming_nothing_is_refused(pool: PgPool) {
    two_meets(&pool).await;

    let refused = CompetitionSync::new(&pool).apply(&[]).await;

    assert!(
        refused.is_err(),
        "an empty tree means a mistake, not an instruction to delete everything"
    );
    assert_eq!(slugs(&pool).await, vec!["gone-meet", "kept-meet"]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_dry_run_reports_without_deleting(pool: PgPool) {
    two_meets(&pool).await;

    let plan = CompetitionSync::new(&pool)
        .dry_run(&["kept-meet".to_string()])
        .await
        .unwrap();

    assert_eq!(
        plan.competitions
            .iter()
            .map(|c| c.slug.as_str())
            .collect::<Vec<_>>(),
        vec!["gone-meet"]
    );
    assert_eq!(plan.athletes, vec!["Gil Gone"]);
    assert_eq!(
        slugs(&pool).await,
        vec!["gone-meet", "kept-meet"],
        "reporting must not be a deletion"
    );
}
