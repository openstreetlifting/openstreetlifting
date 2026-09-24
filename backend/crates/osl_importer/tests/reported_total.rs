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
    athlete.total = Some(Decimal::from(400));
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
fn totals_require_a_nonnegative_competed_result_and_accept_partial_breakdowns() {
    CanonicalValidator::validate(&source()).unwrap();
    let mut negative = source();
    negative.categories[0].athletes[0].total = Some(Decimal::NEGATIVE_ONE);
    assert!(CanonicalValidator::validate(&negative).is_err());
    for status in [AthleteStatus::Disqualified, AthleteStatus::NoShow] {
        let mut invalid = source();
        invalid.categories[0].athletes[0].status = status;
        assert!(CanonicalValidator::validate(&invalid).is_err());
    }
    let mut partial = source();
    partial.movements = vec![Movement::PullUp, Movement::Dips];
    CanonicalValidator::validate(&partial).unwrap();
    let mut mixed = source();
    mixed.categories[0].athletes[0] = common::best(
        mixed.categories[0].athletes[0].clone(),
        Movement::Squat,
        "200",
    );
    CanonicalValidator::validate(&mixed).unwrap();
    mixed.categories[0].athletes[0].total = Some(Decimal::from(199));
    assert!(CanonicalValidator::validate(&mixed).is_err());
}

