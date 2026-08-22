//! A disqualified result stays public and counts for nothing.
//!
//! It keeps its lifts in the category and takes no place, so the lifters behind
//! it move up, and no board or personal record ever sees it.

use osl_db::params::{RankingFilter, RankingMovement, SortDirection};
use osl_db::repository::{
    athlete::AthleteRepository, competition::CompetitionRepository, ranking::RankingRepository,
};
use osl_importer::canonical::models::AthleteData;
use rust_decimal::Decimal;
use sqlx::PgPool;

mod common;

use common::{athlete, disqualified as thrown_out, import, lifting, meet, men_80};

async fn board(pool: &PgPool, movement: RankingMovement) -> (Vec<String>, i64) {
    let filter = RankingFilter {
        gender: None,
        country: None,
        name: None,
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
fn disqualified() -> AthleteData {
    thrown_out(
        lifting(athlete("Dan", "Disqualified"), ["50", "90", "130", "200"]),
        None,
    )
}

fn clean() -> AthleteData {
    lifting(athlete("Clara", "Clean"), ["30", "80", "120", "180"])
}

/// Printed as three zeros by the source, which meant he never turned up.
fn absent() -> AthleteData {
    thrown_out(athlete("Noe", "Absent"), None)
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_disqualified_lifter_takes_no_place(pool: PgPool) {
    import(
        &pool,
        meet("test-meet", vec![men_80(vec![disqualified(), clean()])]),
    )
    .await;

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
    import(
        &pool,
        meet("test-meet", vec![men_80(vec![disqualified(), clean()])]),
    )
    .await;

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
    import(&pool, meet("test-meet", vec![men_80(vec![disqualified()])])).await;

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
    import(
        &pool,
        meet("test-meet", vec![men_80(vec![clean(), absent()])]),
    )
    .await;

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
