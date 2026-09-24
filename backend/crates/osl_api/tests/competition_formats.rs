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
use uuid::Uuid;

async fn get(pool: &PgPool, query: &str) -> (StatusCode, Value) {
    let response = app(AppState::new(Database::from_pool(pool.clone()), false))
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/competitions{query}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    (status, serde_json::from_slice(&body).unwrap())
}

async fn seed(pool: &PgPool) {
    let federation: Uuid = sqlx::query_scalar(
        "INSERT INTO federations (name) VALUES ('Test federation') RETURNING federation_id",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    for (slug, event, status, country) in [
        ("pd-fr", Some("PD"), "completed", "FR"),
        ("pd-de", Some("PD"), "completed", "DE"),
        ("mpd", Some("MPD"), "completed", "FR"),
        ("mpds", Some("MPDS"), "completed", "FR"),
        ("unknown", None, "completed", "FR"),
        ("upcoming-pd", Some("PD"), "upcoming", "FR"),
        ("upcoming-unknown", None, "upcoming", "FR"),
    ] {
        let id: Uuid = sqlx::query_scalar(
            "INSERT INTO competitions (name, slug, federation_id, event_code, status, country, start_date, end_date)
             VALUES ($1, $1, $2, $3, $4, $5, '2026-01-01', '2026-01-01') RETURNING competition_id"
        ).bind(slug).bind(federation).bind(event).bind(status).bind(country)
            .fetch_one(pool).await.unwrap();
        if let Some(event) = event {
            sqlx::query(
                "INSERT INTO competition_movements (competition_id, movement_name, display_order)
                SELECT $1, name, display_order FROM movements WHERE position(code in $2) > 0",
            )
            .bind(id)
            .bind(event)
            .execute(pool)
            .await
            .unwrap();
        }
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn exact_formats_filter_rows_and_totals_before_pagination(pool: PgPool) {
    seed(&pool).await;
    let mut slugs = Vec::new();
    for page in [1, 2] {
        let (status, result) = get(
            &pool,
            &format!("?event=PD&status=completed&include=movements&page_size=1&page={page}"),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(result["pagination"]["total_items"], 2);
        assert_eq!(result["pagination"]["total_pages"], 2);
        assert_eq!(result["data"].as_array().unwrap().len(), 1);
        assert_eq!(result["data"][0]["event_code"], "PD");
        slugs.push(result["data"][0]["slug"].as_str().unwrap().to_string());
    }
    slugs.sort();
    assert_eq!(slugs, ["pd-de", "pd-fr"]);
    let (_, filtered) = get(
        &pool,
        "?event=PD&status=completed&country=FR&year=2026&q=pd",
    )
    .await;
    assert_eq!(filtered["pagination"]["total_items"], 1);
    assert_eq!(filtered["data"][0]["slug"], "pd-fr");
    let (_, upcoming) = get(&pool, "?event=PD&status=upcoming").await;
    assert_eq!(upcoming["pagination"]["total_items"], 1);
    assert_eq!(upcoming["data"][0]["slug"], "upcoming-pd");
    let (_, all) = get(&pool, "?status=upcoming").await;
    assert_eq!(all["pagination"]["total_items"], 2);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn nonexistent_combinations_are_empty_and_choices_stay_global(pool: PgPool) {
    seed(&pool).await;
    let (status, empty) = get(&pool, "?event=MS&country=FR").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(empty["data"], json!([]));
    assert_eq!(empty["pagination"]["total_items"], 0);
    let (status, formats) = get(&pool, "/formats?event=MS&country=ZZ&year=1900").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(formats, json!(["MPD", "MPDS", "PD"]));
    for event in ["", "XX", "PP", "DP"] {
        assert_eq!(
            get(&pool, &format!("?event={event}")).await.0,
            StatusCode::BAD_REQUEST
        );
    }
}
