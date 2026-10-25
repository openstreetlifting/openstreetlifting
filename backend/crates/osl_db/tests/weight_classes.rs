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
async fn seeds_standard_classes(pool: PgPool) {
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM weight_classes WHERE slug IS NOT NULL")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 12);
}

#[sqlx::test(migrations = "./migrations")]
async fn rejects_duplicate_bounds(pool: PgPool) {
    insert(&pool, "M", Some(75), Some(82)).await.unwrap();
    assert!(insert(&pool, "M", Some(75), Some(82)).await.is_err());
}

#[sqlx::test(migrations = "./migrations")]
async fn rejects_duplicate_open_classes(pool: PgPool) {
    insert(&pool, "M", Some(120), None).await.unwrap();
    assert!(insert(&pool, "M", Some(120), None).await.is_err());
}

#[sqlx::test(migrations = "./migrations")]
async fn rejects_classes_without_bounds(pool: PgPool) {
    assert!(insert(&pool, "M", None, None).await.is_err());
}

#[sqlx::test(migrations = "./migrations")]
async fn rejects_reversed_bounds(pool: PgPool) {
    assert!(insert(&pool, "M", Some(90), Some(80)).await.is_err());
}
