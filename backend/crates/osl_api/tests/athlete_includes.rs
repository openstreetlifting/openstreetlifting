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

const SECTIONS: &[(&str, &[&str])] = &[
    ("competitions", &["competitions", "total_competitions"]),
    ("records", &["personal_records", "best_total", "best_ris"]),
    ("standing", &["standing"]),
    ("lift_balance", &["lift_balance"]),
    ("neighbors", &["neighbors"]),
];

async fn get(pool: &PgPool, uri: &str) -> (StatusCode, Value) {
    let response = app(AppState::new(Database::from_pool(pool.clone()), false))
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (status, serde_json::from_slice(&bytes).unwrap())
}

async fn a_meet(pool: &PgPool) {
    common::import(
        pool,
        common::competition(
            "worlds",
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
        .flat_map(|(_, fields)| fields.iter().copied())
        .filter(|field| body.get(*field).is_some())
        .collect()
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn returns_only_the_requested_section(pool: PgPool) {
    a_meet(&pool).await;

    for (include, fields) in SECTIONS {
        let uri = format!("/api/v1/athletes/alex-martin?include={include}");
        let (status, body) = get(&pool, &uri).await;
        assert_eq!(status, StatusCode::OK, "{include}");
        assert_eq!(present(&body), *fields, "{include}");
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn serves_competitions_and_records_from_one_history(pool: PgPool) {
    a_meet(&pool).await;

    let (status, body) = get(
        &pool,
        "/api/v1/athletes/alex-martin?include=competitions,%20records",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["total_competitions"], 1);
    assert_eq!(body["competitions"][0]["competition_slug"], "worlds");
    assert_eq!(body["best_total"]["total"], "315");
    assert_eq!(body["personal_records"].as_array().unwrap().len(), 4);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn omits_every_section_without_includes(pool: PgPool) {
    a_meet(&pool).await;

    let (status, body) = get(&pool, "/api/v1/athletes/alex-martin").await;
    assert_eq!(status, StatusCode::OK);
    assert!(present(&body).is_empty(), "{body}");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn rejects_an_unknown_include_before_looking_up_the_athlete(pool: PgPool) {
    let (status, body) = get(&pool, "/api/v1/athletes/nobody?include=bogus").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let message = body.to_string();
    assert!(message.contains("bogus"), "{message}");
    assert!(message.contains("lift_balance"), "{message}");
}
