//! A total is only a total within one event, so a muscle-up-only result must
//! never be ranked against a four-lift total. A single movement is different:
//! a muscle-up is a muscle-up whatever else the meet ran, so those boards span
//! every event.

use osl_db::params::{RankingFilter, RankingMovement, SortDirection};
use osl_db::repository::ranking::RankingRepository;
use osl_domain::Movement;
use osl_importer::canonical::models::{AthleteData, CanonicalFormat};
use sqlx::PgPool;

mod common;

use common::{attempts, import, lifting, men_80};

fn athlete(first: &str, last: &str, lifts: Vec<(Movement, &str)>) -> AthleteData {
    lifts.into_iter().fold(
        common::athlete(first, last),
        |lifter, (movement, weight)| attempts(lifter, movement, &[(weight, true)]),
    )
}

/// A competition contesting exactly the movements given.
fn meet(slug: &str, movements: &[Movement], athletes: Vec<AthleteData>) -> CanonicalFormat {
    let mut canonical = common::meet(slug, vec![men_80(athletes)]);
    canonical.movements = movements.to_vec();
    canonical
}

const ALL_FOUR: &[Movement] = &Movement::ALL;

fn four_lift_athlete(first: &str, last: &str, muscleup: &str) -> AthleteData {
    lifting(common::athlete(first, last), [muscleup, "90", "100", "150"])
}

fn filter(movement: RankingMovement) -> RankingFilter {
    RankingFilter {
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
    }
}

async fn rank(pool: &PgPool, movement: RankingMovement) -> (Vec<String>, i64) {
    let (rows, total) = RankingRepository::new(pool)
        .get_global_ranking(&filter(movement))
        .await
        .expect("ranking should succeed");

    (rows.into_iter().map(|row| row.last_name).collect(), total)
}

/// One four-lift meet and one muscle-up-only meet.
async fn seed_both_events(pool: &PgPool) {
    import(
        pool,
        meet(
            "four-lift-meet",
            ALL_FOUR,
            vec![four_lift_athlete("Ada", "Fourlift", "50")],
        ),
    )
    .await;

    import(
        pool,
        meet(
            "muscle-up-only",
            &[Movement::MuscleUp],
            vec![athlete(
                "Grace",
                "Specialist",
                vec![(Movement::MuscleUp, "70")],
            )],
        ),
    )
    .await;
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn the_event_records_which_movements_were_contested(pool: PgPool) {
    seed_both_events(&pool).await;

    let events: Vec<(String, Option<String>)> =
        sqlx::query_as("SELECT slug, event_code FROM competitions ORDER BY slug")
            .fetch_all(&pool)
            .await
            .unwrap();

    assert_eq!(
        events,
        vec![
            ("four-lift-meet".to_string(), Some("MPDS".to_string())),
            ("muscle-up-only".to_string(), Some("M".to_string())),
        ]
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_partial_event_stays_out_of_the_total_ranking(pool: PgPool) {
    seed_both_events(&pool).await;

    let (names, total) = rank(&pool, RankingMovement::Total).await;

    assert_eq!(names, vec!["Fourlift"], "only the four-lift total ranks");
    assert_eq!(total, 1, "the count must match the rows, not overcount");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_single_movement_ranking_spans_every_event(pool: PgPool) {
    seed_both_events(&pool).await;

    let (names, total) = rank(&pool, RankingMovement::Muscleup).await;

    // The specialist lifted more, so they lead a board they belong on.
    assert_eq!(names, vec!["Specialist", "Fourlift"]);
    assert_eq!(total, 2);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn an_uncontested_movement_is_absent_rather_than_zero(pool: PgPool) {
    seed_both_events(&pool).await;

    let (names, total) = rank(&pool, RankingMovement::Squat).await;

    assert_eq!(
        names,
        vec!["Fourlift"],
        "the muscle-up specialist never squatted, so they are excluded rather than ranked at zero"
    );
    assert_eq!(total, 1);

    let (rows, _) = RankingRepository::new(&pool)
        .get_global_ranking(&filter(RankingMovement::Muscleup))
        .await
        .unwrap();
    let specialist = rows
        .iter()
        .find(|row| row.last_name == "Specialist")
        .unwrap();
    assert!(specialist.squat.is_none(), "absent, not zero");
    assert!(specialist.muscleup.is_some());
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn no_ris_is_computed_outside_the_full_event(pool: PgPool) {
    seed_both_events(&pool).await;

    let scores: Vec<(String, Option<rust_decimal::Decimal>)> = sqlx::query_as(
        "SELECT c.slug, cp.ris_score
         FROM competition_participants cp
         JOIN competitions c USING (competition_id)
         ORDER BY c.slug",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert!(scores[0].1.is_some(), "the four-lift meet is scored");
    assert!(
        scores[1].1.is_none(),
        "a one-movement total measured against a four-lift benchmark would be meaningless"
    );

    let history: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ris_scores_history")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(history, 1, "and nothing is written to the history either");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_three_movement_meet_is_its_own_event(pool: PgPool) {
    import(
        &pool,
        meet(
            "three-lift-meet",
            &[Movement::MuscleUp, Movement::PullUp, Movement::Dips],
            vec![athlete(
                "Alan",
                "Threelift",
                vec![
                    (Movement::MuscleUp, "60"),
                    (Movement::PullUp, "95"),
                    (Movement::Dips, "110"),
                ],
            )],
        ),
    )
    .await;

    let event: Option<String> = sqlx::query_scalar("SELECT event_code FROM competitions")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(event, Some("MPD".to_string()));

    let (names, _) = rank(&pool, RankingMovement::Total).await;
    assert!(names.is_empty(), "265 kg over three lifts is not a total");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn dropping_a_movement_from_the_file_changes_the_event(pool: PgPool) {
    import(
        &pool,
        meet(
            "shrinking-meet",
            ALL_FOUR,
            vec![four_lift_athlete("Ada", "Fourlift", "50")],
        ),
    )
    .await;

    // Corrected: the squat never happened.
    import(
        &pool,
        meet(
            "shrinking-meet",
            &[Movement::MuscleUp, Movement::PullUp, Movement::Dips],
            vec![athlete(
                "Ada",
                "Fourlift",
                vec![
                    (Movement::MuscleUp, "50"),
                    (Movement::PullUp, "90"),
                    (Movement::Dips, "100"),
                ],
            )],
        ),
    )
    .await;

    let event: Option<String> = sqlx::query_scalar("SELECT event_code FROM competitions")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(event, Some("MPD".to_string()), "the event follows the file");

    let (names, _) = rank(&pool, RankingMovement::Total).await;
    assert!(names.is_empty(), "and it leaves the four-lift board");
}
