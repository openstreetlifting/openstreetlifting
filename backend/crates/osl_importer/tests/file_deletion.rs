//! Deleting a canonical file has to delete its competition, since the files are
//! the data. What it must not take with it is the reference data and the
//! athletes who lifted somewhere else too.

use osl_domain::Movement;
use osl_importer::canonical::models::{AthleteData, CanonicalFormat};
use osl_importer::sync::CompetitionSync;
use sqlx::PgPool;

mod common;

use common::{attempts, import, men_80};

fn athlete(first: &str, last: &str) -> AthleteData {
    let lifter = attempts(
        common::athlete(first, last),
        Movement::MuscleUp,
        &[("40", true)],
    );
    attempts(lifter, Movement::PullUp, &[("90", true)])
}

fn meet(slug: &str, athletes: Vec<AthleteData>) -> CanonicalFormat {
    let mut canonical = common::meet(slug, vec![men_80(athletes)]);
    canonical.movements = vec![Movement::MuscleUp, Movement::PullUp];
    canonical
}

/// One meet whose file is still there, one whose file was deleted, and a lifter
/// who was at both.
async fn two_meets(pool: &PgPool) {
    import(
        pool,
        meet(
            "kept-meet",
            vec![athlete("Clara", "Clean"), athlete("Sam", "Shared")],
        ),
    )
    .await;
    import(
        pool,
        meet(
            "gone-meet",
            vec![athlete("Sam", "Shared"), athlete("Gil", "Gone")],
        ),
    )
    .await;
}

async fn count(pool: &PgPool, query: &'static str) -> i64 {
    sqlx::query_scalar(query).fetch_one(pool).await.unwrap()
}

async fn slugs(pool: &PgPool) -> Vec<String> {
    sqlx::query_scalar("SELECT slug FROM competitions ORDER BY slug")
        .fetch_all(pool)
        .await
        .unwrap()
}

async fn last_names(pool: &PgPool) -> Vec<String> {
    sqlx::query_scalar("SELECT last_name FROM athletes ORDER BY last_name")
        .fetch_all(pool)
        .await
        .unwrap()
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_competition_no_file_claims_is_deleted(pool: PgPool) {
    two_meets(&pool).await;

    let plan = CompetitionSync::new(&pool)
        .apply(&["kept-meet".to_string()])
        .await
        .unwrap();

    assert_eq!(slugs(&pool).await, vec!["kept-meet"]);
    assert_eq!(
        plan.competitions
            .iter()
            .map(|c| c.slug.as_str())
            .collect::<Vec<_>>(),
        vec!["gone-meet"],
        "the plan has to name what it deleted"
    );

    assert_eq!(
        count(&pool, "SELECT COUNT(*) FROM competition_participants").await,
        2
    );
    assert_eq!(count(&pool, "SELECT COUNT(*) FROM lifts").await, 4);
    assert_eq!(
        count(&pool, "SELECT COUNT(*) FROM attempts").await,
        4,
        "the attempts of a deleted competition go with it"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn an_athlete_left_with_no_result_goes_too(pool: PgPool) {
    two_meets(&pool).await;

    CompetitionSync::new(&pool)
        .apply(&["kept-meet".to_string()])
        .await
        .unwrap();

    assert_eq!(
        last_names(&pool).await,
        vec!["Clean", "Shared"],
        "Gone only ever lifted at the deleted meet, Shared was at both"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn reference_data_is_left_alone(pool: PgPool) {
    two_meets(&pool).await;

    let movements = count(&pool, "SELECT COUNT(*) FROM movements").await;
    let weight_classes = count(&pool, "SELECT COUNT(*) FROM weight_classes").await;

    CompetitionSync::new(&pool)
        .apply(&["kept-meet".to_string()])
        .await
        .unwrap();

    assert_eq!(count(&pool, "SELECT COUNT(*) FROM federations").await, 1);
    assert_eq!(count(&pool, "SELECT COUNT(*) FROM categories").await, 1);
    assert_eq!(
        count(&pool, "SELECT COUNT(*) FROM movements").await,
        movements
    );
    assert_eq!(
        count(&pool, "SELECT COUNT(*) FROM weight_classes").await,
        weight_classes
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn claiming_nothing_is_refused(pool: PgPool) {
    two_meets(&pool).await;

    let refused = CompetitionSync::new(&pool).apply(&[]).await;

    assert!(
        refused.is_err(),
        "an empty tree means a mistake, not an instruction to delete everything"
    );
    assert_eq!(slugs(&pool).await, vec!["gone-meet", "kept-meet"]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_dry_run_reports_without_deleting(pool: PgPool) {
    two_meets(&pool).await;

    let plan = CompetitionSync::new(&pool)
        .dry_run(&["kept-meet".to_string()])
        .await
        .unwrap();

    assert_eq!(
        plan.competitions
            .iter()
            .map(|c| c.slug.as_str())
            .collect::<Vec<_>>(),
        vec!["gone-meet"]
    );
    assert_eq!(plan.athletes, vec!["Gil Gone"]);
    assert_eq!(
        slugs(&pool).await,
        vec!["gone-meet", "kept-meet"],
        "reporting must not be a deletion"
    );
}
