//! The unique constraint on bounds is the only thing preventing duplicate
//! weight classes, so it is worth asserting rather than assuming.

use rust_decimal::Decimal;
use sqlx::PgPool;

async fn insert(
    pool: &PgPool,
    gender: &str,
    min: Option<i32>,
    max: Option<i32>,
) -> sqlx::Result<()> {
    sqlx::query("INSERT INTO weight_classes (gender, min_kg, max_kg) VALUES ($1, $2, $3)")
        .bind(gender)
        .bind(min.map(Decimal::from))
        .bind(max.map(Decimal::from))
        .execute(pool)
        .await
        .map(|_| ())
}

#[sqlx::test(migrations = "./migrations")]
async fn the_standard_ladder_is_seeded(pool: PgPool) {
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM weight_classes WHERE slug IS NOT NULL")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 12);
}

#[sqlx::test(migrations = "./migrations")]
async fn the_same_bounds_cannot_be_inserted_twice(pool: PgPool) {
    insert(&pool, "M", Some(75), Some(82)).await.unwrap();
    assert!(insert(&pool, "M", Some(75), Some(82)).await.is_err());
}

#[sqlx::test(migrations = "./migrations")]
async fn an_open_class_cannot_be_inserted_twice(pool: PgPool) {
    // NULLS NOT DISTINCT is what makes this fail. A plain UNIQUE would let
    // both rows through, since Postgres reads two NULLs as different values.
    insert(&pool, "M", Some(120), None).await.unwrap();
    assert!(insert(&pool, "M", Some(120), None).await.is_err());
}

#[sqlx::test(migrations = "./migrations")]
async fn a_class_with_no_bound_is_rejected(pool: PgPool) {
    assert!(insert(&pool, "M", None, None).await.is_err());
}

#[sqlx::test(migrations = "./migrations")]
async fn bounds_in_the_wrong_order_are_rejected(pool: PgPool) {
    assert!(insert(&pool, "M", Some(90), Some(80)).await.is_err());
}
