//! `list_distinct_classes` is built with a QueryBuilder, so nothing checks its
//! SQL at compile time the way the `query!` macros are checked.

use osl_db::repository::ranking::RankingRepository;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

struct Meet {
    competition_id: Uuid,
    federation_id: Uuid,
}

async fn competition(pool: &PgPool) -> Meet {
    let federation_id: Uuid = sqlx::query_scalar(
        "INSERT INTO federations (name) VALUES ('Test Federation') RETURNING federation_id",
    )
    .fetch_one(pool)
    .await
    .unwrap();

    let competition_id: Uuid = sqlx::query_scalar(
        "INSERT INTO competitions (name, slug, status, federation_id, start_date, end_date, country)
         VALUES ('Test Open', 'test-open', 'completed', $1, '2026-01-01', '2026-01-01', 'FR')
         RETURNING competition_id",
    )
    .bind(federation_id)
    .fetch_one(pool)
    .await
    .unwrap();

    Meet {
        competition_id,
        federation_id,
    }
}

async fn enter(
    pool: &PgPool,
    meet: &Meet,
    gender: &str,
    min: Option<i32>,
    max: Option<i32>,
    division: Option<&str>,
) {
    let weight_class_id: Uuid = sqlx::query_scalar(
        "INSERT INTO weight_classes (gender, min_kg, max_kg) VALUES ($1, $2, $3)
         ON CONFLICT ON CONSTRAINT weight_class_bounds_unique DO UPDATE SET gender = EXCLUDED.gender
         RETURNING weight_class_id",
    )
    .bind(gender)
    .bind(min.map(Decimal::from))
    .bind(max.map(Decimal::from))
    .fetch_one(pool)
    .await
    .unwrap();

    let division_id: Option<Uuid> = match division {
        Some(name) => Some(
            sqlx::query_scalar(
                "INSERT INTO divisions (federation_id, name) VALUES ($1, $2)
                 ON CONFLICT ON CONSTRAINT division_name_unique_per_federation
                 DO UPDATE SET name = EXCLUDED.name
                 RETURNING division_id",
            )
            .bind(meet.federation_id)
            .bind(name)
            .fetch_one(pool)
            .await
            .unwrap(),
        ),
        None => None,
    };

    let handle = Uuid::new_v4().to_string();
    let athlete_id: Uuid = sqlx::query_scalar(
        "INSERT INTO athletes (first_name, last_name, gender, country, slug, match_key)
         VALUES ('Test', $1, $2, 'FR', $1, $1)
         RETURNING athlete_id",
    )
    .bind(&handle)
    .bind(gender)
    .fetch_one(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO competition_participants (competition_id, weight_class_id, division_id, athlete_id)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(meet.competition_id)
    .bind(weight_class_id)
    .bind(division_id)
    .bind(athlete_id)
    .execute(pool)
    .await
    .unwrap();
}

#[sqlx::test(migrations = "./migrations")]
async fn classes_are_listed_lightest_first(pool: PgPool) {
    let meet = competition(&pool).await;
    enter(&pool, &meet, "M", Some(94), Some(101), None).await;
    enter(&pool, &meet, "M", None, Some(66), None).await;
    enter(&pool, &meet, "M", Some(101), None, None).await;
    enter(&pool, &meet, "M", Some(73), Some(80), None).await;

    let classes = RankingRepository::new(&pool)
        .list_distinct_classes(Some("M"), None)
        .await
        .unwrap();

    assert_eq!(classes, vec!["-66kg", "-80kg", "-101kg", "+101kg"]);
}

#[sqlx::test(migrations = "./migrations")]
async fn a_gender_only_sees_its_own_classes(pool: PgPool) {
    let meet = competition(&pool).await;
    enter(&pool, &meet, "M", Some(73), Some(80), None).await;
    enter(&pool, &meet, "F", Some(63), Some(70), None).await;

    let repository = RankingRepository::new(&pool);

    assert_eq!(
        repository
            .list_distinct_classes(Some("F"), None)
            .await
            .unwrap(),
        vec!["-70kg"]
    );
    assert_eq!(
        repository
            .list_distinct_classes(Some("M"), None)
            .await
            .unwrap(),
        vec!["-80kg"]
    );
}

/// Two divisions contest the same class, and the dropdown offers it once.
#[sqlx::test(migrations = "./migrations")]
async fn one_class_run_by_two_divisions_is_listed_once(pool: PgPool) {
    let meet = competition(&pool).await;
    enter(&pool, &meet, "M", Some(73), Some(80), Some("Elite")).await;
    enter(&pool, &meet, "M", Some(73), Some(80), Some("Open")).await;

    let classes = RankingRepository::new(&pool)
        .list_distinct_classes(None, None)
        .await
        .unwrap();

    assert_eq!(classes, vec!["-80kg"]);
}

#[sqlx::test(migrations = "./migrations")]
async fn a_competition_narrows_the_list_to_what_it_contested(pool: PgPool) {
    let meet = competition(&pool).await;
    enter(&pool, &meet, "M", Some(73), Some(80), None).await;

    let repository = RankingRepository::new(&pool);

    assert_eq!(
        repository
            .list_distinct_classes(None, Some(meet.competition_id))
            .await
            .unwrap(),
        vec!["-80kg"]
    );
    assert!(
        repository
            .list_distinct_classes(None, Some(Uuid::new_v4()))
            .await
            .unwrap()
            .is_empty()
    );
}
