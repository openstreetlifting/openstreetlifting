use osl_db::params::{RankingFilter, RankingMovement, SortDirection};
use osl_db::repository::ranking::RankingRepository;
use osl_importer::canonical::models::{AthleteData, CanonicalFormat};
use sqlx::PgPool;
use uuid::Uuid;

mod common;

use common::{from, import, lifting, men_80};

fn athlete(first: &str, last: &str, squat: &str) -> AthleteData {
    lifting(common::athlete(first, last), ["50", "90", "100", squat])
}

fn competition(slug: &str, athletes: Vec<AthleteData>) -> CanonicalFormat {
    common::competition(slug, vec![men_80(athletes)])
}

fn filter(name: Option<&str>) -> RankingFilter {
    RankingFilter {
        gender: None,
        country: None,
        federation: None,
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
    import(
        pool,
        competition(
            "search-competition",
            vec![
                athlete("Jean", "Dupont", "150"),
                athlete("Marie", "Dupont", "130"),
                athlete("Pierre", "Martin", "140"),
            ],
        ),
    )
    .await;
}

async fn placings(pool: &PgPool, filter: &RankingFilter) -> (Vec<(String, i64)>, i64) {
    let (rows, total) = RankingRepository::new(pool)
        .get_global_ranking(filter)
        .await
        .expect("ranking should succeed");

    let placings = rows
        .into_iter()
        .map(|row| (format!("{} {}", row.first_name, row.last_name), row.rank))
        .collect();

    (placings, total)
}

async fn search(pool: &PgPool, filter: &RankingFilter) -> (Vec<String>, i64) {
    let (placings, total) = placings(pool, filter).await;

    (placings.into_iter().map(|(name, _)| name).collect(), total)
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn returns_all_athletes_when_search_is_absent(pool: PgPool) {
    seed(&pool).await;

    let (names, total) = search(&pool, &filter(None)).await;

    assert_eq!(total, 3);
    assert_eq!(names.len(), 3);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn matches_last_names_case_insensitively(pool: PgPool) {
    seed(&pool).await;

    let (names, total) = search(&pool, &filter(Some("DUPONT"))).await;

    assert_eq!(total, 2);
    assert!(names.iter().all(|name| name.ends_with("Dupont")));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn matches_first_names(pool: PgPool) {
    seed(&pool).await;

    let (names, _) = search(&pool, &filter(Some("pierre"))).await;

    assert_eq!(names, vec!["Pierre Martin"]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn matches_full_names(pool: PgPool) {
    seed(&pool).await;

    let (names, _) = search(&pool, &filter(Some("jean dup"))).await;

    assert_eq!(names, vec!["Jean Dupont"]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn returns_empty_results_when_name_is_unknown(pool: PgPool) {
    seed(&pool).await;

    let (names, total) = search(&pool, &filter(Some("nobody"))).await;

    assert_eq!(total, 0);
    assert_eq!(names, [] as [String; 0]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn counts_matching_athletes(pool: PgPool) {
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
async fn combines_search_with_ranking_filters(pool: PgPool) {
    seed(&pool).await;

    let unknown_competition = RankingFilter {
        competition_id: Some(Uuid::nil()),
        ..filter(Some("dupont"))
    };
    let (_, total) = search(&pool, &unknown_competition).await;

    assert_eq!(total, 0);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn matches_wildcards_literally(pool: PgPool) {
    seed(&pool).await;

    for wildcard in ["%", "_", "%dupont%"] {
        let (_, total) = search(&pool, &filter(Some(wildcard))).await;
        assert_eq!(total, 0, "{wildcard} should match nobody");
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn preserves_rank_when_searching(pool: PgPool) {
    seed(&pool).await;

    let (placings, _) = placings(&pool, &filter(Some("pierre"))).await;

    assert_eq!(placings, vec![("Pierre Martin".to_string(), 2)]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn preserves_ranks_when_search_has_multiple_matches(pool: PgPool) {
    seed(&pool).await;

    let (placings, _) = placings(&pool, &filter(Some("dupont"))).await;

    assert_eq!(
        placings,
        vec![
            ("Jean Dupont".to_string(), 1),
            ("Marie Dupont".to_string(), 3),
        ]
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn applies_ranking_filters_before_search(pool: PgPool) {
    seed(&pool).await;
    import(
        &pool,
        competition(
            "german-competition",
            vec![from(athlete("Klaus", "Meyer", "200"), "DE")],
        ),
    )
    .await;

    let (worldwide, _) = placings(&pool, &filter(Some("jean"))).await;

    assert_eq!(worldwide, vec![("Jean Dupont".to_string(), 2)]);

    let french = RankingFilter {
        country: Some("FR".to_string()),
        ..filter(Some("jean"))
    };
    let (french, _) = placings(&pool, &french).await;

    assert_eq!(french, vec![("Jean Dupont".to_string(), 1)]);
}
