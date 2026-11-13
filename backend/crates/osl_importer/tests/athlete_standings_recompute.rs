//! Parity between the precomputed standings and rivals tables and the
//! original per-athlete computed queries. After every recompute, both paths
//! must agree for every athlete in the database.

use osl_db::repository::ranking::RankingRepository;
use osl_db::services::athlete_standings;
use osl_importer::canonical::models::CanonicalFormat;
use sqlx::{PgPool, Row};
use uuid::Uuid;

mod common;

use common::{athlete, from, import, lifting, men_80, weighing};

fn a_board() -> CanonicalFormat {
    common::competition(
        "worlds",
        vec![men_80(vec![
            weighing(lifting(athlete("Best", "French"), ["60", "90", "110", "180"]), "80"),
            weighing(lifting(athlete("Second", "French"), ["40", "70", "90", "150"]), "80"),
            weighing(
                from(lifting(athlete("Only", "Italian"), ["50", "80", "100", "160"]), "IT"),
                "80",
            ),
        ])],
    )
}

/// A second competition where "Second" moves down a class and a newcomer
/// competes, so class pools and latest-class logic are exercised.
fn a_later_board() -> CanonicalFormat {
    common::competition(
        "continentals",
        vec![
            common::category(
                osl_domain::WeightClassSlug::M80,
                vec![weighing(lifting(athlete("Newcomer", "Spanish"), ["70", "100", "120", "200"]), "80")],
            ),
            common::category(
                osl_domain::WeightClassSlug::M73,
                vec![weighing(lifting(athlete("Moved", "Second"), ["55", "85", "105", "165"]), "79")],
            ),
        ],
    )
}

async fn recompute(pool: &PgPool) {
    let mut tx = pool.begin().await.expect("transaction should begin");
    athlete_standings::refresh(&mut tx).await.expect("recompute should succeed");
    tx.commit().await.expect("recompute should commit");
}

async fn athlete_ids(pool: &PgPool) -> Vec<Uuid> {
    sqlx::query_scalar("SELECT athlete_id FROM athletes ORDER BY athlete_id")
        .fetch_all(pool)
        .await
        .expect("athletes should exist")
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn stored_standings_match_computed(pool: PgPool) {
    import(&pool, a_board()).await;
    recompute(&pool).await;

    let repo = RankingRepository::new(&pool);
    for athlete_id in athlete_ids(&pool).await {
        let stored = sqlx::query(
            r#"
            SELECT metric, value, global_place, global_field, country,
                   country_place, country_field, weight_class_min, weight_class_max
            FROM athlete_metric_standings WHERE athlete_id = $1 ORDER BY metric
            "#,
        )
        .bind(athlete_id)
        .fetch_all(&pool)
        .await
        .expect("stored standings should read back");

        let computed = repo
            .get_athlete_metric_standings(athlete_id)
            .await
            .expect("computed standings should succeed");

        assert_eq!(
            stored.len(),
            computed.len(),
            "metric count should match for {athlete_id}"
        );

        for (stored_row, computed_row) in stored.iter().zip(&computed) {
            let metric: String = stored_row.get("metric");
            let value: rust_decimal::Decimal = stored_row.get("value");
            let global_place: i64 = stored_row.get("global_place");
            let global_field: i64 = stored_row.get("global_field");
            let country: Option<String> = stored_row.get("country");
            let country_place: i64 = stored_row.get("country_place");
            let country_field: i64 = stored_row.get("country_field");
            let weight_class_min: Option<rust_decimal::Decimal> = stored_row.get("weight_class_min");
            let weight_class_max: Option<rust_decimal::Decimal> =
                stored_row.get("weight_class_max");

            assert_eq!(metric, computed_row.metric.as_str(), "metric for {athlete_id}");
            assert_eq!(value, computed_row.value, "value for {athlete_id}");
            assert_eq!(
                global_place, computed_row.global_place,
                "global place for {athlete_id}"
            );
            assert_eq!(
                global_field, computed_row.global_field,
                "global field for {athlete_id}"
            );
            assert_eq!(country, computed_row.country, "country for {athlete_id}");
            assert_eq!(
                country_place, computed_row.country_place,
                "country place for {athlete_id}"
            );
            assert_eq!(
                country_field, computed_row.country_field,
                "country field for {athlete_id}"
            );
            assert_eq!(
                weight_class_min, computed_row.weight_class_min,
                "weight class bounds for {athlete_id}"
            );
        }
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn stored_standings_match_computed_after_class_change(pool: PgPool) {
    import(&pool, a_board()).await;
    import(&pool, a_later_board()).await;
    recompute(&pool).await;

    let repo = RankingRepository::new(&pool);
    for athlete_id in athlete_ids(&pool).await {
        let stored: Vec<(String, rust_decimal::Decimal, i64, i64)> = sqlx::query_as(
            "SELECT metric, value, global_place, global_field \
             FROM athlete_metric_standings WHERE athlete_id = $1 ORDER BY metric",
        )
        .bind(athlete_id)
        .fetch_all(&pool)
        .await
        .expect("stored standings should read back");

        let computed = repo
            .get_athlete_metric_standings(athlete_id)
            .await
            .expect("computed standings should succeed");

        assert_eq!(
            stored.len(),
            computed.len(),
            "metric count should match for {athlete_id}"
        );
        for ((metric, value, global_place, global_field), computed_row) in
            stored.iter().zip(&computed)
        {
            assert_eq!(metric, computed_row.metric.as_str(), "metric for {athlete_id}");
            assert_eq!(*value, computed_row.value, "value for {athlete_id}");
            assert_eq!(
                *global_place, computed_row.global_place,
                "global place for {athlete_id}"
            );
            assert_eq!(
                *global_field, computed_row.global_field,
                "global field for {athlete_id}"
            );
        }
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn stored_rivals_match_computed(pool: PgPool) {
    import(&pool, a_board()).await;
    import(&pool, a_later_board()).await;
    recompute(&pool).await;

    let repo = RankingRepository::new(&pool);
    for athlete_id in athlete_ids(&pool).await {
        let stored = sqlx::query(
            r#"
            SELECT scope, neighbor_athlete_id, place, field, value, gap
            FROM athlete_rivals WHERE profile_athlete_id = $1
            ORDER BY scope, place
            "#,
        )
        .bind(athlete_id)
        .fetch_all(&pool)
        .await
        .expect("stored rivals should read back");

        let computed = repo
            .get_athlete_total_neighbors(athlete_id)
            .await
            .expect("computed neighbors should succeed");

        assert_eq!(
            stored.len(),
            computed.len(),
            "rival count should match for {athlete_id}"
        );

        for (stored_row, computed_row) in stored.iter().zip(&computed) {
            let scope: String = stored_row.get("scope");
            let neighbor: Uuid = stored_row.get("neighbor_athlete_id");
            let place: i64 = stored_row.get("place");
            let field: i64 = stored_row.get("field");
            let value: rust_decimal::Decimal = stored_row.get("value");
            let gap: rust_decimal::Decimal = stored_row.get("gap");

            assert_eq!(scope, computed_row.scope, "scope for {athlete_id}");
            assert_eq!(neighbor, computed_row.athlete_id, "neighbor for {athlete_id}");
            assert_eq!(place, computed_row.place, "place for {athlete_id}");
            assert_eq!(field, computed_row.field, "field for {athlete_id}");
            assert_eq!(value, computed_row.value, "value for {athlete_id}");
            assert_eq!(gap, computed_row.gap, "gap for {athlete_id}");
        }
    }
}
