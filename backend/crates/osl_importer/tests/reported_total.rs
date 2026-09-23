use osl_db::params::{RankingFilter, RankingMovement, SortDirection};
use osl_db::repository::{
    athlete::AthleteRepository, competition::CompetitionRepository, ranking::RankingRepository,
    ris::RisRepository,
};
use osl_db::services::ris_computation::recompute_all_ris;
use osl_domain::{AthleteStatus, Edition, Gender, Movement};
use osl_importer::canonical::{models::CanonicalFormat, store, validator::CanonicalValidator};
use rust_decimal::Decimal;
use sqlx::PgPool;

mod common;

fn source() -> CanonicalFormat {
    let mut athlete = common::athlete("Published", "Total");
    athlete.reported_total = Some(Decimal::from(400));
    common::competition("reported-total", vec![common::men_80(vec![athlete])])
}

fn filter(movement: RankingMovement) -> RankingFilter {
    RankingFilter {
        gender: None,
        country: None,
        federation: None,
        name: None,
        movement,
        direction: SortDirection::Desc,
        event: osl_domain::FULL_EVENT.into(),
        category: None,
        year: None,
        competition_id: None,
        offset: 0,
        limit: 10,
    }
}

#[test]
fn reported_totals_require_a_positive_competed_all4_result_without_lift_values() {
    CanonicalValidator::validate(&source()).unwrap();
    for value in [0, -1] {
        let mut invalid = source();
        invalid.categories[0].athletes[0].reported_total = Some(Decimal::from(value));
        assert!(CanonicalValidator::validate(&invalid).is_err());
    }
    for status in [AthleteStatus::Disqualified, AthleteStatus::NoShow] {
        let mut invalid = source();
        invalid.categories[0].athletes[0].status = status;
        assert!(CanonicalValidator::validate(&invalid).is_err());
    }
    let mut partial = source();
    partial.movements = vec![Movement::PullUp, Movement::Dips];
    assert!(CanonicalValidator::validate(&partial).is_err());
    let mut mixed = source();
    mixed.categories[0].athletes[0] = common::best(
        mixed.categories[0].athletes[0].clone(),
        Movement::Squat,
        "200",
    );
    assert!(CanonicalValidator::validate(&mixed).is_err());
}

