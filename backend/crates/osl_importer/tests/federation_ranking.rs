//! A ranking narrowed to one federation only counts what that federation ran,
//! and it numbers the places within that federation rather than carrying the
//! worldwide ones.

use osl_db::params::{RankingFilter, RankingMovement, SortDirection};
use osl_db::repository::ranking::RankingRepository;
use osl_importer::canonical::models::{AthleteData, CanonicalFormat};
use sqlx::PgPool;

mod common;

use common::{import, lifting, men_80};

fn athlete(first: &str, last: &str, squat: &str) -> AthleteData {
    lifting(common::athlete(first, last), ["50", "90", "100", squat])
}

fn competition(slug: &str, federation: &str, athletes: Vec<AthleteData>) -> CanonicalFormat {
    let mut canonical = common::competition(slug, vec![men_80(athletes)]);
    canonical.competition.federation.name = federation.to_string();
    canonical
}

fn filter(federation: Option<&str>) -> RankingFilter {
    RankingFilter {
        gender: None,
        country: None,
        federation: federation.map(str::to_string),
        name: None,
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
            "fnsl-open",
            "FNSL",
            vec![
                athlete("Jean", "Dupont", "150"),
                athlete("Pierre", "Martin", "130"),
            ],
        ),
    )
    .await;
    import(
        pool,
        competition(
            "finalrep-open",
            "FinalRep",
            vec![athlete("Klaus", "Meyer", "200")],
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
        .map(|row| (row.last_name, row.rank))
        .collect();

    (placings, total)
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn no_federation_ranks_everyone(pool: PgPool) {
    seed(&pool).await;

    let (placings, total) = placings(&pool, &filter(None)).await;

    assert_eq!(total, 3);
    assert_eq!(
        placings,
        vec![
            ("Meyer".to_string(), 1),
            ("Dupont".to_string(), 2),
            ("Martin".to_string(), 3),
        ]
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_federation_ranks_only_its_own_athletes(pool: PgPool) {
    seed(&pool).await;

    let (placings, total) = placings(&pool, &filter(Some("FNSL"))).await;

    assert_eq!(total, 2);
    assert_eq!(
        placings,
        vec![("Dupont".to_string(), 1), ("Martin".to_string(), 2)]
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_federation_nobody_competed_under_ranks_nobody(pool: PgPool) {
    seed(&pool).await;

    let (placings, total) = placings(&pool, &filter(Some("Unknown Federation"))).await;

    assert_eq!(total, 0);
    assert!(placings.is_empty());
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_search_inside_a_federation_keeps_the_federation_place(pool: PgPool) {
    seed(&pool).await;

    let searched = RankingFilter {
        name: Some("martin".to_string()),
        ..filter(Some("FNSL"))
    };
    let (placings, total) = placings(&pool, &searched).await;

    assert_eq!(total, 1);
    assert_eq!(placings, vec![("Martin".to_string(), 2)]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn the_federation_list_holds_only_federations_with_results(pool: PgPool) {
    seed(&pool).await;
    let mut announced = common::announcement("upcoming-open");
    announced.competition.federation.name = "Announced Federation".to_string();
    import(&pool, announced).await;

    let federations = RankingRepository::new(&pool)
        .list_distinct_federations()
        .await
        .expect("listing should succeed");

    assert_eq!(federations, vec!["FinalRep", "FNSL"]);
}
