use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use osl_api::{AppState, app};
use osl_db::Database;
use serde_json::{Value, json};
use sqlx::PgPool;
use std::{collections::HashMap, path::Path};
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
    for (slug, series, date, status) in [
        ("current", Some("worlds"), "2026-10-01", "upcoming"),
        ("renamed", Some("worlds"), "2025-05-01", "completed"),
        ("older", Some("worlds"), "2023-01-01", "completed"),
        ("same-year", Some("worlds"), "2026-02-01", "completed"),
        ("same-day", Some("worlds"), "2026-10-01", "completed"),
        ("future", Some("worlds"), "2027-01-01", "completed"),
        ("draft", Some("worlds"), "2024-01-01", "draft"),
        ("cancelled", Some("worlds"), "2024-01-01", "cancelled"),
        ("live", Some("worlds"), "2024-01-01", "live"),
        ("upcoming", Some("worlds"), "2024-01-01", "upcoming"),
        (
            "unrelated",
            Some("different-worlds"),
            "2025-01-01",
            "completed",
        ),
        ("unassigned", None, "2026-01-01", "completed"),
        ("unassigned-past", None, "2025-01-01", "completed"),
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
async fn lists_available_editions_of_the_explicit_series_in_chronological_order(pool: PgPool) {
    seed(&pool).await;
    for include in ["editions", "federation,movements,results,editions"] {
        let (status, body) = get(&pool, &format!("/current?include={include}")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            body["editions"],
            json!([
                {"slug": "older", "name": "Worlds", "start_date": "2023-01-01"},
                {"slug": "live", "name": "Worlds", "start_date": "2024-01-01"},
                {"slug": "upcoming", "name": "Worlds", "start_date": "2024-01-01"},
                {"slug": "renamed", "name": "Old name", "start_date": "2025-05-01"},
                {"slug": "same-year", "name": "Worlds", "start_date": "2026-02-01"},
                {"slug": "current", "name": "Worlds", "start_date": "2026-10-01"},
                {"slug": "same-day", "name": "Worlds", "start_date": "2026-10-01"},
                {"slug": "future", "name": "Worlds", "start_date": "2027-01-01"},
            ])
        );
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn handles_absent_series_and_keeps_the_section_opt_in(pool: PgPool) {
    seed(&pool).await;
    for slug in ["unassigned", "unassigned-past"] {
        let (status, body) = get(&pool, &format!("/{slug}?include=editions")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["editions"], json!([]), "{slug}");
    }
    assert_editions(&pool, "unrelated", &["unrelated"]).await;
    let (_, body) = get(&pool, "/current").await;
    assert!(body.get("editions").is_none());
    assert_eq!(
        get(&pool, "/missing?include=editions").await.0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        get(&pool, "/current?include=unknown").await.0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        get(&pool, "?include=editions").await.0,
        StatusCode::BAD_REQUEST
    );
}

async fn seed_archive(pool: &PgPool) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/competitions");
    let mut directories = Vec::new();
    osl_importer::canonical::store::collect_competitions(&root, &mut directories).unwrap();
    let mut federations = HashMap::new();
    for directory in directories {
        let competition = osl_importer::canonical::store::read(&directory)
            .unwrap()
            .competition;
        let federation = if let Some(id) = federations.get(&competition.federation.name) {
            *id
        } else {
            let id: Uuid = sqlx::query_scalar(
                "INSERT INTO federations (name) VALUES ($1) RETURNING federation_id",
            )
            .bind(&competition.federation.name)
            .fetch_one(pool)
            .await
            .unwrap();
            federations.insert(competition.federation.name.clone(), id);
            id
        };
        sqlx::query(
            "INSERT INTO competitions (slug, name, federation_id, series, start_date, end_date, status)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(competition.slug)
        .bind(competition.name)
        .bind(federation)
        .bind(competition.series)
        .bind(competition.start_date)
        .bind(competition.end_date)
        .bind(competition.status.unwrap_or(osl_domain::CompetitionStatus::Completed).as_str())
        .execute(pool)
        .await
        .unwrap();
    }
}

async fn assert_editions(pool: &PgPool, slug: &str, expected: &[&str]) {
    let (status, body) = get(pool, &format!("/{slug}?include=editions")).await;
    assert_eq!(status, StatusCode::OK, "{slug}");
    let slugs: Vec<_> = body["editions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|edition| edition["slug"].as_str().unwrap())
        .collect();
    assert_eq!(slugs, expected, "{slug}");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn archive_keeps_championships_within_a_federation_separate(pool: PgPool) {
    seed_archive(&pool).await;
    assert_editions(&pool, "worlds-2025", &["worlds-2023", "worlds-2025"]).await;
    assert_editions(
        &pool,
        "german-open-2025",
        &["german-open-2024", "german-open-2025"],
    )
    .await;
    assert_editions(
        &pool,
        "portugal-open-2026",
        &["portuguese-open-2025", "portugal-open-2026"],
    )
    .await;
    assert_editions(&pool, "ace-cup-2026", &["ace-cup-2025", "ace-cup-2026"]).await;
    assert_editions(
        &pool,
        "fnsl-elite-2027",
        &["fnsl-elite-2026", "fnsl-elite-2027"],
    )
    .await;
    assert_editions(
        &pool,
        "fnsl-interregionale-centre-2027",
        &[
            "fnsl-interregionale-centre-2026",
            "fnsl-interregionale-centre-2027",
        ],
    )
    .await;
    assert_editions(
        &pool,
        "dcsv-german-championship-2026",
        &[
            "dcsv-german-championship-2022",
            "dcsv-german-championship-2023",
            "dcsv-german-championship-2024",
            "dcsv-german-championship-2026",
        ],
    )
    .await;
    assert_editions(
        &pool,
        "australian-national-finals-2026",
        &[
            "australian-national-finals-2025",
            "australian-national-finals-2026",
        ],
    )
    .await;
    assert_editions(
        &pool,
        "australian-open-2026-event-4",
        &[
            "australian-open-2025-event-1",
            "australian-open-2026-event-2",
            "australian-open-2026-event-3",
            "australian-open-2026-event-4",
        ],
    )
    .await;
    assert_editions(
        &pool,
        "usastreetlifting-nationals-3-2026",
        &[
            "usastreetlifting-national-championships-2024",
            "usastreetlifting-national-championships-2025",
            "usastreetlifting-nationals-3-2026",
        ],
    )
    .await;
    assert_editions(
        &pool,
        "bodystrong-uk-ire-nationals-2026",
        &[
            "bodystrong-uk-nationals-2024",
            "bodystrong-uk-ire-nationals-2025",
            "bodystrong-uk-ire-nationals-2026",
        ],
    )
    .await;
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn archive_keeps_contests_at_the_same_meet_separate(pool: PgPool) {
    seed_archive(&pool).await;
    assert_editions(
        &pool,
        "aesl-gods-battles-all4-2025",
        &[
            "aesl-gods-battles-all4-2021",
            "aesl-gods-battles-all4-2022",
            "aesl-gods-battles-all4-2023",
            "aesl-gods-battles-all4-2024",
            "aesl-gods-battles-all4-2025",
        ],
    )
    .await;
    assert_editions(
        &pool,
        "aesl-gods-battles-classic-2025",
        &[
            "aesl-gods-battles-classic-2021",
            "aesl-gods-battles-classic-2022",
            "aesl-gods-battles-classic-2023",
            "aesl-gods-battles-classic-2024",
            "aesl-gods-battles-classic-2025",
        ],
    )
    .await;
    assert_editions(
        &pool,
        "nationals-streetlifting-glogow-2022-single-pullup",
        &[
            "nationals-streetlifting-glogow-2021-single-pullup",
            "nationals-streetlifting-glogow-2022-single-pullup",
        ],
    )
    .await;
    assert_editions(
        &pool,
        "iii-turniej-lipka-krajenska-2024-m",
        &[
            "ii-turniej-lipka-krajenska-2023-m",
            "iii-turniej-lipka-krajenska-2024-m",
        ],
    )
    .await;
    assert_editions(
        &pool,
        "lomza-street-power-ix-2026",
        &[
            "lomza-street-power-2018",
            "lomza-street-power-viii-2025",
            "lomza-street-power-ix-2026",
        ],
    )
    .await;
    assert_editions(
        &pool,
        "lomza-street-power-viii-2025-muscleup",
        &[
            "lomza-street-power-2024",
            "lomza-street-power-viii-2025-muscleup",
        ],
    )
    .await;
    assert_editions(
        &pool,
        "lomza-street-power-2021",
        &["lomza-street-power-2019", "lomza-street-power-2021"],
    )
    .await;
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn links_older_and_later_editions_of_the_same_championship(pool: PgPool) {
    seed_archive(&pool).await;
    for slug in ["euros-2024", "euros-2026"] {
        let (status, body) = get(&pool, &format!("/{slug}?include=editions")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            body["editions"],
            json!([
                { "slug": "euros-2024", "name": "Euros", "start_date": "2024-11-02" },
                { "slug": "euros-2026", "name": "Euros", "start_date": "2026-10-02" },
            ])
        );
    }
    assert_editions(&pool, "worlds-2023", &["worlds-2023", "worlds-2025"]).await;
    assert_editions(
        &pool,
        "belgian-streetlifting-championship-2025",
        &[
            "belgian-streetlifting-championship-2025",
            "belgian-streetlifting-championship-2026",
        ],
    )
    .await;
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn includes_the_current_edition_even_when_draft_or_cancelled(pool: PgPool) {
    seed(&pool).await;
    for slug in ["draft", "cancelled"] {
        assert_editions(
            &pool,
            slug,
            &[
                "older",
                slug,
                "live",
                "upcoming",
                "renamed",
                "same-year",
                "current",
                "same-day",
                "future",
            ],
        )
        .await;
    }
}