#[test]
fn formatting_preserves_a_reported_total_without_creating_lifts() {
    let directory = std::env::temp_dir().join(format!("osl-total-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&directory).unwrap();
    store::write(&directory, &source()).unwrap();
    let read = store::read(&directory).unwrap();
    CanonicalValidator::validate(&read).unwrap();
    assert_eq!(
        read.categories[0].athletes[0].reported_total,
        Some(Decimal::from(400))
    );
    assert!(read.categories[0].athletes[0].lifts.is_empty());
    let original = std::fs::read(directory.join("entries.csv")).unwrap();
    store::write(&directory, &read).unwrap();
    assert_eq!(
        std::fs::read(directory.join("entries.csv")).unwrap(),
        original
    );
    std::fs::remove_dir_all(directory).unwrap();
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn total_only_results_rank_and_score_without_movement_records(pool: PgPool) {
    let mut canonical = source();
    let mut unknown_weight = common::athlete("Unknown", "Weight");
    unknown_weight.bodyweight = None;
    unknown_weight.reported_total = Some(Decimal::from(300));
    let mut reported_ris = common::scored(common::athlete("Published", "Score"), "60");
    reported_ris.reported_total = Some(Decimal::from(250));
    canonical.categories[0].athletes.extend([
        unknown_weight,
        reported_ris,
        common::no_show(common::athlete("Absent", "Lifter"), None),
    ]);
    common::import(&pool, canonical).await;
    assert_eq!(recompute_all_ris(&pool).await.unwrap(), 1);
    let expected_ris = osl_domain::ris::compute(
        Decimal::from(80),
        Decimal::from(400),
        Gender::M,
        Edition::CURRENT,
    );
    let repo = RankingRepository::new(&pool);
    let (total, count) = repo
        .get_global_ranking(&filter(RankingMovement::Total))
        .await
        .unwrap();
    assert_eq!(count, 3);
    assert_eq!(total[0].total, Some(Decimal::from(400)));
    assert_eq!(total[0].ris_score, Some(expected_ris));
    assert!(
        total[0].muscleup.is_none()
            && total[0].pullup.is_none()
            && total[0].dips.is_none()
            && total[0].squat.is_none()
    );
    assert_eq!(
        repo.get_global_ranking(&filter(RankingMovement::Ris))
            .await
            .unwrap()
            .1,
        2
    );
    for movement in [
        RankingMovement::Muscleup,
        RankingMovement::Pullup,
        RankingMovement::Dips,
        RankingMovement::Squat,
    ] {
        assert_eq!(
            repo.get_global_ranking(&filter(movement)).await.unwrap().1,
            0
        );
    }
    let detail = CompetitionRepository::new(&pool)
        .find_by_slug_detailed("reported-total")
        .await
        .unwrap();
    let participants = &detail.categories[0].participants;
    let winner = participants
        .iter()
        .find(|p| p.athlete.last_name == "Total")
        .unwrap();
    assert_eq!(winner.total, Some(Decimal::from(400)));
    assert_eq!(winner.rank, Some(1));
    assert!(winner.lifts.is_empty());
    let unscored = participants
        .iter()
        .find(|p| p.athlete.last_name == "Weight")
        .unwrap();
    assert_eq!(unscored.rank, Some(2));
    assert_eq!(unscored.total, Some(Decimal::from(300)));
    assert!(unscored.ris_score.is_none());
    let absent = participants
        .iter()
        .find(|p| p.athlete.last_name == "Lifter")
        .unwrap();
    assert!(absent.total.is_none() && absent.rank.is_none());
    let history = AthleteRepository::new(&pool)
        .find_by_slug_detailed("published-total")
        .await
        .unwrap();
    assert_eq!(history.competitions[0].total, Some(Decimal::from(400)));
    assert_eq!(history.competitions[0].rank, Some(1));
    assert!(history.personal_records.is_empty());
    let standings = repo
        .get_athlete_metric_standings(winner.athlete.athlete_id)
        .await
        .unwrap();
    assert_eq!(standings.len(), 2);
    let performances = RisRepository::new(&pool)
        .scored_performances()
        .await
        .unwrap();
    assert_eq!(performances.len(), 1);
    assert_eq!(performances[0].total, Decimal::from(400));
    let counts: (i64, i64) =
        sqlx::query_as("SELECT (SELECT COUNT(*) FROM lifts), (SELECT COUNT(*) FROM attempts)")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(counts, (0, 0));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn reimport_can_replace_a_reported_total_with_lifts_and_back(pool: PgPool) {
    common::import(&pool, source()).await;
    let mut breakdown = source();
    let athlete = &mut breakdown.categories[0].athletes[0];
    athlete.reported_total = None;
    *athlete = common::lifting(athlete.clone(), ["20", "70", "110", "210"]);
    common::import(&pool, breakdown).await;
    let (ranking, _) = RankingRepository::new(&pool)
        .get_global_ranking(&filter(RankingMovement::Total))
        .await
        .unwrap();
    assert_eq!(ranking[0].total, Some(Decimal::from(410)));
    let stored: Option<Decimal> =
        sqlx::query_scalar("SELECT reported_total FROM competition_participants")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(stored, None);
    common::import(&pool, source()).await;
    let (ranking, _) = RankingRepository::new(&pool)
        .get_global_ranking(&filter(RankingMovement::Total))
        .await
        .unwrap();
    assert_eq!(ranking[0].total, Some(Decimal::from(400)));
    assert!(ranking[0].squat.is_none());
    let attempts: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM attempts")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(attempts, 0);
    let mut invalid = source();
    invalid.categories[0].athletes[0].status = AthleteStatus::NoShow;
    assert!(common::try_import(&pool, invalid).await.is_err());
}
