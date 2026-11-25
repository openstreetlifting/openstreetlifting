use osl_db::filter::Filters;
use osl_db::params::{RankingFilter, RankingMovement, SortDirection};
use osl_db::repository::ranking::RankingRepository;
use sqlx::PgPool;

mod common;

use common::{import, lifting, men_80};

fn filter(movement: RankingMovement) -> RankingFilter {
    RankingFilter {
        by: Filters::default(),
        name: None,
        movement,
        direction: SortDirection::Desc,
        event: Some(osl_domain::FULL_EVENT.to_string()),
        offset: 0,
        limit: 50,
    }
}

async fn classic_rank(pool: &PgPool) -> (Vec<String>, Vec<Option<String>>, i64) {
    let (rows, total) = RankingRepository::new(pool)
        .get_global_ranking(&filter(RankingMovement::ClassicTotal))
        .await
        .expect("ranking should succeed");

    (
        rows.iter().map(|row| row.last_name.clone()).collect(),
        rows.iter().map(|row| row.event_code.clone()).collect(),
        total,
    )
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn ranks_classic_totals_and_keeps_mpds_off_the_board(pool: PgPool) {
    import(
        &pool,
        common::classic_competition(
            "polish-classic",
            vec![
                common::classic_athlete("Jan", "Kowalski", "60", "100"),
                common::classic_athlete("Adam", "Nowak", "50", "90"),
            ],
        ),
    )
    .await;
    import(
        &pool,
        common::competition(
            "mpds-meet",
            vec![men_80(vec![lifting(
                common::athlete("Ada", "Mpds"),
                ["200", "90", "100", "150"],
            )])],
        ),
    )
    .await;

    let (names, events, total) = classic_rank(&pool).await;

    assert_eq!(names, vec!["Kowalski", "Nowak"]);
    assert_eq!(events, vec![Some("PD".to_string()); 2]);
    assert_eq!(
        total, 2,
        "a 540 kg MPDS total has no place on the Classic board"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn classic_total_ignores_a_conflicting_event_filter(pool: PgPool) {
    import(
        &pool,
        common::classic_competition(
            "polish-classic",
            vec![common::classic_athlete("Jan", "Kowalski", "60", "100")],
        ),
    )
    .await;

    let mut scoped = filter(RankingMovement::ClassicTotal);
    scoped.event = Some(osl_domain::FULL_EVENT.to_string());

    let (rows, total) = RankingRepository::new(&pool)
        .get_global_ranking(&scoped)
        .await
        .expect("ranking should succeed");

    assert_eq!(total, 1, "Classic always means the PD event");
    assert_eq!(rows[0].last_name, "Kowalski");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn breaks_classic_total_ties_by_lower_bodyweight(pool: PgPool) {
    import(
        &pool,
        common::classic_competition(
            "tied-classic",
            vec![
                common::weighing(common::classic_athlete("Ada", "Heavier", "60", "90"), "79"),
                common::weighing(
                    common::classic_athlete("Grace", "Lighter", "60", "90"),
                    "75",
                ),
            ],
        ),
    )
    .await;

    let (names, _, _) = classic_rank(&pool).await;

    assert_eq!(names, vec!["Lighter", "Heavier"]);
}
