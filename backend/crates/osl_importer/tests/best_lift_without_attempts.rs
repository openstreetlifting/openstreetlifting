//! Some sources publish only the best lift per movement, never the attempts
//! behind it. The transformer must store that weight without inventing an
//! attempt, and totals must still add up.

mod common;

use common::{athlete, best, competition, import, men_80};
use osl_domain::Movement;
use osl_importer::canonical::models::AthleteData;
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::str::FromStr;

async fn best_weight(pool: &PgPool, last_name: &str, movement: &str) -> Option<Decimal> {
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

fn stated_best_lifter() -> AthleteData {
    let lifter = athlete("Result", "Card");
    let lifter = best(lifter, Movement::MuscleUp, "47.5");
    let lifter = best(lifter, Movement::PullUp, "90");
    let lifter = best(lifter, Movement::Dips, "130");
    best(lifter, Movement::Squat, "200")
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_stated_best_lift_is_stored_as_max_weight(pool: PgPool) {
    import(
        &pool,
        competition("test-competition", vec![men_80(vec![stated_best_lifter()])]),
    )
    .await;

    assert_eq!(
        best_weight(&pool, "Card", "Squat").await,
        Some(Decimal::from(200))
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_stated_best_lift_creates_no_attempt_rows(pool: PgPool) {
    import(
        &pool,
        competition("test-competition", vec![men_80(vec![stated_best_lifter()])]),
    )
    .await;

    let attempts: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM attempts a
         JOIN lifts l USING (lift_id)
         WHERE l.movement_name = 'Squat'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        attempts, 0,
        "we do not know what was attempted, so nothing is invented"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_stated_best_lift_still_counts_toward_the_total(pool: PgPool) {
    import(
        &pool,
        competition("test-competition", vec![men_80(vec![stated_best_lifter()])]),
    )
    .await;

    let total: Option<Decimal> = sqlx::query_scalar(
        "SELECT SUM(l.max_weight) FROM lifts l
         JOIN competition_participants cp USING (participant_id)
         JOIN athletes a USING (athlete_id)
         WHERE a.last_name = 'Card'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(total, Some(Decimal::from_str("467.5").unwrap()));
}
