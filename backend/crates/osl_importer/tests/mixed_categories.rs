use osl_db::repository::{athlete::AthleteRepository, competition::CompetitionRepository};
use osl_domain::{Gender, Scoring};
use osl_importer::canonical::{
    models::CanonicalFormat, store, transformer::CanonicalTransformer,
    validator::CanonicalValidator,
};
use sqlx::PgPool;
use uuid::Uuid;

mod common;

fn mixed(scoring: Option<Scoring>, classed: bool) -> CanonicalFormat {
    let mut man = common::athlete("Alex", "Example");
    man.total = Some(common::decimal("300"));
    let mut woman = common::athlete("Alex", "Example");
    woman.gender = Some(Gender::F);
    woman.bodyweight = Some(common::decimal("60"));
    woman.total = Some(common::decimal("200"));
    let mut category = common::men_80(vec![man, woman]);
    category.gender = Gender::Mx;
    category.weight_class_slug = None;
    category.weight_class_max = classed.then(|| common::decimal("80"));
    let mut canonical = common::competition("mixed", vec![category]);
    canonical.competition.scoring = scoring;
    canonical
}

#[test]
fn mixed_contests_require_athlete_sex_and_an_explicit_scoring_rule() {
    assert!(
        CanonicalValidator::validate(&mixed(None, false))
            .unwrap_err()
            .to_string()
            .contains("scoring")
    );
    for sex in [None, Some(Gender::Mx)] {
        let mut canonical = mixed(Some(Scoring::Ris), false);
        canonical.categories[0].athletes[0].gender = sex;
        assert!(
            CanonicalValidator::validate(&canonical)
                .unwrap_err()
                .to_string()
                .contains("Sex must be M or F")
        );
    }
    let mut canonical = mixed(Some(Scoring::Ris), false);
    canonical.movements.pop();
    assert!(
        CanonicalValidator::validate(&canonical)
            .unwrap_err()
            .to_string()
            .contains("MPDS")
    );
}