#[test]
fn formatting_preserves_a_total_without_creating_lifts() {
    let directory = std::env::temp_dir().join(format!("osl-total-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&directory).unwrap();
    store::write(&directory, &source()).unwrap();
    let read = store::read(&directory).unwrap();
    CanonicalValidator::validate(&read).unwrap();
    assert_eq!(
        read.categories[0].athletes[0].total,
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
    unknown_weight.total = Some(Decimal::from(300));
    let mut reported_ris = common::scored(common::athlete("Published", "Score"), "60");
    reported_ris.total = Some(Decimal::from(250));
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
async fn reimport_can_replace_a_total_with_lifts_and_back(pool: PgPool) {
    common::import(&pool, source()).await;
    let mut breakdown = source();
    let athlete = &mut breakdown.categories[0].athletes[0];
    athlete.total = None;
    *athlete = common::lifting(athlete.clone(), ["20", "70", "110", "210"]);
    common::import(&pool, breakdown).await;
    let (ranking, _) = RankingRepository::new(&pool)
        .get_global_ranking(&filter(RankingMovement::Total))
        .await
        .unwrap();
    assert_eq!(ranking[0].total, Some(Decimal::from(410)));
    let stored: Option<Decimal> = sqlx::query_scalar("SELECT total FROM competition_participants")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(stored, Some(Decimal::from(410)));
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

#[test]
fn preparation_fills_complete_totals_and_preserves_partial_source_totals() {
    use osl_importer::canonical::format::prepare;
    let mut canonical = source();
    let athlete = &mut canonical.categories[0].athletes[0];
    athlete.total = None;
    *athlete = common::lifting(athlete.clone(), ["0", "70", "110", "210"]);
    assert!(
        CanonicalValidator::validate(&canonical)
            .unwrap_err()
            .to_string()
            .contains("run `osl-import prepare`")
    );
    prepare(&mut canonical).unwrap();
    assert_eq!(
        canonical.categories[0].athletes[0].total,
        Some(Decimal::from(390))
    );
    prepare(&mut canonical).unwrap();
    canonical.categories[0].athletes[0].lifts.pop();
    prepare(&mut canonical).unwrap();
    assert_eq!(
        canonical.categories[0].athletes[0].total,
        Some(Decimal::from(390))
    );
    canonical.categories[0].athletes[0].total = None;
    prepare(&mut canonical).unwrap();
    assert_eq!(canonical.categories[0].athletes[0].total, None);
}

#[test]
fn preparation_rejects_conflicts_without_replacing_the_total() {
    use osl_importer::canonical::format::prepare;
    for total in [399, 401] {
        let mut canonical = source();
        let athlete = &mut canonical.categories[0].athletes[0];
        *athlete = common::lifting(athlete.clone(), ["20", "70", "110", "200"]);
        athlete.total = Some(Decimal::from(total));
        assert!(
            prepare(&mut canonical)
                .unwrap_err()
                .to_string()
                .contains("contradicts")
        );
        assert_eq!(
            canonical.categories[0].athletes[0].total,
            Some(Decimal::from(total))
        );
    }
}

#[test]
fn preparation_uses_the_declared_event_and_counts_successful_zeroes() {
    use osl_importer::canonical::format::prepare;
    let mut canonical = source();
    canonical.movements = vec![Movement::PullUp, Movement::Dips];
    canonical.categories[0].athletes[0] = common::best(
        common::best(common::athlete("Zero", "Weight"), Movement::PullUp, "0"),
        Movement::Dips,
        "0",
    );
    prepare(&mut canonical).unwrap();
    assert_eq!(
        canonical.categories[0].athletes[0].total,
        Some(Decimal::ZERO)
    );
    let duplicate = canonical.categories[0].athletes[0].lifts[0].clone();
    canonical.categories[0].athletes[0].lifts.push(duplicate);
    assert!(
        prepare(&mut canonical)
            .unwrap_err()
            .to_string()
            .contains("duplicate")
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn partial_breakdowns_never_become_totals_and_reimport_clears_scores(pool: PgPool) {
    let mut canonical = source();
    canonical.categories[0].athletes[0] = common::best(
        canonical.categories[0].athletes[0].clone(),
        Movement::Squat,
        "200",
    );
    common::import(&pool, canonical.clone()).await;
    canonical.categories[0].athletes[0].total = None;
    common::import(&pool, canonical).await;
    let row: (Option<Decimal>, Option<Decimal>) =
        sqlx::query_as("SELECT total, ris_score FROM competition_participants")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(row, (None, None));
    assert_eq!(recompute_all_ris(&pool).await.unwrap(), 0);
    let detail = CompetitionRepository::new(&pool)
        .find_by_slug_detailed("reported-total")
        .await
        .unwrap();
    let participant = &detail.categories[0].participants[0];
    assert_eq!(participant.total, None);
    assert_eq!(participant.rank, None);
    assert!(!participant.lifts.is_empty());
    let history = AthleteRepository::new(&pool)
        .find_by_slug_detailed("published-total")
        .await
        .unwrap();
    assert_eq!(history.competitions[0].total, None);
    assert_eq!(history.competitions[0].rank, None);
    let repo = RankingRepository::new(&pool);
    assert_eq!(
        repo.get_global_ranking(&filter(RankingMovement::Total))
            .await
            .unwrap()
            .1,
        0
    );
    assert_eq!(
        repo.get_global_ranking(&filter(RankingMovement::Squat))
            .await
            .unwrap()
            .1,
        1
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn import_rejects_missing_or_conflicting_totals_before_changing_stored_results(pool: PgPool) {
    use osl_importer::canonical::transformer::CanonicalTransformer;
    common::import(&pool, source()).await;
    for total in [None, Some(Decimal::from(399)), Some(Decimal::from(401))] {
        let mut canonical = source();
        let athlete = &mut canonical.categories[0].athletes[0];
        *athlete = common::lifting(athlete.clone(), ["20", "70", "110", "200"]);
        athlete.total = total;
        assert!(
            CanonicalTransformer::new(&pool)
                .import_to_database(canonical)
                .await
                .is_err()
        );
        let stored: Decimal = sqlx::query_scalar("SELECT total FROM competition_participants")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(stored, Decimal::from(400));
        let lifts: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM lifts")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(lifts, 0);
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn total_migration_backfills_complete_results_and_removes_subtotal_scores(pool: PgPool) {
    let complete = common::lifting(
        common::athlete("Complete", "Result"),
        ["0", "70", "110", "210"],
    );
    let partial = common::best(common::athlete("Partial", "Result"), Movement::Squat, "200");
    let mut published = partial.clone();
    published.first_name = "Published".into();
    published.total = Some(Decimal::from(400));
    let disqualified = common::disqualified(
        common::lifting(
            common::athlete("Disqualified", "Result"),
            ["20", "70", "110", "210"],
        ),
        None,
    );
    common::import(
        &pool,
        common::competition(
            "migration",
            vec![common::men_80(vec![
                complete,
                partial,
                published,
                disqualified,
            ])],
        ),
    )
    .await;

    // Recreate the previous schema and its computed subtotal, then run the migration.
    sqlx::raw_sql("ALTER TABLE competition_participants RENAME COLUMN total TO reported_total;
        ALTER TABLE competition_participants DROP CONSTRAINT total_valid;
        ALTER TABLE competition_participants ADD CONSTRAINT reported_total_valid CHECK (reported_total IS NULL OR (reported_total > 0 AND status = 'competed'));
        UPDATE competition_participants cp SET reported_total = NULL FROM athletes a WHERE a.athlete_id = cp.athlete_id AND a.first_name <> 'Published';
        UPDATE competition_participants cp SET ris_score = 50, ris_source = 'computed', ris_edition = 2026 FROM athletes a WHERE a.athlete_id = cp.athlete_id AND a.first_name = 'Partial';")
        .execute(&pool).await.unwrap();
    sqlx::raw_sql(include_str!(
        "../../osl_db/migrations/20260923130000_store_complete_totals.sql"
    ))
    .execute(&pool)
    .await
    .unwrap();
    let rows: Vec<(String, Option<Decimal>, Option<Decimal>)> = sqlx::query_as("SELECT a.first_name, cp.total, cp.ris_score FROM competition_participants cp JOIN athletes a USING (athlete_id) ORDER BY a.first_name")
        .fetch_all(&pool).await.unwrap();
    assert_eq!(rows[0].1, Some(Decimal::from(390)));
    assert!(rows[0].2.is_some());
    assert_eq!((rows[1].1, rows[1].2), (None, None));
    assert_eq!((rows[2].1, rows[2].2), (None, None));
    assert_eq!(rows[3].1, Some(Decimal::from(400)));
    assert!(rows[3].2.is_some());
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn classless_reported_scores_rank_without_inventing_totals(pool: PgPool) {
    let mut known = common::scored(common::athlete("Known", "Total"), "100");
    known.total = Some(Decimal::from(400));
    let unknown = common::scored(common::athlete("Unknown", "Total"), "100");
    let mut category = common::men_80(vec![known, unknown]);
    category.weight_class_slug = None;
    common::import(&pool, common::competition("classless", vec![category])).await;
    let detail = CompetitionRepository::new(&pool)
        .find_by_slug_detailed("classless")
        .await
        .unwrap();
    let participants = &detail.categories[0].participants;
    let known = participants
        .iter()
        .find(|p| p.athlete.first_name == "Known")
        .unwrap();
    let unknown = participants
        .iter()
        .find(|p| p.athlete.first_name == "Unknown")
        .unwrap();
    assert_eq!(known.rank, Some(1));
    assert_eq!(unknown.rank, Some(2));
    assert_eq!(unknown.total, None);
    let history = AthleteRepository::new(&pool)
        .find_by_slug_detailed("unknown-total")
        .await
        .unwrap();
    assert_eq!(history.competitions[0].rank, Some(2));
    assert_eq!(history.competitions[0].total, None);
    let repo = RankingRepository::new(&pool);
    assert_eq!(
        repo.get_global_ranking(&filter(RankingMovement::Ris))
            .await
            .unwrap()
            .1,
        2
    );
    assert_eq!(
        repo.get_global_ranking(&filter(RankingMovement::Total))
            .await
            .unwrap()
            .1,
        1
    );
}
