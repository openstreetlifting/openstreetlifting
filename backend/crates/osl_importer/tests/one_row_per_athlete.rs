//! A ranking lists people, so a lifter who has competed three times is one of
//! them, shown at the competition where they were at their best.
//!
//! Which competition that is depends on the column being ranked: a lifter whose
//! best muscle-up and best total came from different years is a different row on
//! each board.

use osl_db::params::{RankingFilter, RankingMovement, SortDirection};
use osl_db::repository::ranking::RankingRepository;
use sqlx::PgPool;
use uuid::Uuid;

mod common;

use common::{athlete, competition, import, lifting, men_80};

fn filter(movement: RankingMovement, competition_id: Option<Uuid>) -> RankingFilter {
    RankingFilter {
        gender: None,
        country: None,
        name: None,
        movement,
        direction: SortDirection::Desc,
        event: osl_domain::FULL_EVENT.to_string(),
        category: None,
        year: None,
        competition_id,
        offset: 0,
        limit: 50,
    }
}

/// The same lifter twice: a bigger muscle-up in the first year, a bigger
/// everything-else in the second.
async fn two_years(pool: &PgPool) {
    import(
        pool,
        competition(
            "first-year",
            vec![men_80(vec![lifting(
                athlete("Ada", "Twice"),
                ["40", "60", "100", "150"],
            )])],
        ),
    )
    .await;
    import(
        pool,
        competition(
            "second-year",
            vec![men_80(vec![lifting(
                athlete("Ada", "Twice"),
                ["20", "80", "120", "200"],
            )])],
        ),
    )
    .await;
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_lifter_holds_one_place(pool: PgPool) {
    two_years(&pool).await;

    let (rows, total) = RankingRepository::new(&pool)
        .get_global_ranking(&filter(RankingMovement::Total, None))
        .await
        .unwrap();

    assert_eq!(total, 1, "two results, one athlete, one place");
    assert_eq!(rows.len(), 1);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn the_place_shown_is_their_best_on_the_ranked_column(pool: PgPool) {
    two_years(&pool).await;
    let repo = RankingRepository::new(&pool);

    let (by_total, _) = repo
        .get_global_ranking(&filter(RankingMovement::Total, None))
        .await
        .unwrap();
    assert_eq!(by_total[0].competition_slug, "second-year");

    let (by_muscleup, _) = repo
        .get_global_ranking(&filter(RankingMovement::Muscleup, None))
        .await
        .unwrap();
    assert_eq!(
        by_muscleup[0].competition_slug, "first-year",
        "their bigger muscle-up came from the year with the smaller total"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_competition_still_lists_every_entry(pool: PgPool) {
    two_years(&pool).await;

    let id: Uuid = sqlx::query_scalar("SELECT competition_id FROM competitions WHERE slug = $1")
        .bind("first-year")
        .fetch_one(&pool)
        .await
        .unwrap();

    let (_, total) = RankingRepository::new(&pool)
        .get_global_ranking(&filter(RankingMovement::Total, Some(id)))
        .await
        .unwrap();

    assert_eq!(
        total, 1,
        "scoped to one competition, nothing is deduplicated"
    );
}
