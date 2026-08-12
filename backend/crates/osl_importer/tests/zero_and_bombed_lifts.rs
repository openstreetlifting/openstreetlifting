//! Two results that look alike in a total and are nothing alike.
//!
//! A muscle-up with no added weight is a successful lift worth 0. A movement
//! where every attempt failed is not a lift at all. Both contribute nothing to
//! the total, so only the stored value tells them apart: 0 against NULL.

use osl_db::params::{RankingFilter, RankingMovement};
use osl_db::repository::ranking::RankingRepository;
use osl_importer::canonical::{models::CanonicalFormat, transformer::CanonicalTransformer};
use rust_decimal::Decimal;
use serde_json::{Value, json};
use sqlx::PgPool;

fn lift(movement: &str, attempts: Vec<(&str, bool)>) -> Value {
    json!({
        "movement": movement,
        "attempts": attempts.iter().enumerate().map(|(i, (w, ok))| json!({
            "attempt_number": i as i16 + 1, "weight": w, "is_successful": ok
        })).collect::<Vec<_>>(),
    })
}

fn athlete(first: &str, last: &str, lifts: Vec<Value>) -> Value {
    json!({
        "first_name": first, "last_name": last, "country": "FR",
        "bodyweight": 80, "status": "competed", "lifts": lifts,
    })
}

fn meet(slug: &str, athletes: Vec<Value>) -> CanonicalFormat {
    serde_json::from_value(json!({
        "format_version": "1.5.0",
        "source": { "type": "manual", "extracted_at": "2026-01-01T00:00:00Z", "extractor": "zero-lift-test" },
        "competition": {
            "name": slug, "slug": slug,
            "federation": { "name": "Test Federation" },
            "start_date": "2026-01-01", "end_date": "2026-01-01", "country": "FR",
        },
        "movements": [
            { "name": "Muscle-up", "order": 1 }, { "name": "Pull-up", "order": 2 },
            { "name": "Dips", "order": 3 }, { "name": "Squat", "order": 4 },
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

async fn best(pool: &PgPool, last_name: &str, movement: &str) -> Option<Decimal> {
    sqlx::query_scalar(
        "SELECT l.max_weight FROM lifts l
         JOIN competition_participants cp USING (participant_id)
         JOIN athletes a USING (athlete_id)
         WHERE a.last_name = $1 AND l.movement_name = $2",
    )
    .bind(last_name)
    .bind(movement)
    .fetch_one(pool)
    .await
    .unwrap()
}

/// Lifted bodyweight and nothing more, then missed twice going up.
fn bodyweight_lifter() -> Value {
    athlete(
        "Swann",
        "Bodyweight",
        vec![
            lift(
                "Muscle-up",
                vec![("0", true), ("2.5", false), ("2.5", false)],
            ),
            lift("Pull-up", vec![("30", true)]),
            lift("Dips", vec![("70", true)]),
            lift("Squat", vec![("150", true)]),
        ],
    )
}

/// Made two movements, missed every attempt in the other two.
fn bomber() -> Value {
    athlete(
        "Tom",
        "Bombed",
        vec![
            lift("Muscle-up", vec![("20", true), ("25", true), ("30", false)]),
            lift("Pull-up", vec![("85", false), ("90", false), ("90", true)]),
            lift("Dips", vec![("160", false), ("170", false), ("170", false)]),
            lift(
                "Squat",
                vec![("260", false), ("260", false), ("260", false)],
            ),
        ],
    )
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_bodyweight_lift_is_stored_as_zero(pool: PgPool) {
    import(&pool, meet("test-meet", vec![bodyweight_lifter()])).await;

    assert_eq!(
        best(&pool, "Bodyweight", "Muscle-up").await,
        Some(Decimal::ZERO),
        "she lifted her bodyweight, which is a result rather than an absence"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_bombed_movement_is_stored_as_absent(pool: PgPool) {
    import(&pool, meet("test-meet", vec![bomber()])).await;

    assert_eq!(
        best(&pool, "Bombed", "Dips").await,
        None,
        "he lifted nothing, which must not read as having lifted zero"
    );
    assert_eq!(
        best(&pool, "Bombed", "Pull-up").await,
        Some(Decimal::from(90))
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn every_attempt_is_kept_including_the_failures(pool: PgPool) {
    import(&pool, meet("test-meet", vec![bomber()])).await;

    let attempts: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM attempts a
         JOIN lifts l USING (lift_id)
         WHERE l.movement_name = 'Squat'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(attempts, 3, "three missed squats are still three squats");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_bombed_movement_costs_only_itself(pool: PgPool) {
    import(&pool, meet("test-meet", vec![bomber()])).await;

    let total: Option<Decimal> = sqlx::query_scalar(
        "SELECT SUM(l.max_weight) FROM lifts l
         JOIN competition_participants cp USING (participant_id)
         JOIN athletes a USING (athlete_id)
         WHERE a.last_name = 'Bombed'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    // 25 from the muscle-up plus 90 from the pull-up. The two bombs add nothing
    // and take nothing away.
    assert_eq!(total, Some(Decimal::from(115)));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_bodyweight_lifter_still_appears_on_that_movement(pool: PgPool) {
    import(
        &pool,
        meet("test-meet", vec![bodyweight_lifter(), bomber()]),
    )
    .await;

    let filter = RankingFilter {
        gender: None,
        country: None,
        movement: RankingMovement::Muscleup,
        event: osl_domain::FULL_EVENT.to_string(),
        offset: 0,
        limit: 50,
    };
    let (rows, total) = RankingRepository::new(&pool)
        .get_global_ranking(&filter)
        .await
        .unwrap();

    let names: Vec<&str> = rows.iter().map(|r| r.last_name.as_str()).collect();
    assert_eq!(names, vec!["Bombed", "Bodyweight"], "25 kg beats 0 kg");
    assert_eq!(total, 2);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_bomber_is_left_off_that_movements_board(pool: PgPool) {
    import(
        &pool,
        meet("test-meet", vec![bodyweight_lifter(), bomber()]),
    )
    .await;

    let filter = RankingFilter {
        gender: None,
        country: None,
        movement: RankingMovement::Dips,
        event: osl_domain::FULL_EVENT.to_string(),
        offset: 0,
        limit: 50,
    };
    let (rows, total) = RankingRepository::new(&pool)
        .get_global_ranking(&filter)
        .await
        .unwrap();

    let names: Vec<&str> = rows.iter().map(|r| r.last_name.as_str()).collect();
    assert_eq!(
        names,
        vec!["Bodyweight"],
        "no successful dip means no place on the dip board"
    );
    assert_eq!(total, 1, "the count has to agree with the rows");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_negative_weight_is_still_refused(pool: PgPool) {
    import(&pool, meet("test-meet", vec![bodyweight_lifter()])).await;

    let inserted = sqlx::query(
        "INSERT INTO attempts (lift_id, attempt_number, weight, is_successful)
         SELECT lift_id, 3, -5, false FROM lifts LIMIT 1",
    )
    .execute(&pool)
    .await;

    assert!(inserted.is_err(), "zero is a weight, below zero is not");
}
