//! Most of the archive was lifted before RIS existed, so the date lookup has to
//! answer for those meets rather than refuse them.

use chrono::NaiveDate;
use osl_db::repository::ris::RisRepository;
use sqlx::PgPool;

fn date(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

async fn insert_2027_formula(pool: &PgPool) {
    sqlx::query(
        "INSERT INTO ris_formula_versions (
             year, effective_from, is_current,
             men_a, men_k, men_b, men_v, men_q,
             women_a, women_k, women_b, women_v, women_q
         ) VALUES (2027, '2027-01-01', false, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1)",
    )
    .execute(pool)
    .await
    .unwrap();
}

#[sqlx::test(migrations = "./migrations")]
async fn a_meet_inside_a_formula_window_uses_that_formula(pool: PgPool) {
    insert_2027_formula(&pool).await;
    let formula = RisRepository::new(&pool)
        .get_formula_for_date(date(2027, 6, 1))
        .await
        .unwrap();

    assert_eq!(formula.year, 2027);
}

#[sqlx::test(migrations = "./migrations")]
async fn a_meet_older_than_every_formula_falls_back_to_the_earliest(pool: PgPool) {
    insert_2027_formula(&pool).await;
    let formula = RisRepository::new(&pool)
        .get_formula_for_date(date(2022, 8, 20))
        .await
        .unwrap();

    assert_eq!(formula.year, 2025);
}
