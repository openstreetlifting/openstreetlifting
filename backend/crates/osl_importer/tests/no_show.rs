//! Someone who entered and never lifted is not the same as someone who lifted
//! and was ruled out, and a boolean could not tell them apart.
//!
//! Both stay out of the rankings. Both stay in the competition, because the
//! federation's own results list them.

use osl_db::params::{RankingFilter, RankingMovement, SortDirection};
use osl_db::repository::ranking::RankingRepository;
use sqlx::PgPool;

mod common;

use common::{athlete, competition, disqualified, import, lifting, men_80, no_show};

async fn statuses(pool: &PgPool, slug: &str) -> Vec<(String, String)> {
    sqlx::query_as(
        "SELECT a.last_name, cp.status
         FROM competition_participants cp
         JOIN athletes a USING (athlete_id)
         JOIN competitions c USING (competition_id)
         WHERE c.slug = $1
         ORDER BY a.last_name",
    )
    .bind(slug)
    .fetch_all(pool)
    .await
    .unwrap()
}

fn entered() -> osl_importer::canonical::models::CanonicalFormat {
    competition(
        "entered",
        vec![men_80(vec![
            lifting(athlete("Ana", "Placed"), ["20", "80", "120", "200"]),
            disqualified(
                lifting(athlete("Cy", "Bombed"), ["15", "70", "110", "180"]),
                Some("No successful squat"),
            ),
            no_show(athlete("Dee", "Absent"), None),
        ])],
    )
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn each_outcome_keeps_its_own_status(pool: PgPool) {
    import(&pool, entered()).await;

    assert_eq!(
        statuses(&pool, "entered").await,
        vec![
            ("Absent".to_string(), "no_show".to_string()),
            ("Bombed".to_string(), "disqualified".to_string()),
            ("Placed".to_string(), "competed".to_string()),
        ]
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn only_the_lifter_who_competed_is_ranked(pool: PgPool) {
    import(&pool, entered()).await;

    let (ranked, total) = RankingRepository::new(&pool)
        .get_global_ranking(&RankingFilter {
            gender: None,
            country: None,
            name: None,
            movement: RankingMovement::Total,
            direction: SortDirection::Desc,
            event: osl_domain::FULL_EVENT.to_string(),
            category: None,
            year: None,
            competition_id: None,
            offset: 0,
            limit: 50,
        })
        .await
        .unwrap();

    assert_eq!(total, 1);
    assert_eq!(ranked[0].last_name, "Placed");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn the_competition_still_lists_everyone(pool: PgPool) {
    import(&pool, entered()).await;

    let listed: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM competition_participants cp
         JOIN competitions c USING (competition_id)
         WHERE c.slug = 'entered'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(
        listed, 3,
        "a competition records who turned up, not who placed"
    );
}
