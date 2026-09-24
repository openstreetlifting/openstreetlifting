use axum::body::Body;
use axum::http::{Request, StatusCode};
use chrono::NaiveDate;
use http_body_util::BodyExt;
use osl_api::{AppState, app};
use osl_db::Database;
use osl_domain::{
    AthleteStatus, CompetitionStatus, CountryCode, Gender, Movement, WeightClassSlug,
};
use osl_importer::canonical::models::{
    AthleteData, AttemptData, CanonicalFormat, CategoryData, CompetitionData, FederationData,
    LiftData,
};
use osl_importer::canonical::transformer::CanonicalTransformer;
use osl_importer::sync::CompetitionSync;
use rust_decimal::Decimal;
use serde_json::Value;
use sqlx::PgPool;
use std::str::FromStr;
use tower::ServiceExt;

fn decimal(raw: &str) -> Decimal {
    Decimal::from_str(raw).unwrap()
}

fn entry(first: &str, last: &str, native: Option<&str>) -> AthleteData {
    AthleteData {
        first_name: first.to_string(),
        last_name: last.to_string(),
        native_name: native.map(str::to_string),
        disambiguation: None,
        gender: None,
        country: Some(CountryCode::parse("FR").unwrap()),
        bodyweight: Some(decimal("78.5")),
        bodyweight_source: None,
        reported_ris: None,
        reported_ris_edition: None,
        total: None,
        status: AthleteStatus::Competed,
        status_reason: None,
        lifts: Movement::ALL
            .into_iter()
            .zip(["45", "65", "75", "130"])
            .map(|(movement, weight)| LiftData {
                movement,
                attempts: Some(vec![AttemptData {
                    attempt_number: 1,
                    weight: decimal(weight),
                    is_successful: true,
                }]),
                best_lift: None,
            })
            .collect(),
    }
}

fn meet(slug: &str, athletes: Vec<AthleteData>) -> CanonicalFormat {
    CanonicalFormat {
        sources: vec!["Synthetic test results".into()],
        competition: CompetitionData {
            name: slug.to_string(),
            slug: slug.to_string(),
            federation: FederationData {
                name: "Test Federation".to_string(),
                abbreviation: None,
                country: None,
            },
            start_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            city: Some("Paris".to_string()),
            region: None,
            country: CountryCode::parse("FR").unwrap(),
            status: Some(CompetitionStatus::Completed),
        },
        movements: Movement::ALL.to_vec(),
        categories: vec![CategoryData {
            division: None,
            gender: Gender::M,
            weight_class_slug: Some(WeightClassSlug::from_str("M-80").unwrap()),
            weight_class_min: None,
            weight_class_max: None,
            athletes,
        }],
    }
}

async fn import(pool: &PgPool, mut canonical: CanonicalFormat) {
    osl_importer::canonical::format::prepare(&mut canonical).unwrap();
    CanonicalTransformer::new(pool)
        .import_to_database(canonical)
        .await
        .unwrap();
}

async fn get(pool: &PgPool, uri: &str) -> (StatusCode, Value) {
    let state = AppState::new(Database::from_pool(pool.clone()), false);

    let response = app(state)
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();

    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body = serde_json::from_slice(&bytes).unwrap_or(Value::Null);

    (status, body)
}

/// The name is replaced in the files, so the import creates a new athlete and
/// the prune takes the old one with it.
async fn redact_in_place(pool: &PgPool) {
    import(
        pool,
        meet("meet", vec![entry("", "Redacted Athlete #1", None)]),
    )
    .await;

    CompetitionSync::new(pool)
        .apply(&["meet".to_string()])
        .await
        .unwrap();
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn the_athlete_page_carries_no_identity(pool: PgPool) {
    import(
        &pool,
        meet("meet", vec![entry("Alina", "Riyaz", Some("Алина Рияз"))]),
    )
    .await;
    redact_in_place(&pool).await;

    let (status, body) = get(&pool, "/api/v1/athletes/redacted-athlete-1").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["first_name"], "");
    assert_eq!(body["last_name"], "Redacted Athlete #1");
    assert_eq!(body["slug"], "redacted-athlete-1");
    assert!(body.get("native_name").is_none(), "{body}");
    assert!(body.get("instagram_handle").is_none(), "{body}");
    assert_eq!(body["profile_picture_url"], Value::Null);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn the_old_page_is_gone(pool: PgPool) {
    import(&pool, meet("meet", vec![entry("Alina", "Riyaz", None)])).await;

    let (found, _) = get(&pool, "/api/v1/athletes/alina-riyaz").await;
    assert_eq!(found, StatusCode::OK);

    redact_in_place(&pool).await;

    let (gone, _) = get(&pool, "/api/v1/athletes/alina-riyaz").await;
    assert_eq!(gone, StatusCode::NOT_FOUND);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn the_result_still_counts(pool: PgPool) {
    import(&pool, meet("meet", vec![entry("Alina", "Riyaz", None)])).await;
    redact_in_place(&pool).await;

    let (status, body) = get(
        &pool,
        "/api/v1/athletes/redacted-athlete-1?include=competitions",
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let competitions = body["competitions"].as_array().unwrap();
    assert_eq!(competitions.len(), 1);

    let (status, rankings) = get(&pool, "/api/v1/rankings").await;
    assert_eq!(status, StatusCode::OK);

    let entries = rankings["data"].as_array().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0]["athlete"]["last_name"], "Redacted Athlete #1");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn no_endpoint_gives_the_name_back(pool: PgPool) {
    import(
        &pool,
        meet("meet", vec![entry("Alina", "Riyaz", Some("Алина Рияз"))]),
    )
    .await;
    redact_in_place(&pool).await;

    for uri in [
        "/api/v1/athletes",
        "/api/v1/athletes?search=Riyaz",
        "/api/v1/rankings",
        "/api/v1/competitions/meet",
    ] {
        let (_, body) = get(&pool, uri).await;
        let text = body.to_string();

        assert!(
            !text.contains("Riyaz"),
            "{uri} still names the athlete: {text}"
        );
        assert!(
            !text.contains("Алина"),
            "{uri} still names the athlete: {text}"
        );
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn unknown_country_keeps_results_and_global_standings(pool: PgPool) {
    let mut athlete = entry("Alina", "Riyaz", None);
    athlete.country = None;
    import(&pool, meet("meet", vec![athlete])).await;
    let (status, body) = get(
        &pool,
        "/api/v1/athletes/alina-riyaz?include=competitions,records,standing",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["country"], Value::Null);
    assert_eq!(body["competitions"].as_array().unwrap().len(), 1);
    assert_eq!(body["personal_records"].as_array().unwrap().len(), 4);
    for metric in body["standing"].as_object().unwrap().values() {
        assert_eq!(metric["global"]["place"], 1);
        assert_eq!(metric["country"], Value::Null);
    }
    let (status, rankings) = get(&pool, "/api/v1/rankings").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(rankings["data"].as_array().unwrap().len(), 1);
    assert_eq!(rankings["data"][0]["athlete"]["country"], Value::Null);
}
