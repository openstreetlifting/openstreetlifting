use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use osl_api::{AppState, app};
use osl_db::Database;
use osl_importer::canonical::models::CanonicalFormat;
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;

#[path = "../../osl_importer/tests/common/mod.rs"]
mod common;

async fn facets(pool: &PgPool, query: &str) -> (StatusCode, Value) {
    let response = app(AppState::new(Database::from_pool(pool.clone()), false))
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/rankings/facets{query}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

fn meet(slug: &str, year: i32, country: &str, last_name: &str) -> CanonicalFormat {
    let athlete = common::from(common::athlete("Alex", last_name), country);
    let mut meet = common::competition(
        slug,
        vec![common::men_80(vec![common::lifting(
            athlete,
            ["45", "65", "75", "130"],
        )])],
    );
    meet.competition.start_date = chrono::NaiveDate::from_ymd_opt(year, 6, 1).unwrap();
    meet.competition.end_date = meet.competition.start_date;
    meet
}

async fn two_meets(pool: &PgPool) {
    common::import(pool, meet("paris-open", 2025, "FR", "Martin")).await;
    common::import(pool, meet("rome-open", 2026, "IT", "Rossi")).await;
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn narrows_each_option_list_by_the_other_filters(pool: PgPool) {
    two_meets(&pool).await;

    let (status, all) = facets(&pool, "").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(all["countries"], json!(["FR", "IT"]));
    assert_eq!(all["years"], json!([2026, 2025]));

    let (_, french) = facets(&pool, "?country=FR").await;
    assert_eq!(french["years"], json!([2025]));
    assert_eq!(french["countries"], json!(["FR", "IT"]));

    let (_, impossible) = facets(&pool, "?country=FR&year=2026").await;
    assert_eq!(impossible["countries"], json!(["IT"]));
    assert_eq!(impossible["years"], json!([2025]));
    assert_eq!(impossible["classes"], json!([]));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn ignores_search_sort_and_paging(pool: PgPool) {
    two_meets(&pool).await;

    let (_, plain) = facets(&pool, "?country=FR").await;
    let (_, searched) = facets(&pool, "?country=FR&q=nobody&movement=squat&page=9").await;
    assert_eq!(plain, searched);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn scopes_options_to_one_competition(pool: PgPool) {
    two_meets(&pool).await;
    let competition_id: uuid::Uuid =
        sqlx::query_scalar("SELECT competition_id FROM competitions WHERE slug = 'rome-open'")
            .fetch_one(&pool)
            .await
            .unwrap();

    let (_, rome) = facets(&pool, &format!("?competition_id={competition_id}")).await;
    assert_eq!(rome["countries"], json!(["IT"]));
    assert_eq!(rome["classes"], json!(["-80kg"]));

    let (status, _) = facets(&pool, "?gender=MX").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}
