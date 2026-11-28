use osl_db::filter::Filters;
use osl_db::params::{RankingBy, RankingFilter};
use osl_db::repository::{athlete::AthleteRepository, competition::CompetitionRepository};
use osl_domain::{Gender, RankBy};
use osl_importer::canonical::{
    models::CanonicalFormat, store, transformer::CanonicalTransformer,
    validator::CanonicalValidator,
};
use sqlx::PgPool;
use uuid::Uuid;

mod common;

fn mixed(rank_by: Option<RankBy>, classed: bool) -> CanonicalFormat {
    let mut man = common::athlete("Alex", "Example");
    man.total = Some(common::decimal("300"));
    let mut woman = common::athlete("Alex", "Example");
    woman.gender = Gender::F;
    woman.bodyweight = Some(common::decimal("60"));
    woman.total = Some(common::decimal("200"));
    let mut category = common::men_80(vec![man, woman]);
    category.gender = Gender::Mx;
    category.weight_class_slug = None;
    category.weight_class_max = classed.then(|| common::decimal("80"));
    let mut canonical = common::competition("mixed", vec![category]);
    canonical.competition.rank_by = rank_by;
    canonical
}

#[test]
fn requires_athlete_sex_but_not_rank_by_for_mixed_contests() {
    CanonicalValidator::validate(&mixed(None, false)).unwrap();
    let mut canonical = mixed(Some(RankBy::Ris), false);
    canonical.categories[0].athletes[0].gender = Gender::Mx;
    assert!(
        CanonicalValidator::validate(&canonical)
            .unwrap_err()
            .to_string()
            .contains("Sex must be M or F")
    );
    let mut canonical = mixed(Some(RankBy::Ris), false);
    canonical.movements.pop();
    canonical.categories[0].weight_class_max = Some(common::decimal("80"));
    assert!(
        CanonicalValidator::validate(&canonical)
            .unwrap_err()
            .to_string()
            .contains("MPDS")
    );
}

#[test]
fn preserves_athlete_sex_on_round_trip() {
    let canonical = mixed(Some(RankBy::Ris), false);
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
    assert_eq!(read.categories[0].athletes[1].gender, Gender::F);
    assert_eq!(read.competition.rank_by, Some(RankBy::Ris));
    std::fs::write(path.join("entries.csv"), text.replace("\nM,MX,", "\n,MX,")).unwrap();
    assert!(store::read(&path).is_err());
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
async fn uses_athlete_sex_and_declared_rank_by(pool: PgPool) {
    for classed in [false, true] {
        for rank_by in [RankBy::Ris, RankBy::Total] {
            let canonical = mixed(Some(rank_by), classed);
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
                if rank_by == RankBy::Ris {
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
async fn omits_rank_when_ranking_metric_is_missing(pool: PgPool) {
    let mut canonical = mixed(Some(RankBy::Ris), false);
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
async fn shares_identity_across_contest_types(pool: PgPool) {
    let mut canonical = mixed(Some(RankBy::Total), false);
    let mut men = canonical.categories[0].clone();
    men.gender = Gender::M;
    men.athletes.retain(|a| a.gender == Gender::M);
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

    let mut filter = RankingFilter {
        by: Filters::default(),
        name: None,
        movement: osl_db::params::RankingMovement::Total,
        direction: osl_db::params::SortDirection::Desc,
        event: osl_domain::FULL_EVENT.into(),
        offset: 0,
        limit: 20,
    };
    let rankings = osl_db::repository::ranking::RankingRepository::new(&pool);
    for gender in [Gender::M, Gender::F] {
        filter.by = [RankingBy::Sex(gender)].into();
        let (rows, count) = rankings.get_global_ranking(&filter).await.unwrap();
        assert_eq!(count, 1);
        assert_eq!(rows[0].gender, gender);
    }
}
