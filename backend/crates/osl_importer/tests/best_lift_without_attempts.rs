//! Some sources publish only the best lift per movement, never the attempts
//! behind it. The transformer must store that weight without inventing an
//! attempt, and totals must still add up.

use osl_importer::canonical::{models::CanonicalFormat, transformer::CanonicalTransformer};
use rust_decimal::Decimal;
use serde_json::{Value, json};
use sqlx::PgPool;
use std::str::FromStr;

fn athlete(first: &str, last: &str, lifts: Vec<Value>) -> Value {
    json!({
        "first_name": first, "last_name": last, "country": "FR",
        "bodyweight": 80, "status": "competed", "lifts": lifts,
    })
}

fn meet(slug: &str, athletes: Vec<Value>) -> CanonicalFormat {
    serde_json::from_value(json!({
        "format_version": "1.6.0",
        "source": { "type": "manual", "extracted_at": "2026-01-01T00:00:00Z", "extractor": "best-lift-test" },
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

fn stated_best_lifter() -> Value {
    athlete(
        "Result",
        "Card",
        vec![
            json!({ "movement": "Muscle-up", "best_lift": "47.5" }),
            json!({ "movement": "Pull-up", "best_lift": "90" }),
            json!({ "movement": "Dips", "best_lift": "130" }),
            json!({ "movement": "Squat", "best_lift": "200" }),
        ],
    )
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_stated_best_lift_is_stored_as_max_weight(pool: PgPool) {
    import(&pool, meet("test-meet", vec![stated_best_lifter()])).await;

    assert_eq!(best(&pool, "Card", "Squat").await, Some(Decimal::from(200)));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_stated_best_lift_creates_no_attempt_rows(pool: PgPool) {
    import(&pool, meet("test-meet", vec![stated_best_lifter()])).await;

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
    import(&pool, meet("test-meet", vec![stated_best_lifter()])).await;

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
