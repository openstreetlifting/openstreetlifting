use osl_domain::FULL_EVENT;
use sqlx::PgPool;
use uuid::Uuid;

mod common;

use common::{announcement, athlete, competition, lifting, men_80};

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn an_announcement_imports_without_results(pool: PgPool) {
    common::import(&pool, announcement("test-open")).await;

    let row =
        sqlx::query!(r#"SELECT status, event_code FROM competitions WHERE slug = 'test-open'"#)
            .fetch_one(&pool)
            .await
            .unwrap();

    assert_eq!(row.status, "upcoming");
    assert_eq!(row.event_code, None);
    assert_eq!(participant_count(&pool).await, 0);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn results_fill_in_the_announced_competition(pool: PgPool) {
    common::import(&pool, announcement("test-open")).await;
    let announced = competition_id(&pool).await;

    let lifter = lifting(athlete("John", "Doe"), ["50", "60", "80", "120"]);
    common::import(&pool, competition("test-open", vec![men_80(vec![lifter])])).await;

    assert_eq!(competition_id(&pool).await, announced);
    assert_eq!(participant_count(&pool).await, 1);

    let row =
        sqlx::query!(r#"SELECT status, event_code FROM competitions WHERE slug = 'test-open'"#)
            .fetch_one(&pool)
            .await
            .unwrap();

    assert_eq!(row.status, "completed");
    assert_eq!(row.event_code.as_deref(), Some(FULL_EVENT));
}

async fn competition_id(pool: &PgPool) -> Uuid {
    sqlx::query_scalar!(
        r#"SELECT competition_id as "competition_id: Uuid" FROM competitions WHERE slug = 'test-open'"#
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn participant_count(pool: &PgPool) -> i64 {
    sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM competition_participants"#)
        .fetch_one(pool)
        .await
        .unwrap()
}
