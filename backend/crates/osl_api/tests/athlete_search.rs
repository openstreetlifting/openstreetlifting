use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use osl_api::{AppState, app};
use osl_db::Database;
use osl_domain::Movement;
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

async fn meets(pool: &PgPool) {
    common::import(
        pool,
        common::competition(
            "full",
            vec![common::men_80(vec![
                common::lifting(common::athlete("José", "Rossi"), ["45", "65", "75", "130"]),
                common::disqualified(common::athlete("Luca", "Rossi"), None),
            ])],
        ),
    )
    .await;

    let mut partial = common::competition(
        "partial",
        vec![common::men_80(vec![common::best(
            common::best(common::athlete("Marco", "Rossi"), Movement::PullUp, "70"),
            Movement::Dips,
            "80",
        )])],
    );
    partial.movements = vec![Movement::PullUp, Movement::Dips];
    common::import(pool, partial).await;
}

fn slugs(body: &Value) -> Vec<&str> {
    body["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|hit| hit["slug"].as_str().unwrap())
        .collect()
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn finds_every_athlete_ranked_first(pool: PgPool) {
    meets(&pool).await;

    let (status, body) = get(&pool, "/api/v1/athletes/search?q=rossi").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["total_items"], 3);
    assert_eq!(slugs(&body), ["jose-rossi", "marco-rossi", "luca-rossi"]);

    let rankings = |index: usize| body["data"][index]["rankings"].clone();
    assert_eq!(
        rankings(0),
        serde_json::json!(["ris", "total", "muscleup", "pullup", "dips", "squat"])
    );
    assert_eq!(rankings(1), serde_json::json!(["pullup", "dips"]));
    assert_eq!(rankings(2), serde_json::json!([]));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn matches_words_in_any_order_without_accents(pool: PgPool) {
    meets(&pool).await;

    let (_, body) = get(&pool, "/api/v1/athletes/search?q=ROSSI%20jose").await;
    assert_eq!(slugs(&body), ["jose-rossi"]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn caps_results_and_counts_all_matches(pool: PgPool) {
    meets(&pool).await;

    let (_, body) = get(&pool, "/api/v1/athletes/search?q=rossi&limit=1").await;
    assert_eq!(slugs(&body), ["jose-rossi"]);
    assert_eq!(body["total_items"], 3);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn returns_nothing_without_a_name(pool: PgPool) {
    meets(&pool).await;

    let (status, body) = get(&pool, "/api/v1/athletes/search?q=%20-%20").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["total_items"], 0);
    assert_eq!(slugs(&body), [] as [&str; 0]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn rejects_a_limit_out_of_range(pool: PgPool) {
    for limit in ["0", "21"] {
        let (status, _) = get(
            &pool,
            &format!("/api/v1/athletes/search?q=rossi&limit={limit}"),
        )
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{limit}");
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn rankings_name_filter_ignores_accents_and_order(pool: PgPool) {
    meets(&pool).await;

    let (status, body) = get(&pool, "/api/v1/rankings?q=rossi%20jose").await;
    assert_eq!(status, StatusCode::OK);
    let entries = body["data"].as_array().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0]["athlete"]["slug"], "jose-rossi");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn keeps_only_athletes_without_a_full_result(pool: PgPool) {
    meets(&pool).await;

    let (_, body) = get(
        &pool,
        "/api/v1/athletes/search?q=rossi&without_full_result=true",
    )
    .await;
    assert_eq!(slugs(&body), ["marco-rossi", "luca-rossi"]);
    assert_eq!(body["total_items"], 2);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn shows_the_best_entry_of_each_athlete(pool: PgPool) {
    meets(&pool).await;
    let mut weaker = common::competition(
        "weaker",
        vec![common::men_80(vec![common::best(
            common::athlete("Marco", "Rossi"),
            Movement::PullUp,
            "60",
        )])],
    );
    weaker.movements = vec![Movement::PullUp];
    common::import(&pool, weaker).await;

    let (_, body) = get(
        &pool,
        "/api/v1/athletes/search?q=rossi&without_full_result=true",
    )
    .await;
    let marco = &body["data"][0]["best_entry"];
    assert_eq!(marco["competition"]["slug"], "partial");
    assert_eq!(marco["federation"]["name"], "Test Federation");
    assert_eq!(marco["event"], "PD");
    assert_eq!(marco["category"], "-80kg");
    assert_eq!(marco["pullup"], 70.0);
    assert_eq!(marco["dips"], 80.0);
    assert_eq!(marco["squat"], serde_json::Value::Null);
    assert_eq!(marco["status"], "competed");

    let luca = &body["data"][1]["best_entry"];
    assert_eq!(luca["competition"]["slug"], "full");
    assert_eq!(luca["status"], "disqualified");
    for lift in ["muscleup", "pullup", "dips", "squat"] {
        assert_eq!(luca[lift], serde_json::Value::Null, "{lift}");
    }
}
