use osl_db::repository::competition::CompetitionRepository;
use osl_domain::{Gender, Scoring};
use osl_importer::canonical::{
    models::CanonicalFormat, transformer::CanonicalTransformer, validator::CanonicalValidator,
};
use sqlx::PgPool;

mod common;

fn split() -> CanonicalFormat {
    let mut heavy = common::athlete("Heavy", "Lifter");
    heavy.bodyweight = Some(common::decimal("100"));
    heavy.total = Some(common::decimal("320"));
    let mut light = common::athlete("Light", "Lifter");
    light.bodyweight = Some(common::decimal("60"));
    light.total = Some(common::decimal("300"));

    let mut open = common::men_80(vec![heavy.clone(), light.clone()]);
    open.division = Some("Open".into());
    let mut elite = common::men_80(vec![heavy, light]);
    elite.division = Some("Elite".into());
    elite.gender = Gender::Mx;
    elite.weight_class_slug = None;

    let mut canonical = common::competition("split", vec![open, elite]);
    canonical
        .competition
        .divisions
        .insert("Open".into(), Scoring::Total);
    canonical
        .competition
        .divisions
        .insert("Elite".into(), Scoring::Ris);
    canonical
}

#[test]
fn accepts_mixed_contests_scored_by_division() {
    CanonicalValidator::validate(&split()).unwrap();
    let mut canonical = split();
    canonical.competition.divisions.remove("Elite");
    assert!(
        CanonicalValidator::validate(&canonical)
            .unwrap_err()
            .to_string()
            .contains("scoring")
    );
}

#[test]
fn rejects_unused_division_scoring() {
    let mut canonical = split();
    canonical
        .competition
        .divisions
        .insert("Masters".into(), Scoring::Ris);
    assert!(
        CanonicalValidator::validate(&canonical)
            .unwrap_err()
            .to_string()
            .contains("Masters")
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn ranks_each_division_by_its_own_scoring(pool: PgPool) {
    CanonicalTransformer::new(&pool)
        .import_to_database(split())
        .await
        .unwrap();
    let detail = CompetitionRepository::new(&pool)
        .find_by_slug_detailed("split")
        .await
        .unwrap();
    assert_eq!(detail.categories.len(), 2);
    for contest in &detail.categories {
        let winner = contest
            .participants
            .iter()
            .find(|p| p.rank == Some(1))
            .unwrap();
        match contest.category.division.as_deref() {
            Some("Open") => {
                assert_eq!(contest.category.scoring, Scoring::Total);
                assert_eq!(winner.athlete.first_name, "Heavy");
            }
            Some("Elite") => {
                assert_eq!(contest.category.scoring, Scoring::Ris);
                assert_eq!(winner.athlete.first_name, "Light");
            }
            other => panic!("unexpected division {other:?}"),
        }
    }
}
