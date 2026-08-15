use osl_db::params::{RankingFilter, RankingMovement, SortDirection};
use osl_db::repository::ranking::RankingRepository;
use osl_importer::canonical::{models::CanonicalFormat, transformer::CanonicalTransformer};
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

fn lift(movement: &str, weight: i32) -> Value {
    json!({
        "movement": movement,
        "attempts": [{ "attempt_number": 1, "weight": weight, "is_successful": true }],
    })
}

fn athlete(first: &str, last: &str) -> Value {
    json!({
        "first_name": first,
        "last_name": last,
        "country": "FR",
        "bodyweight": 80,
        "status": "competed",
        "lifts": [
            lift("Muscle-up", 50),
            lift("Pull-up", 90),
            lift("Dips", 100),
            lift("Squat", 150),
        ],
    })
}

fn meet(slug: &str, athletes: Vec<Value>) -> CanonicalFormat {
    let movements: Vec<Value> = ["Muscle-up", "Pull-up", "Dips", "Squat"]
        .iter()
        .enumerate()
        .map(|(index, name)| json!({ "name": name, "order": index as i16 + 1 }))
        .collect();

    let document = json!({
        "format_version": "1.5.0",
        "source": {
            "type": "manual",
            "extracted_at": "2026-01-01T00:00:00Z",
            "extractor": "athlete-name-search-test",
        },
        "competition": {
            "name": slug,
            "slug": slug,
            "federation": { "name": "Test Federation" },
            "start_date": "2026-01-01",
            "end_date": "2026-01-01",
            "country": "FR",
        },
        "movements": movements,
        "categories": [{
            "name": "Men -80kg",
            "gender": "M",
            "weight_class_slug": "M-80",
            "athletes": athletes,
        }],
    });

    serde_json::from_value(document).expect("test document should be a valid canonical file")
}

fn filter(name: Option<&str>) -> RankingFilter {
    RankingFilter {
        gender: None,
        country: None,
        name: name.map(str::to_string),
        movement: RankingMovement::Total,
        direction: SortDirection::Desc,
        event: osl_domain::FULL_EVENT.to_string(),
        category: None,
        year: None,
        competition_id: None,
        offset: 0,
        limit: 50,
    }
}

async fn seed(pool: &PgPool) {
    CanonicalTransformer::new(pool)
        .import_to_database(meet(
            "search-meet",
            vec![
                athlete("Jean", "Dupont"),
                athlete("Marie", "Dupont"),
                athlete("Pierre", "Martin"),
            ],
        ))
        .await
        .expect("import should succeed");
}

async fn search(pool: &PgPool, filter: &RankingFilter) -> (Vec<String>, i64) {
    let (rows, total) = RankingRepository::new(pool)
        .get_global_ranking(filter)
        .await
        .expect("ranking should succeed");

    let names = rows
        .into_iter()
        .map(|row| format!("{} {}", row.first_name, row.last_name))
        .collect();

    (names, total)
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn no_search_returns_everyone(pool: PgPool) {
    seed(&pool).await;

    let (names, total) = search(&pool, &filter(None)).await;

    assert_eq!(total, 3);
    assert_eq!(names.len(), 3);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_last_name_matches_regardless_of_case(pool: PgPool) {
    seed(&pool).await;

    let (names, total) = search(&pool, &filter(Some("DUPONT"))).await;

    assert_eq!(total, 2);
    assert!(names.iter().all(|name| name.ends_with("Dupont")));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_first_name_matches_too(pool: PgPool) {
    seed(&pool).await;

    let (names, _) = search(&pool, &filter(Some("pierre"))).await;

    assert_eq!(names, vec!["Pierre Martin"]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_full_name_matches_across_both_columns(pool: PgPool) {
    seed(&pool).await;

    let (names, _) = search(&pool, &filter(Some("jean dup"))).await;

    assert_eq!(names, vec!["Jean Dupont"]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_name_that_matches_nobody_returns_nothing(pool: PgPool) {
    seed(&pool).await;

    let (names, total) = search(&pool, &filter(Some("nobody"))).await;

    assert_eq!(total, 0);
    assert!(names.is_empty());
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn the_total_counts_the_matches_and_not_the_whole_ranking(pool: PgPool) {
    seed(&pool).await;

    let paged = RankingFilter {
        limit: 1,
        ..filter(Some("dupont"))
    };
    let (names, total) = search(&pool, &paged).await;

    assert_eq!(total, 2);
    assert_eq!(names.len(), 1);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn search_composes_with_the_other_filters(pool: PgPool) {
    seed(&pool).await;

    let unknown_competition = RankingFilter {
        competition_id: Some(Uuid::nil()),
        ..filter(Some("dupont"))
    };
    let (_, total) = search(&pool, &unknown_competition).await;

    assert_eq!(total, 0);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn like_wildcards_are_matched_literally(pool: PgPool) {
    seed(&pool).await;

    for wildcard in ["%", "_", "%dupont%"] {
        let (_, total) = search(&pool, &filter(Some(wildcard))).await;
        assert_eq!(total, 0, "{wildcard} should match nobody");
    }
}
