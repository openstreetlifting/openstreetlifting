use osl_db::params::{RankingFilter, RankingMovement, SortDirection};
use osl_db::repository::{competition::CompetitionRepository, ranking::RankingRepository};
use osl_domain::{AthleteStatus, Gender, Movement, Scoring};
use osl_importer::canonical::{models::CanonicalFormat, validator::CanonicalValidator};
use rust_decimal::Decimal;
use sqlx::PgPool;

mod common;

fn underground() -> CanonicalFormat {
    let mut higher_total = common::athlete("Higher", "Total");
    higher_total.reported_ris = Some(common::decimal("70"));
    let mut higher_ris = common::scored(common::athlete("Higher", "Score"), "90");
    for (movement, weight) in [
        (Movement::PullUp, "50"),
        (Movement::Dips, "100"),
        (Movement::Squat, "150"),
    ] {
        higher_total = common::best(higher_total, movement, weight);
        higher_ris = common::best(higher_ris, movement, "50");
    }
    let mut unscored = common::athlete("Missing", "Score");
    unscored.total = Some(common::decimal("400"));
    let mut disqualified = common::disqualified(
        common::scored(common::athlete("Bombed", "Squat"), "99"),
        Some("Bombed squat"),
    );
    disqualified = common::attempts(disqualified, Movement::Squat, &[("100", false)]);
    let mut category = common::men_80(vec![higher_total, higher_ris, unscored, disqualified]);
    category.weight_class_slug = None;
    let mut canonical = common::competition("underground", vec![category]);
    canonical.movements = vec![Movement::PullUp, Movement::Dips, Movement::Squat];
    canonical.competition.scoring = Some(Scoring::Ris);
    for athlete in &mut canonical.categories[0].athletes {
        if athlete.total.is_none() {
            athlete.total = athlete.total_from_lifts(&canonical.movements);
        }
    }
    canonical
}

#[test]
fn shorter_ris_events_require_no_weight_classes_in_the_entire_meet() {
    for movements in [
        vec![Movement::Squat],
        vec![Movement::PullUp, Movement::Dips],
        vec![Movement::PullUp, Movement::Dips, Movement::Squat],
    ] {
        let mut canonical = underground();
        canonical.movements = movements;
        canonical.categories[0].athletes.truncate(1);
        canonical.categories[0].athletes[0].lifts.clear();
        CanonicalValidator::validate(&canonical).unwrap();
        for (min, max) in [
            (None, Some(common::decimal("80"))),
            (Some(common::decimal("101")), None),
        ] {
            let mut classed = canonical.categories[0].clone();
            classed.weight_class_min = min;
            classed.weight_class_max = max;
            canonical.categories.push(classed);
            let error = CanonicalValidator::validate(&canonical)
                .unwrap_err()
                .to_string();
            assert!(error.contains("no weight classes"), "{error}");
            canonical.categories.pop();
        }
        canonical.categories[0].weight_class_slug = Some(osl_domain::WeightClassSlug::M80);
        assert!(CanonicalValidator::validate(&canonical).is_err());
    }
}

#[test]
fn shorter_ris_events_allow_sex_and_division_groups_and_warn_about_missing_scores() {
    let mut canonical = underground();
    let mut women = canonical.categories[0].clone();
    women.gender = Gender::F;
    women.athletes.truncate(1);
    women.athletes[0].gender = Gender::F;
    canonical.categories.push(women);
    for category in &mut canonical.categories {
        category.division = Some("Open".into());
    }
    let report = CanonicalValidator::validate(&canonical).unwrap();
    assert!(
        report
            .warnings
            .iter()
            .any(|warning| warning.contains("no published RIS"))
    );
    canonical.categories[1].gender = Gender::Mx;
    CanonicalValidator::validate(&canonical).unwrap();
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn published_short_event_scores_decide_placings_and_survive_recomputation(pool: PgPool) {
    let canonical = underground();
    CanonicalValidator::validate(&canonical).unwrap();
    common::import(&pool, canonical).await;
    osl_db::services::ris_computation::recompute_all_ris(&pool)
        .await
        .unwrap();
    let detail = CompetitionRepository::new(&pool)
        .find_by_slug_detailed("underground")
        .await
        .unwrap();
    let participants = &detail.categories[0].participants;
    let winner = participants.iter().find(|p| p.rank == Some(1)).unwrap();
    assert_eq!(winner.athlete.first_name, "Higher");
    assert_eq!(winner.athlete.last_name, "Score");
    let known_bodyweight = participants
        .iter()
        .find(|p| p.athlete.last_name == "Total")
        .unwrap();
    assert_eq!(known_bodyweight.ris_score, Some(common::decimal("70")));
    assert_eq!(
        known_bodyweight.ris_source,
        Some(osl_domain::RisSource::Reported)
    );
    for participant in participants
        .iter()
        .filter(|p| p.athlete.first_name == "Missing" || p.status == AthleteStatus::Disqualified)
    {
        assert_eq!(participant.rank, None);
        assert_eq!(participant.ris_score, None);
    }
    let score: Option<Decimal> = sqlx::query_scalar(
        "SELECT reported_ris_score FROM competition_participants WHERE status = 'disqualified'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(score, Some(common::decimal("99")));
    let computed: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM competition_participants WHERE ris_source = 'computed'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(computed, 0);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn shorter_event_scores_stay_out_of_all4_rankings_and_profile_standings(pool: PgPool) {
    common::import(&pool, underground()).await;
    let repository = RankingRepository::new(&pool);
    let mut filter = RankingFilter {
        gender: None,
        country: None,
        federation: None,
        name: None,
        movement: RankingMovement::Ris,
        direction: SortDirection::Desc,
        event: osl_domain::FULL_EVENT.into(),
        category: None,
        year: None,
        competition_id: None,
        offset: 0,
        limit: 50,
    };
    let (all4, count) = repository.get_global_ranking(&filter).await.unwrap();
    assert!(all4.is_empty());
    assert_eq!(count, 0);
    filter.event = "PDS".into();
    let (shorter, count) = repository.get_global_ranking(&filter).await.unwrap();
    assert_eq!(count, 2);
    assert_eq!(shorter[0].last_name, "Score");
    assert!(
        repository
            .get_athlete_metric_standings(shorter[0].athlete_id)
            .await
            .unwrap()
            .is_empty()
    );
    filter.event = osl_domain::FULL_EVENT.into();
    filter.competition_id = Some(shorter[0].competition_id);
    assert_eq!(repository.get_global_ranking(&filter).await.unwrap().1, 2);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn shorter_scores_require_explicit_ris_scoring_and_are_cleared_on_correction(pool: PgPool) {
    common::import(&pool, underground()).await;
    let mut canonical = underground();
    canonical.competition.scoring = Some(Scoring::Total);
    common::import(&pool, canonical).await;
    let rows: Vec<(Option<Decimal>, Option<Decimal>)> =
        sqlx::query_as("SELECT reported_ris_score, ris_score FROM competition_participants")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert!(rows.iter().all(|(_, score)| score.is_none()));
    assert_eq!(
        rows.iter()
            .filter(|(reported, _)| reported.is_some())
            .count(),
        3
    );
}
