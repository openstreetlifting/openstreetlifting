use osl_db::repository::competition::CompetitionRepository;
use osl_domain::RankBy;
use osl_importer::canonical::transformer::CanonicalTransformer;
use sqlx::PgPool;

mod common;

fn lifter(
    first: &str,
    bodyweight: &str,
    total: &str,
    published: Option<&str>,
) -> osl_importer::canonical::models::AthleteData {
    let mut athlete = common::athlete(first, "Lifter");
    athlete.bodyweight = Some(common::decimal(bodyweight));
    athlete.total = Some(common::decimal(total));
    athlete.reported_ris = published.map(common::decimal);
    athlete
}

async fn placings(
    pool: &PgPool,
    heavy: Option<&str>,
    light: Option<&str>,
) -> Vec<(String, i32, rust_decimal::Decimal)> {
    let category = common::men_80(vec![
        lifter("Heavy", "100", "320", heavy),
        lifter("Light", "60", "300", light),
    ]);
    let mut canonical = common::competition("published", vec![category]);
    canonical.competition.rank_by = Some(RankBy::Ris);
    CanonicalTransformer::new(pool)
        .import_to_database(canonical)
        .await
        .unwrap();
    let detail = CompetitionRepository::new(pool)
        .find_by_slug_detailed("published")
        .await
        .unwrap();
    detail.categories[0]
        .participants
        .iter()
        .map(|p| {
            (
                p.athlete.first_name.clone(),
                p.rank.unwrap(),
                p.ris_score.unwrap(),
            )
        })
        .collect()
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn places_by_published_ris_but_shows_current_ris(pool: PgPool) {
    let placed = placings(&pool, Some("900"), Some("800")).await;
    assert_eq!(placed[0].0, "Heavy");
    assert_eq!(placed[1].0, "Light");
    assert!(
        placed[1].2 > placed[0].2,
        "current RIS stays on display: {placed:?}"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn places_by_current_ris_without_published_scores(pool: PgPool) {
    let placed = placings(&pool, None, None).await;
    assert_eq!(placed[0].0, "Light");
    assert_eq!(placed[1].0, "Heavy");
}
