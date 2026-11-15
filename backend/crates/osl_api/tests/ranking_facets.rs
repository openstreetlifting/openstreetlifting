use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use osl_api::{AppState, app};
use osl_db::Database;
use osl_domain::{Gender, Movement};
use osl_importer::canonical::models::CanonicalFormat;
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;

#[path = "../../osl_importer/tests/common/mod.rs"]
mod common;

async fn facets(pool: &PgPool, query: &str) -> (StatusCode, Value) {
    get(pool, &format!("/api/v1/rankings/facets{query}")).await
}

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

fn pull_and_dips_meet() -> CanonicalFormat {
    let athlete = common::from(common::athlete("Sam", "Keller"), "DE");
    let athlete = common::best(athlete, Movement::PullUp, "60");
    let athlete = common::best(athlete, Movement::Dips, "90");
    let mut meet = common::competition(
        "berlin-pull-dips",
        vec![common::open_category(Gender::M, "62.5", vec![athlete])],
    );
    meet.movements = vec![Movement::PullUp, Movement::Dips];
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
    let (_, searched) = facets(&pool, "?country=FR&q=nobody&direction=asc&page=9").await;
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

    let (status, lenient) = facets(&pool, "?category=nonsense&page=0").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(lenient["countries"], json!(["FR", "IT"]));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn offers_only_options_ranked_on_the_metric(pool: PgPool) {
    two_meets(&pool).await;
    common::import(&pool, pull_and_dips_meet()).await;

    let (_, ris) = facets(&pool, "").await;
    assert_eq!(ris["classes"], json!(["-80kg"]));
    assert_eq!(ris["countries"], json!(["FR", "IT"]));

    let (_, total) = facets(&pool, "?movement=total").await;
    assert_eq!(total["classes"], json!(["-80kg"]));

    let (_, pullup) = facets(&pool, "?movement=pullup").await;
    assert_eq!(pullup["classes"], json!(["+62.5kg", "-80kg"]));
    assert_eq!(pullup["countries"], json!(["DE", "FR", "IT"]));

    let (_, squat) = facets(&pool, "?movement=squat").await;
    assert_eq!(squat["classes"], json!(["-80kg"]));

    let (_, short_event) = facets(&pool, "?movement=total&event=PD").await;
    assert_eq!(short_event["classes"], json!(["+62.5kg"]));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn every_offered_class_has_ranked_athletes(pool: PgPool) {
    two_meets(&pool).await;
    common::import(&pool, pull_and_dips_meet()).await;

    for movement in ["ris", "total", "muscleup", "pullup", "dips", "squat"] {
        let (_, offered) = facets(&pool, &format!("?movement={movement}")).await;
        let classes = offered["classes"].as_array().unwrap();
        assert!(!classes.is_empty(), "{movement} offers no class");

        for class in classes {
            let category = class.as_str().unwrap().replace('+', "%2B");
            let (status, ranking) = get(
                &pool,
                &format!("/api/v1/rankings?movement={movement}&category={category}"),
            )
            .await;
            assert_eq!(status, StatusCode::OK);
            assert_ne!(
                ranking["pagination"]["total_items"],
                json!(0),
                "{movement} offers {class} with nobody ranked in it"
            );
        }
    }
}
