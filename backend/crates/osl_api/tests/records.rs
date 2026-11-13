use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use osl_api::{AppState, app};
use osl_db::Database;
use osl_domain::WeightClassSlug;
use serde_json::{Value, json};
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
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

async fn a_meet(pool: &PgPool) {
    common::import(
        pool,
        common::competition(
            "worlds",
            vec![
                common::men_80(vec![
                    common::lifting(common::athlete("Alex", "Martin"), ["45", "65", "75", "130"]),
                    common::lifting(common::athlete("Ben", "Dupont"), ["40", "70", "70", "120"]),
                ]),
                common::category(
                    WeightClassSlug::MPlus101,
                    vec![common::lifting(
                        common::athlete("Hugo", "Heavy"),
                        ["20", "60", "90", "200"],
                    )],
                ),
            ],
        ),
    )
    .await;
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn groups_every_record_without_parameters(pool: PgPool) {
    a_meet(&pool).await;
    let (status, body) = get(&pool, "/api/v1/records").await;
    assert_eq!(status, StatusCode::OK);

    let groups = body.as_array().unwrap();
    assert_eq!(
        groups.len(),
        12,
        "four movements, each with open, M-80 and M+101"
    );
    assert_eq!(
        groups
            .iter()
            .take(3)
            .map(|group| (group["movement"].clone(), group["weight_class"].clone()))
            .collect::<Vec<_>>(),
        vec![
            (json!("muscleup"), Value::Null),
            (json!("muscleup"), json!("M-80")),
            (json!("muscleup"), json!("M+101")),
        ]
    );

    let open = &groups[0];
    assert_eq!(open["gender"], "M");
    assert_eq!(open["entries"].as_array().unwrap().len(), 3);
    assert_eq!(
        open["entries"][0],
        json!({
            "rank": 1,
            "athlete": {
                "first_name": "Alex",
                "last_name": "Martin",
                "slug": "alex-martin",
                "country": "FR",
            },
            "weight": "45",
            "date": "2026-01-01",
            "competition": { "name": "worlds", "slug": "worlds" },
        })
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn narrows_to_one_group(pool: PgPool) {
    a_meet(&pool).await;
    let (status, body) = get(
        &pool,
        "/api/v1/records?movement=squat&gender=M&weight_class=M%2B101&limit=1",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body,
        json!([{
            "movement": "squat",
            "gender": "M",
            "weight_class": "M+101",
            "entries": [{
                "rank": 1,
                "athlete": {
                    "first_name": "Hugo",
                    "last_name": "Heavy",
                    "slug": "hugo-heavy",
                    "country": "FR",
                },
                "weight": "200",
                "date": "2026-01-01",
                "competition": { "name": "worlds", "slug": "worlds" },
            }],
        }])
    );

    let (_, open) = get(
        &pool,
        "/api/v1/records?movement=pullup&weight_class=open&limit=2",
    )
    .await;
    let entries = open[0]["entries"].as_array().unwrap();
    assert_eq!(
        entries
            .iter()
            .map(|entry| entry["athlete"]["last_name"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["Dupont", "Martin"]
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn returns_nothing_for_a_country_without_athletes(pool: PgPool) {
    a_meet(&pool).await;
    let (status, body) = get(&pool, "/api/v1/records?country=it").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!([]));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn rejects_invalid_parameters(pool: PgPool) {
    for uri in [
        "/api/v1/records?limit=0",
        "/api/v1/records?limit=51",
        "/api/v1/records?weight_class=M-75",
        "/api/v1/records?gender=MX",
        "/api/v1/records?movement=total",
    ] {
        let (status, _) = get(&pool, uri).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{uri}");
    }
}
