//! A disqualified result stays public and counts for nothing.
//!
//! It keeps its lifts in the category and takes no place, so the lifters behind
//! it move up, and no board or personal record ever sees it.

use osl_db::params::{RankingFilter, RankingMovement, SortDirection};
use osl_db::repository::{
    athlete::AthleteRepository, competition::CompetitionRepository, ranking::RankingRepository,
};
use osl_importer::canonical::{models::CanonicalFormat, transformer::CanonicalTransformer};
use rust_decimal::Decimal;
use serde_json::{Value, json};
use sqlx::PgPool;

fn lift(movement: &str, weight: i32) -> Value {
    json!({
        "movement": movement,
        "attempts": [{ "attempt_number": 1, "weight": weight, "is_successful": true }],
    })
}

fn athlete(first: &str, last: &str, status: &str, lifts: Vec<Value>) -> Value {
    json!({
        "first_name": first, "last_name": last, "country": "FR",
        "bodyweight": 80, "status": status, "lifts": lifts,
    })
}

fn meet(slug: &str, athletes: Vec<Value>) -> CanonicalFormat {
    serde_json::from_value(json!({
        "format_version": "1.5.0",
        "source": { "type": "manual", "extracted_at": "2026-01-01T00:00:00Z", "extractor": "disqualification-test" },
        "competition": {
            "name": slug, "slug": slug,
            "federation": { "name": "Test Federation" },
            "start_date": "2026-01-01", "end_date": "2026-01-01", "country": "FR",
        },
        "movements": [
            { "name": "Muscle-up", "order": 1 }, { "name": "Pull-up", "order": 2 },
            { "name": "Dips", "order": 3 }, { "name": "Squat", "order": 4 },
        ],
        "categories": [{
            "name": "Men -80kg", "gender": "M", "weight_class_slug": "M-80",
            "athletes": athletes,
        }],
    }))
    .expect("test document should be a valid canonical file")
}

async fn import(pool: &PgPool, canonical: CanonicalFormat) {
    CanonicalTransformer::new(pool)
        .import_to_database(canonical)
        .await
        .expect("import should succeed");
}

async fn board(pool: &PgPool, movement: RankingMovement) -> (Vec<String>, i64) {
    let filter = RankingFilter {
        gender: None,
        country: None,
        movement,
        direction: SortDirection::Desc,
        event: osl_domain::FULL_EVENT.to_string(),
        category: None,
        year: None,
        competition_id: None,
        offset: 0,
        limit: 50,
    };
    let (rows, count) = RankingRepository::new(pool)
        .get_global_ranking(&filter)
        .await
        .unwrap();

    (rows.into_iter().map(|r| r.last_name).collect(), count)
}

/// The biggest total of the day, thrown out.
fn disqualified() -> Value {
    athlete(
        "Dan",
        "Disqualified",
        "disqualified",
        vec![
            lift("Muscle-up", 50),
            lift("Pull-up", 90),
            lift("Dips", 130),
            lift("Squat", 200),
        ],
    )
}

fn clean() -> Value {
    athlete(
        "Clara",
        "Clean",
        "competed",
        vec![
            lift("Muscle-up", 30),
            lift("Pull-up", 80),
            lift("Dips", 120),
            lift("Squat", 180),
        ],
    )
}

/// Printed as three zeros by the source, which meant he never turned up.
fn absent() -> Value {
    athlete("Noe", "Absent", "disqualified", vec![])
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_disqualified_lifter_takes_no_place(pool: PgPool) {
    import(&pool, meet("test-meet", vec![disqualified(), clean()])).await;

    let detail = CompetitionRepository::new(&pool)
        .find_by_slug_detailed("test-meet")
        .await
        .unwrap();

    let places: Vec<(&str, Option<i32>)> = detail.categories[0]
        .participants
        .iter()
        .map(|p| (p.athlete.last_name.as_str(), p.rank))
        .collect();

    assert_eq!(
        places,
        vec![("Clean", Some(1)), ("Disqualified", None)],
        "the biggest total was thrown out, so the win belongs to the lifter behind it"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_disqualified_result_reaches_no_board(pool: PgPool) {
    import(&pool, meet("test-meet", vec![disqualified(), clean()])).await;

    let (names, count) = board(&pool, RankingMovement::Total).await;
    assert_eq!(names, vec!["Clean"], "a thrown-out total is not a total");
    assert_eq!(count, 1, "the count has to agree with the rows");

    let (names, _) = board(&pool, RankingMovement::Squat).await;
    assert_eq!(
        names,
        vec!["Clean"],
        "the single movement boards drop it too"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_disqualified_lift_is_not_a_personal_record(pool: PgPool) {
    import(&pool, meet("test-meet", vec![disqualified()])).await;

    let detail = AthleteRepository::new(&pool)
        .find_by_slug_detailed("dan-disqualified")
        .await
        .unwrap();

    assert!(
        detail.personal_records.is_empty(),
        "his 200 kg squat happened, but a disqualified meet sets no record"
    );
    assert_eq!(
        detail.competitions.len(),
        1,
        "the result itself stays on his page"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn somebody_who_never_lifted_has_no_total(pool: PgPool) {
    import(&pool, meet("test-meet", vec![clean(), absent()])).await;

    let detail = CompetitionRepository::new(&pool)
        .find_by_slug_detailed("test-meet")
        .await
        .unwrap();

    let totals: Vec<(&str, Option<Decimal>)> = detail.categories[0]
        .participants
        .iter()
        .map(|p| (p.athlete.last_name.as_str(), p.total))
        .collect();

    assert_eq!(
        totals,
        vec![("Clean", Some(Decimal::from(410))), ("Absent", None)],
        "no lifts at all is an absence, which must not read as a total of zero"
    );
}
