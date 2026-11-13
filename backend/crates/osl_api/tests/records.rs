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

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn filters_by_federation_and_year(pool: PgPool) {
    a_meet(&pool).await;

    let (status, known) = get(
        &pool,
        "/api/v1/records?federation=Test%20Federation&year=2026",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(known.as_array().unwrap().len(), 12);

    for uri in [
        "/api/v1/records?federation=Elsewhere",
        "/api/v1/records?year=2020",
    ] {
        let (_, body) = get(&pool, uri).await;
        assert_eq!(body, json!([]), "{uri}");
    }

    let mut upcoming = common::announcement("next-year");
    upcoming.competition.start_date = chrono::NaiveDate::from_ymd_opt(2027, 6, 1).unwrap();
    upcoming.competition.end_date = upcoming.competition.start_date;
    common::import(&pool, upcoming).await;
    let (_, years) = get(&pool, "/api/v1/records/years").await;
    assert_eq!(years, json!([2026]), "an upcoming meet has no lifts yet");

    let (status, _) = get(&pool, "/api/v1/records?year=soon").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn marks_current_records_on_rankings_results_and_history(pool: PgPool) {
    a_meet(&pool).await;

    let (_, rankings) = get(&pool, "/api/v1/rankings?movement=muscleup").await;
    let leader = &rankings["data"][0];
    assert_eq!(leader["athlete"]["slug"], "alex-martin");
    assert!(
        leader["records"]
            .as_array()
            .unwrap()
            .contains(&json!({ "movement": "Muscle-up", "mark": "WR" }))
    );

    let (_, competition) = get(&pool, "/api/v1/competitions/worlds?include=results").await;
    let lifts = competition["categories"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|category| category["participants"].as_array().unwrap())
        .find(|participant| participant["athlete"]["slug"] == "ben-dupont")
        .unwrap()["lifts"]
        .as_array()
        .unwrap()
        .clone();
    let mark = |movement: &str| {
        lifts
            .iter()
            .find(|lift| lift["movement_name"] == movement)
            .unwrap()["record"]
            .clone()
    };
    assert_eq!(mark("Pull-up"), "WR", "Ben out-pulled Alex in M-80");
    assert_eq!(
        mark("Muscle-up"),
        Value::Null,
        "Alex holds both muscle-up records"
    );

    let (_, athlete) = get(&pool, "/api/v1/athletes/hugo-heavy?include=competitions").await;
    let squat = athlete["competitions"][0]["lifts"]
        .as_array()
        .unwrap()
        .iter()
        .find(|lift| lift["movement_name"] == "Squat")
        .unwrap()
        .clone();
    assert_eq!(squat["record"], "WR");
}
