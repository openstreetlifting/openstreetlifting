//! A canonical file is the whole truth about its competition. Upserting alone
//! could never express a correction that removes something, so these tests pin
//! down what a re-import deletes — and, just as importantly, what it leaves
//! alone once a row is shared with another competition.

use osl_domain::Movement;
use osl_importer::canonical::models::{AthleteData, CanonicalFormat};
use sqlx::PgPool;
use uuid::Uuid;

mod common;

use common::{attempts, import, men_80, weighing};

fn athlete(
    first_name: &str,
    last_name: &str,
    lifts: Vec<(Movement, Vec<(&str, bool)>)>,
) -> AthleteData {
    let lifter = weighing(common::athlete(first_name, last_name), "79");

    lifts.into_iter().fold(lifter, |lifter, (movement, cells)| {
        attempts(lifter, movement, &cells)
    })
}

/// One category holding whichever athletes the case needs.
fn file(slug: &str, athletes: Vec<AthleteData>) -> CanonicalFormat {
    let mut canonical = common::meet(slug, vec![men_80(athletes)]);
    canonical.movements = vec![Movement::MuscleUp, Movement::PullUp];
    canonical
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

fn two_lifters() -> Vec<AthleteData> {
    vec![
        athlete(
            "Ada",
            "Lovelace",
            vec![(Movement::MuscleUp, vec![("60", true)])],
        ),
        athlete(
            "Grace",
            "Hopper",
            vec![(Movement::MuscleUp, vec![("70", true)])],
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
        vec![(Movement::MuscleUp, vec![("60", true)])],
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
            (Movement::MuscleUp, vec![("60", true)]),
            (Movement::PullUp, vec![("40", true)]),
        ],
    )];
    import(&pool, file("test-meet", both)).await;
    assert_eq!(count(&pool, "SELECT COUNT(*) FROM lifts").await, 2);

    let one = vec![athlete(
        "Ada",
        "Lovelace",
        vec![(Movement::MuscleUp, vec![("60", true)])],
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
        vec![(
            Movement::MuscleUp,
            vec![("60", true), ("65", true), ("70", false)],
        )],
    )];
    import(&pool, file("test-meet", three)).await;
    assert_eq!(count(&pool, "SELECT COUNT(*) FROM attempts").await, 3);

    // The third attempt never happened and is corrected away.
    let two = vec![athlete(
        "Ada",
        "Lovelace",
        vec![(Movement::MuscleUp, vec![("60", true), ("65", true)])],
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
        vec![(Movement::MuscleUp, vec![("60", true)])],
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
        .retain(|movement| *movement == Movement::MuscleUp);
    import(&pool, canonical).await;

    assert_eq!(
        count(&pool, "SELECT COUNT(*) FROM competition_movements").await,
        1
    );
}

/// The same two lifters, contesting all four movements so the meet is a full
/// event and therefore actually gets a RIS.
fn four_lifts() -> Vec<(Movement, Vec<(&'static str, bool)>)> {
    vec![
        (Movement::MuscleUp, vec![("60", true)]),
        (Movement::PullUp, vec![("90", true)]),
        (Movement::Dips, vec![("100", true)]),
        (Movement::Squat, vec![("150", true)]),
    ]
}

fn two_lifters_full_event() -> Vec<AthleteData> {
    vec![
        athlete("Ada", "Lovelace", four_lifts()),
        athlete("Grace", "Hopper", four_lifts()),
    ]
}

fn full_event_file(slug: &str, athletes: Vec<AthleteData>) -> CanonicalFormat {
    let mut canonical = file(slug, athletes);
    canonical.movements = Movement::ALL.to_vec();
    canonical
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn ris_history_goes_with_the_participant(pool: PgPool) {
    import(
        &pool,
        full_event_file("test-meet", two_lifters_full_event()),
    )
    .await;
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

    // Still the full event, so only the dropped lifter changes.
    let remaining = vec![athlete("Ada", "Lovelace", four_lifts())];
    import(&pool, full_event_file("test-meet", remaining)).await;

    let orphaned: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM ris_scores_history WHERE participant_id = $1")
            .bind(dropped)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(orphaned, 0);
}