#[test]
fn mixed_csv_round_trips_without_changing_athlete_sex() {
    let canonical = mixed(Some(Scoring::Ris), false);
    CanonicalValidator::validate(&canonical).unwrap();
    let path = std::env::temp_dir().join(format!("osl-mixed-{}", Uuid::new_v4()));
    std::fs::create_dir_all(&path).unwrap();
    store::write(&path, &canonical).unwrap();
    let text = std::fs::read_to_string(path.join("entries.csv")).unwrap();
    assert!(text.starts_with("Sex,CategorySex,"));
    assert!(text.contains("\nM,MX,"));
    assert!(text.contains("\nF,MX,"));
    let read = store::read(&path).unwrap();
    assert_eq!(read.categories.len(), 1);
    assert_eq!(read.categories[0].gender, Gender::Mx);
    assert_eq!(read.categories[0].athletes[1].gender, Some(Gender::F));
    assert_eq!(read.competition.scoring, Some(Scoring::Ris));
    std::fs::write(
        path.join("entries.csv"),
        text.replace("\nM,MX,", "\nMX,MX,"),
    )
    .unwrap();
    assert!(
        store::read(&path)
            .unwrap_err()
            .to_string()
            .contains("Sex must be M or F")
    );
    std::fs::write(path.join("entries.csv"), text.replace("\nM,MX,", "\nM,F,")).unwrap();
    assert!(
        store::read(&path)
            .unwrap_err()
            .to_string()
            .contains("CategorySex must match Sex")
    );
    std::fs::remove_dir_all(path).unwrap();
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn mixed_results_use_athlete_formulas_and_the_declared_ranking(pool: PgPool) {
    for classed in [false, true] {
        for scoring in [Scoring::Ris, Scoring::Total] {
            let canonical = mixed(Some(scoring), classed);
            CanonicalTransformer::new(&pool)
                .import_to_database(canonical)
                .await
                .unwrap();
            let detail = CompetitionRepository::new(&pool)
                .find_by_slug_detailed("mixed")
                .await
                .unwrap();
            assert_eq!(detail.categories.len(), 1);
            let contest = &detail.categories[0];
            assert_eq!(contest.category.gender, Gender::Mx);
            assert_eq!(contest.participants.len(), 2);
            let winner = contest
                .participants
                .iter()
                .find(|p| p.rank == Some(1))
                .unwrap();
            assert_eq!(
                winner.athlete.gender,
                if scoring == Scoring::Ris {
                    Gender::F
                } else {
                    Gender::M
                }
            );
            for participant in &contest.participants {
                let expected = osl_domain::ris::compute(
                    participant.bodyweight.unwrap(),
                    participant.total.unwrap(),
                    participant.athlete.gender,
                    osl_domain::Edition::CURRENT,
                );
                assert_eq!(participant.ris_score, Some(expected));
                let profile = AthleteRepository::new(&pool)
                    .find_by_slug_detailed(&participant.athlete.slug)
                    .await
                    .unwrap();
                assert_eq!(profile.competitions[0].rank, participant.rank);
                assert_eq!(profile.competitions[0].category_gender, Gender::Mx);
            }
        }
    }
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM athletes")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 2);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_missing_scoring_metric_has_no_placing(pool: PgPool) {
    let mut canonical = mixed(Some(Scoring::Ris), false);
    canonical.categories[0].athletes[0].bodyweight = None;
    CanonicalTransformer::new(&pool)
        .import_to_database(canonical)
        .await
        .unwrap();
    let detail = CompetitionRepository::new(&pool)
        .find_by_slug_detailed("mixed")
        .await
        .unwrap();
    let unscored = detail.categories[0]
        .participants
        .iter()
        .find(|p| p.athlete.gender == Gender::M)
        .unwrap();
    assert_eq!(unscored.rank, None);
    assert_eq!(unscored.ris_score, None);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn one_identity_can_enter_mixed_and_single_sex_contests(pool: PgPool) {
    let mut canonical = mixed(Some(Scoring::Total), false);
    let mut men = canonical.categories[0].clone();
    men.gender = Gender::M;
    men.athletes.retain(|a| a.gender == Some(Gender::M));
    canonical.categories.push(men);
    CanonicalTransformer::new(&pool)
        .import_to_database(canonical)
        .await
        .unwrap();
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM athletes")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 2);
    let detail = CompetitionRepository::new(&pool)
        .find_by_slug_detailed("mixed")
        .await
        .unwrap();
    assert_eq!(detail.categories.len(), 2);
    assert_eq!(
        detail
            .categories
            .iter()
            .map(|c| c.participants.len())
            .sum::<usize>(),
        3
    );

    let mut filter = osl_db::params::RankingFilter {
        gender: Some(Gender::Mx),
        country: None,
        federation: None,
        name: None,
        movement: osl_db::params::RankingMovement::Total,
        direction: osl_db::params::SortDirection::Desc,
        event: osl_domain::FULL_EVENT.into(),
        category: None,
        year: None,
        competition_id: Some(detail.competition.competition_id),
        offset: 0,
        limit: 20,
    };
    let rankings = osl_db::repository::ranking::RankingRepository::new(&pool);
    let (rows, count) = rankings.get_global_ranking(&filter).await.unwrap();
    assert_eq!(count, 2);
    let mixed = detail
        .categories
        .iter()
        .find(|c| c.category.gender == Gender::Mx)
        .unwrap();
    assert!(rows.iter().all(|row| {
        mixed
            .participants
            .iter()
            .any(|p| p.participant_id == row.participant_id)
    }));
    filter.gender = Some(Gender::M);
    assert_eq!(rankings.get_global_ranking(&filter).await.unwrap().1, 1);
    filter.competition_id = None;
    for gender in [Gender::M, Gender::F] {
        filter.gender = Some(gender);
        let (rows, count) = rankings.get_global_ranking(&filter).await.unwrap();
        assert_eq!(count, 1);
        assert_eq!(rows[0].gender, gender);
    }
}
