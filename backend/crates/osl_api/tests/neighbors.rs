use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use osl_api::{AppState, app};
use osl_db::Database;
use serde_json::Value;
use sqlx::PgPool;
use tower::ServiceExt;

#[path = "../../osl_importer/tests/common/mod.rs"]
mod common;

async fn get(pool: &PgPool, uri: &str) -> (StatusCode, Value) {
    let response = app(AppState::new(Database::from_pool(pool.clone()), false))
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (status, serde_json::from_slice(&bytes).unwrap())
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn includes_total_neighbors_with_exact_gaps_and_both_scopes(pool: PgPool) {
    common::import(
        &pool,
        common::competition(
            "neighbors",
            vec![common::men_80(vec![
                common::lifting(common::athlete("Alex", "Martin"), ["45", "65", "75", "130"]),
                common::lifting(common::athlete("Alina", "Riyaz"), ["45", "65", "75", "130"]),
            ])],
        ),
    )
    .await;
    let (status, body) = get(
        &pool,
        "/api/v1/athletes/alex-martin?include=standing,neighbors",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let neighbors = &body["neighbors"];
    assert_eq!(neighbors["category"], "Men -80kg");
    for scope in ["global", "national"] {
        let rows = neighbors[scope].as_array().unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0]["place"], 1);
        assert_eq!(rows[1]["place"], 2);
        assert!(
            rows.iter()
                .all(|r| r["value"] == "315" && r["gap"] == "0" && r["field"] == 2)
        );
        let own = rows.iter().find(|r| r["slug"] == "alex-martin").unwrap();
        assert_eq!(own["place"], body["standing"]["total"]["global"]["place"]);
        assert_eq!(own["competition_slug"], "neighbors");
        assert_eq!(own["competition_date"], "2026-01-01");
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn omits_unrequested_neighbors_and_returns_null_without_an_all4_total(pool: PgPool) {
    let mut competition = common::competition(
        "pull",
        vec![common::men_80(vec![common::best(
            common::athlete("Alex", "Martin"),
            osl_domain::Movement::PullUp,
            "80",
        )])],
    );
    competition.movements = vec![osl_domain::Movement::PullUp];
    common::import(&pool, competition).await;
    let (status, body) = get(&pool, "/api/v1/athletes/alex-martin").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.get("neighbors").is_none());
    let (status, body) = get(&pool, "/api/v1/athletes/alex-martin?include=neighbors").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.get("neighbors"), Some(&Value::Null));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn missing_nationality_keeps_global_neighbors_and_null_national(pool: PgPool) {
    let mut athlete = common::lifting(common::athlete("Alex", "Martin"), ["45", "65", "75", "130"]);
    athlete.country = None;
    common::import(
        &pool,
        common::competition("alone", vec![common::men_80(vec![athlete])]),
    )
    .await;
    let (status, body) = get(&pool, "/api/v1/athletes/alex-martin?include=neighbors").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["neighbors"]["global"].as_array().unwrap().len(), 1);
    assert_eq!(body["neighbors"]["national"], Value::Null);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn standing_is_served_from_the_precomputed_table(pool: PgPool) {
    common::import(
        &pool,
        common::competition(
            "worlds",
            vec![common::men_80(vec![
                common::lifting(common::athlete("Alex", "Martin"), ["45", "65", "75", "130"]),
                common::lifting(common::athlete("Alina", "Riyaz"), ["45", "65", "75", "125"]),
            ])],
        ),
    )
    .await;

    sqlx::query("UPDATE athlete_metric_standings SET global_place = 7, global_field = 42 WHERE metric = 'total' AND global_place = 1")
        .execute(&pool)
        .await
        .unwrap();

    let (_, body) = get(&pool, "/api/v1/athletes/alex-martin?include=standing").await;
    let total = &body["standing"]["total"]["global"];
    assert_eq!(total["place"], 7);
    assert_eq!(total["field"], 42);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn missing_precomputed_rows_are_omitted(pool: PgPool) {
    common::import(
        &pool,
        common::competition(
            "worlds",
            vec![common::men_80(vec![common::lifting(
                common::athlete("Alex", "Martin"),
                ["45", "65", "75", "130"],
            )])],
        ),
    )
    .await;

    sqlx::query("DELETE FROM athlete_metric_standings")
        .execute(&pool)
        .await
        .unwrap();

    let (status, body) = get(&pool, "/api/v1/athletes/alex-martin?include=standing").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.get("standing"), Some(&Value::Null));
}
