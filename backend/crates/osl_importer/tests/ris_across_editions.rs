//! Most of the archive was lifted before RIS existed, and a ranking is only
//! readable if every score in it came from the same formula. So the competition's date
//! decides nothing: the current edition scores all of them.

use chrono::NaiveDate;
use osl_domain::Movement;
use osl_importer::canonical::models::{AthleteData, CanonicalFormat};
use rust_decimal::Decimal;
use sqlx::PgPool;

mod common;

use common::{best, import, men_80};

fn lifter() -> AthleteData {
    let mut athlete = common::athlete("Gianluca", "Macchia");
    for (movement, weight) in [
        (Movement::MuscleUp, "20"),
        (Movement::PullUp, "80"),
        (Movement::Dips, "120"),
        (Movement::Squat, "200"),
    ] {
        athlete = best(athlete, movement, weight);
    }
    athlete
}

fn competition_held_in(year: i32, slug: &str) -> CanonicalFormat {
    let mut canonical = common::competition(slug, vec![men_80(vec![lifter()])]);
    let day = NaiveDate::from_ymd_opt(year, 8, 20).unwrap();
    canonical.competition.start_date = day;
    canonical.competition.end_date = day;
    canonical
}

async fn scored_with(pool: &PgPool, slug: &str) -> (Decimal, i32) {
    sqlx::query_as(
        "SELECT cp.ris_score, cp.ris_edition
         FROM competition_participants cp
         JOIN competitions c USING (competition_id)
         WHERE c.slug = $1",
    )
    .bind(slug)
    .fetch_one(pool)
    .await
    .expect("the competition should have been scored")
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_meet_older_than_the_formula_is_still_scored(pool: PgPool) {
    import(&pool, competition_held_in(2022, "old-competition")).await;

    let (score, year) = scored_with(&pool, "old-competition").await;

    assert_eq!(year, 2026);
    assert!(score > Decimal::ZERO);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn meets_from_different_years_share_one_formula(pool: PgPool) {
    import(&pool, competition_held_in(2022, "old-competition")).await;
    import(&pool, competition_held_in(2026, "recent-competition")).await;

    assert_eq!(
        scored_with(&pool, "old-competition").await,
        scored_with(&pool, "recent-competition").await,
        "the same lifts at the same bodyweight must score the same in any year"
    );
}
