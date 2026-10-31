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

async fn get(pool: &PgPool, path: &str) -> (StatusCode, Value) {
    let response = app(AppState::new(Database::from_pool(pool.clone()), false))
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/competitions{path}"))
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
        "INSERT INTO federations (name) VALUES ('Organizer') RETURNING federation_id",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    // Names and locations do not define a series, and editions need not be annual.
    for (slug, series, date, status) in [
        ("current", Some("worlds"), Some("2026-10-01"), "upcoming"),
        ("renamed", Some("worlds"), Some("2025-05-01"), "completed"),
        ("older", Some("worlds"), Some("2023-01-01"), "completed"),
        ("same-year", Some("worlds"), Some("2026-02-01"), "completed"),
        ("same-day", Some("worlds"), Some("2026-10-01"), "completed"),
        ("future", Some("worlds"), Some("2027-01-01"), "completed"),
        ("draft", Some("worlds"), Some("2024-01-01"), "draft"),
        ("cancelled", Some("worlds"), Some("2024-01-01"), "cancelled"),
        ("live", Some("worlds"), Some("2024-01-01"), "live"),
        ("upcoming", Some("worlds"), Some("2024-01-01"), "upcoming"),
        (
            "unrelated",
            Some("different-worlds"),
            Some("2025-01-01"),
            "completed",
        ),
        ("unassigned", None, Some("2026-01-01"), "completed"),
        ("unassigned-past", None, Some("2025-01-01"), "completed"),
    ] {
        sqlx::query(
            "INSERT INTO competitions (slug, name, federation_id, series, start_date, end_date, status)
             VALUES ($1, 'Worlds', $2, $3, $4::text::date, $4::text::date, $5)",
        )
        .bind(slug)
        .bind(federation)
        .bind(series)
        .bind(date)
        .bind(status)
        .execute(pool)
        .await
        .unwrap();
    }
    // A new organizer can take over a series without breaking its history.
    sqlx::query("INSERT INTO federations (name) VALUES ('Previous organizer')")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query(
        "UPDATE competitions SET name = 'Old name', city = 'Elsewhere',
        federation_id = (SELECT federation_id FROM federations WHERE name = 'Previous organizer')
        WHERE slug = 'renamed'",
    )
    .execute(pool)
    .await
    .unwrap();
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn lists_only_earlier_completed_editions_of_the_explicit_series(pool: PgPool) {
    seed(&pool).await;
    for include in [
        "previous_editions",
        "federation,movements,results,previous_editions",
    ] {
        let (status, body) = get(&pool, &format!("/current?include={include}")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            body["previous_editions"],
            json!([
                {"slug": "same-year", "name": "Worlds", "start_date": "2026-02-01"},
                {"slug": "renamed", "name": "Old name", "start_date": "2025-05-01"},
                {"slug": "older", "name": "Worlds", "start_date": "2023-01-01"},
            ])
        );
    }
    let (_, earlier) = get(&pool, "/renamed?include=previous_editions").await;
    assert_eq!(earlier["previous_editions"].as_array().unwrap().len(), 1);
    assert_eq!(earlier["previous_editions"][0]["slug"], "older");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn handles_absent_series_and_keeps_the_section_opt_in(pool: PgPool) {
    seed(&pool).await;
    for slug in ["older", "unassigned", "unassigned-past", "unrelated"] {
        let (status, body) = get(&pool, &format!("/{slug}?include=previous_editions")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["previous_editions"], json!([]), "{slug}");
    }
    let (_, body) = get(&pool, "/current").await;
    assert!(body.get("previous_editions").is_none());
    assert_eq!(
        get(&pool, "/missing?include=previous_editions").await.0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        get(&pool, "/current?include=editions").await.0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        get(&pool, "?include=previous_editions").await.0,
        StatusCode::BAD_REQUEST
    );
}
