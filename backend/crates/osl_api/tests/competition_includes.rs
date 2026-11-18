use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use osl_api::{AppState, app};
use osl_db::Database;
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;

#[path = "../../osl_importer/tests/common/mod.rs"]
mod common;

const SECTIONS: &[(&str, &str)] = &[
    ("federation", "federation"),
    ("movements", "movements"),
    ("results", "categories"),
    ("editions", "editions"),
];

async fn get(pool: &PgPool, uri: &str) -> (StatusCode, Value) {
    let response = app(AppState::new(Database::from_pool(pool.clone()), false))
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

async fn a_meet(pool: &PgPool, slug: &str) {
    common::import(
        pool,
        common::competition(
            slug,
            vec![common::men_80(vec![common::lifting(
                common::athlete("Alex", "Martin"),
                ["45", "65", "75", "130"],
            )])],
        ),
    )
    .await;
}

fn present(body: &Value) -> Vec<&str> {
    SECTIONS
        .iter()
        .map(|(_, field)| *field)
        .filter(|field| body.get(*field).is_some())
        .collect()
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn returns_only_the_requested_section(pool: PgPool) {
    a_meet(&pool, "worlds").await;

    for (include, field) in SECTIONS {
        let uri = format!("/api/v1/competitions/worlds?include={include}");
        let (status, body) = get(&pool, &uri).await;
        assert_eq!(status, StatusCode::OK, "{include}");
        assert_eq!(present(&body), vec![*field], "{include}");
    }

    let (_, plain) = get(&pool, "/api/v1/competitions/worlds").await;
    assert!(present(&plain).is_empty(), "{plain}");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn sets_the_event_code_only_with_movements(pool: PgPool) {
    a_meet(&pool, "worlds").await;

    let (_, plain) = get(&pool, "/api/v1/competitions/worlds").await;
    assert_eq!(plain["event_code"], Value::Null);

    let (_, body) = get(&pool, "/api/v1/competitions/worlds?include=movements").await;
    assert_eq!(body["event_code"], "MPDS");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn attaches_list_sections_to_the_right_competition(pool: PgPool) {
    a_meet(&pool, "worlds").await;
    a_meet(&pool, "euros").await;

    let (status, body) = get(&pool, "/api/v1/competitions?include=federation,movements").await;
    assert_eq!(status, StatusCode::OK);
    let competitions = body["data"].as_array().unwrap();
    assert_eq!(competitions.len(), 2);
    for competition in competitions {
        assert_eq!(
            competition["federation"]["federation_id"],
            competition["federation_id"]
        );
        assert_eq!(competition["event_code"], "MPDS");
        assert_eq!(competition["movements"].as_array().unwrap().len(), 4);
        assert_eq!(competition["lifter_count"], 1);
    }

    let (_, only_federation) = get(&pool, "/api/v1/competitions?include=federation").await;
    assert!(only_federation["data"][0]["federation"].is_object());
    assert_eq!(only_federation["data"][0].get("movements"), None);
    assert_eq!(only_federation["data"][0]["event_code"], json!(null));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn rejects_includes_outside_each_endpoint(pool: PgPool) {
    for uri in [
        "/api/v1/competitions?include=results",
        "/api/v1/competitions?include=editions",
        "/api/v1/competitions/nowhere?include=bogus",
    ] {
        let (status, body) = get(&pool, uri).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{uri}");
        assert!(body["error"].as_str().unwrap().contains("expected"));
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn filters_facets_and_lists_with_query_values(pool: PgPool) {
    a_meet(&pool, "worlds").await;

    let (status, list) = get(&pool, "/api/v1/competitions?year=2026&event=MPDS").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list["data"].as_array().unwrap().len(), 1);

    let (_, other_year) = get(&pool, "/api/v1/competitions?year=1999").await;
    assert_eq!(other_year["data"], json!([]));

    let (status, facets) = get(&pool, "/api/v1/competitions/facets?year=2026").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(facets["years"], json!([2026]));

    let (status, _) = get(&pool, "/api/v1/competitions?year=soon").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}
