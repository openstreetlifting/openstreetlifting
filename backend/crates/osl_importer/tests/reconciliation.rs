//! A canonical file is the whole truth about its competition. Upserting alone
//! could never express a correction that removes something, so these tests pin
//! down what a re-import deletes — and, just as importantly, what it leaves
//! alone once a row is shared with another competition.

use osl_importer::canonical::{models::CanonicalFormat, transformer::CanonicalTransformer};
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

fn attempt(number: i16, weight: i32, is_successful: bool) -> Value {
    json!({
        "attempt_number": number,
        "weight": weight,
        "is_successful": is_successful,
    })
}

fn lift(movement: &str, attempts: Vec<Value>) -> Value {
    json!({ "movement": movement, "attempts": attempts })
}

fn athlete(first_name: &str, last_name: &str, lifts: Vec<Value>) -> Value {
    json!({
        "first_name": first_name,
        "last_name": last_name,
        "country": "FR",
        "bodyweight": 79,
        "status": "competed",
        "lifts": lifts,
    })
}

/// One category holding whichever athletes the case needs.
fn file(slug: &str, athletes: Vec<Value>) -> CanonicalFormat {
    let document = json!({
        "format_version": "1.4.0",
        "source": {
            "type": "manual",
            "extracted_at": "2026-01-01T00:00:00Z",
            "extractor": "reconciliation-test",
        },
        "competition": {
            "name": slug,
            "slug": slug,
            "federation": { "name": "Test Federation" },
            "start_date": "2026-01-01",
            "end_date": "2026-01-01",
            "country": "FR",
        },
        "movements": [
            { "name": "Muscle-up", "order": 1 },
            { "name": "Pull-up", "order": 2 },
        ],
        "categories": [{
            "name": "Men -80kg",
            "gender": "M",
            "weight_class_slug": "M-80",
            "athletes": athletes,
        }],
    });

    serde_json::from_value(document).expect("test document should be a valid canonical file")
}

async fn import(pool: &PgPool, canonical: CanonicalFormat) {
    CanonicalTransformer::new(pool)
        .import_to_database(canonical)
        .await
        .expect("import should succeed");
}

async fn count(pool: &PgPool, query: &'static str) -> i64 {
    sqlx::query_scalar(query).fetch_one(pool).await.unwrap()
}

async fn participant_count(pool: &PgPool) -> i64 {
    count(pool, "SELECT COUNT(*) FROM competition_participants").await
}

async fn athlete_exists(pool: &PgPool, last_name: &str) -> bool {
    let found: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM athletes WHERE last_name = $1")
        .bind(last_name)
        .fetch_one(pool)
        .await
        .unwrap();
    found > 0
}

