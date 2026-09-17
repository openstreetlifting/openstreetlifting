use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use osl_api::{AppState, app};
use osl_db::Database;
use serde_json::{Value, json};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

async fn get(uri: &str) -> (StatusCode, Vec<u8>) {
    let pool = PgPoolOptions::new()
        .connect_lazy("postgres://unused:unused@localhost/unused")
        .unwrap();
    let response = app(AppState::new(Database::from_pool(pool), false))
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    (status, body.to_vec())
}

#[tokio::test]
async fn formulas_are_a_collection_with_or_without_a_year() {
    let (status, body) = get("/api/v1/ris/formulas").await;
    assert_eq!(status, StatusCode::OK);
    let all: Vec<Value> = serde_json::from_slice(&body).unwrap();
    let years: Vec<_> = all.iter().map(|formula| formula["year"].clone()).collect();
    assert_eq!(years, vec![json!(2024), json!(2025), json!(2026)]);

    for formula in all {
        let (status, body) = get(&format!("/api/v1/ris/formulas?year={}", formula["year"])).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            serde_json::from_slice::<Value>(&body).unwrap(),
            json!([formula])
        );
    }
}

#[tokio::test]
async fn an_unpublished_year_returns_an_empty_collection() {
    let (status, body) = get("/api/v1/ris/formulas?year=1900").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(serde_json::from_slice::<Value>(&body).unwrap(), json!([]));
}

#[tokio::test]
async fn malformed_year_filters_are_rejected() {
    for year in ["invalid", "2026.5", "", "999999999999"] {
        assert_eq!(
            get(&format!("/api/v1/ris/formulas?year={year}")).await.0,
            StatusCode::BAD_REQUEST,
        );
    }
}

#[tokio::test]
async fn the_year_path_is_removed_and_current_still_resolves() {
    assert_eq!(
        get("/api/v1/ris/formulas/2026").await.0,
        StatusCode::NOT_FOUND
    );
    let (status, body) = get("/api/v1/ris/formulas/current").await;
    assert_eq!(status, StatusCode::OK);
    let formula: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(formula["is_current"], true);
    assert!(formula.is_object());
}
