mod common;

use sqlx::PgPool;

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn imports_corrects_and_removes_series(pool: PgPool) {
    for series in [Some("original-series"), Some("corrected-series"), None] {
        let mut canonical = common::announcement("series-test");
        canonical.competition.series = series.map(str::to_string);
        common::import(&pool, canonical).await;
        let stored: Option<String> =
            sqlx::query_scalar("SELECT series FROM competitions WHERE slug = 'series-test'")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(stored.as_deref(), series);
    }
}
