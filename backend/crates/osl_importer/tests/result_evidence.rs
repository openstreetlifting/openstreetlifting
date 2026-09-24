use osl_domain::{AthleteStatus, Movement};
use osl_importer::canonical::{
    models::{AttemptData, LiftData},
    validator::CanonicalValidator,
};
use sqlx::PgPool;

mod common;

#[test]
fn disqualification_does_not_allow_invalid_lift_evidence() {
    for (number, weight, best) in [
        (0, "10", None),
        (4, "10", None),
        (1, "-10", None),
        (1, "10", Some("20")),
    ] {
        let mut athlete = common::disqualified(common::athlete("Invalid", "Evidence"), None);
        athlete.lifts.push(LiftData {
            movement: Movement::MuscleUp,
            attempts: Some(vec![AttemptData {
                attempt_number: number,
                weight: common::decimal(weight),
                is_successful: true,
            }]),
            best_lift: best.map(common::decimal),
        });
        assert!(
            CanonicalValidator::validate(&common::competition(
                "invalid",
                vec![common::men_80(vec![athlete])]
            ))
            .is_err()
        );
    }
}

#[test]
fn a_no_show_may_keep_a_published_zero_but_cannot_have_a_positive_score() {
    for (score, valid) in [(None, true), (Some("0"), true), (Some("1"), false)] {
        let mut athlete = common::no_show(common::athlete("Absent", "Lifter"), None);
        athlete.reported_ris = score.map(common::decimal);
        assert_eq!(
            CanonicalValidator::validate(&common::competition(
                "no-show",
                vec![common::men_80(vec![athlete])]
            ))
            .is_ok(),
            valid
        );
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn partial_results_keep_reported_ris_even_when_bodyweight_is_known(pool: PgPool) {
    let mut athlete = common::athlete("Partial", "Result");
    athlete.reported_ris = Some(common::decimal("72.3"));
    athlete = common::best(athlete, Movement::PullUp, "60");
    common::import(
        &pool,
        common::competition("partial", vec![common::men_80(vec![athlete])]),
    )
    .await;
    let result: (
        Option<rust_decimal::Decimal>,
        Option<rust_decimal::Decimal>,
        Option<String>,
    ) = sqlx::query_as("SELECT total, ris_score, ris_source FROM competition_participants")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        result,
        (None, Some(common::decimal("72.3")), Some("reported".into()))
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn disqualified_published_scores_are_preserved_without_a_ranking_score(pool: PgPool) {
    let mut athlete = common::athlete("Disqualified", "Result");
    athlete.bodyweight = None;
    athlete.status = AthleteStatus::Disqualified;
    athlete.reported_ris = Some(common::decimal("42"));
    common::import(
        &pool,
        common::competition("disqualified", vec![common::men_80(vec![athlete])]),
    )
    .await;
    let result: (Option<rust_decimal::Decimal>, Option<rust_decimal::Decimal>) =
        sqlx::query_as("SELECT reported_ris_score, ris_score FROM competition_participants")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(result, (Some(common::decimal("42")), None));
}