fn two_lifters() -> Vec<Value> {
    vec![
        athlete(
            "Ada",
            "Lovelace",
            vec![lift("Muscle-up", vec![attempt(1, 60, true)])],
        ),
        athlete(
            "Grace",
            "Hopper",
            vec![lift("Muscle-up", vec![attempt(1, 70, true)])],
        ),
    ]
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_reimport_of_the_same_file_changes_nothing(pool: PgPool) {
    import(&pool, file("test-meet", two_lifters())).await;
    let before = participant_count(&pool).await;

    import(&pool, file("test-meet", two_lifters())).await;

    assert_eq!(participant_count(&pool).await, before);
    assert_eq!(count(&pool, "SELECT COUNT(*) FROM lifts").await, 2);
    assert_eq!(count(&pool, "SELECT COUNT(*) FROM attempts").await, 2);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn an_athlete_dropped_from_the_file_loses_their_participation(pool: PgPool) {
    import(&pool, file("test-meet", two_lifters())).await;
    assert_eq!(participant_count(&pool).await, 2);

    let remaining = vec![athlete(
        "Ada",
        "Lovelace",
        vec![lift("Muscle-up", vec![attempt(1, 60, true)])],
    )];
    import(&pool, file("test-meet", remaining)).await;

    assert_eq!(participant_count(&pool).await, 1);
    // The lift and attempt went with the participant, through the cascade.
    assert_eq!(count(&pool, "SELECT COUNT(*) FROM lifts").await, 1);
    assert_eq!(count(&pool, "SELECT COUNT(*) FROM attempts").await, 1);

    // The athlete is shared reference data and outlives the participation.
    assert!(athlete_exists(&pool, "Hopper").await);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_lift_dropped_from_the_file_is_removed(pool: PgPool) {
    let both = vec![athlete(
        "Ada",
        "Lovelace",
        vec![
            lift("Muscle-up", vec![attempt(1, 60, true)]),
            lift("Pull-up", vec![attempt(1, 40, true)]),
        ],
    )];
    import(&pool, file("test-meet", both)).await;
    assert_eq!(count(&pool, "SELECT COUNT(*) FROM lifts").await, 2);

    let one = vec![athlete(
        "Ada",
        "Lovelace",
        vec![lift("Muscle-up", vec![attempt(1, 60, true)])],
    )];
    import(&pool, file("test-meet", one)).await;

    assert_eq!(count(&pool, "SELECT COUNT(*) FROM lifts").await, 1);
    assert_eq!(participant_count(&pool).await, 1);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn an_attempt_dropped_from_the_file_is_removed(pool: PgPool) {
    let three = vec![athlete(
        "Ada",
        "Lovelace",
        vec![lift(
            "Muscle-up",
            vec![
                attempt(1, 60, true),
                attempt(2, 65, true),
                attempt(3, 70, false),
            ],
        )],
    )];
    import(&pool, file("test-meet", three)).await;
    assert_eq!(count(&pool, "SELECT COUNT(*) FROM attempts").await, 3);

    // The third attempt never happened and is corrected away.
    let two = vec![athlete(
        "Ada",
        "Lovelace",
        vec![lift(
            "Muscle-up",
            vec![attempt(1, 60, true), attempt(2, 65, true)],
        )],
    )];
    import(&pool, file("test-meet", two)).await;

    assert_eq!(count(&pool, "SELECT COUNT(*) FROM attempts").await, 2);
    assert_eq!(count(&pool, "SELECT COUNT(*) FROM lifts").await, 1);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn pruning_one_competition_leaves_the_others_alone(pool: PgPool) {
    import(&pool, file("first-meet", two_lifters())).await;
    import(&pool, file("second-meet", two_lifters())).await;
    assert_eq!(participant_count(&pool).await, 4);

    // Grace is dropped from the first meet only.
    let remaining = vec![athlete(
        "Ada",
        "Lovelace",
        vec![lift("Muscle-up", vec![attempt(1, 60, true)])],
    )];
    import(&pool, file("first-meet", remaining)).await;

    assert_eq!(participant_count(&pool).await, 3);

    let second_meet: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM competition_participants cp
         JOIN competitions c ON c.competition_id = cp.competition_id
         WHERE c.slug = 'second-meet'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(second_meet, 2, "the other meet keeps both of its lifters");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_movement_dropped_from_the_file_is_removed(pool: PgPool) {
    import(&pool, file("test-meet", two_lifters())).await;
    assert_eq!(
        count(&pool, "SELECT COUNT(*) FROM competition_movements").await,
        2
    );

    let mut canonical = file("test-meet", two_lifters());
    canonical
        .movements
        .retain(|movement| movement.name == "Muscle-up");
    import(&pool, canonical).await;

    assert_eq!(
        count(&pool, "SELECT COUNT(*) FROM competition_movements").await,
        1
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn ris_history_goes_with_the_participant(pool: PgPool) {
    import(&pool, file("test-meet", two_lifters())).await;
    let scored: i64 = count(&pool, "SELECT COUNT(*) FROM ris_scores_history").await;
    assert_eq!(scored, 2, "both lifters should have been scored");

    let dropped: Uuid = sqlx::query_scalar(
        "SELECT cp.participant_id FROM competition_participants cp
         JOIN athletes a ON a.athlete_id = cp.athlete_id
         WHERE a.last_name = 'Hopper'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let remaining = vec![athlete(
        "Ada",
        "Lovelace",
        vec![lift("Muscle-up", vec![attempt(1, 60, true)])],
    )];
    import(&pool, file("test-meet", remaining)).await;

    let orphaned: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM ris_scores_history WHERE participant_id = $1")
            .bind(dropped)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(orphaned, 0);
}
