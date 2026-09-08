use chrono::NaiveDate;
use osl_db::projections::athlete::AthleteStrengthRow;
use osl_db::repository::athlete::AthleteRepository;
use osl_domain::{Gender, Movement, WeightClassSlug};
use sqlx::PgPool;

mod common;
use common::{
    athlete, best, category, competition, disqualified, import, lifting, men_80, weighing,
};

async fn profile(pool: &PgPool) -> Vec<AthleteStrengthRow> {
    let repo = AthleteRepository::new(pool);
    let target = repo.find_by_slug("test-target").await.unwrap();
    repo.strength_profile(target.athlete_id).await.unwrap()
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn compares_one_best_per_other_athlete_and_gives_ties_half_credit(pool: PgPool) {
    import(
        &pool,
        competition(
            "first",
            vec![men_80(vec![
                lifting(athlete("Test", "Target"), ["40", "80", "100", "180"]),
                lifting(athlete("Test", "Lower"), ["20", "60", "80", "160"]),
                lifting(athlete("Test", "Tied"), ["40", "80", "100", "180"]),
                lifting(athlete("Test", "Higher"), ["60", "100", "120", "200"]),
                disqualified(
                    lifting(athlete("Test", "Disqualified"), ["90", "130", "140", "250"]),
                    None,
                ),
            ])],
        ),
    )
    .await;
    import(
        &pool,
        competition(
            "second",
            vec![men_80(vec![lifting(
                athlete("Test", "Lower"),
                ["30", "70", "90", "170"],
            )])],
        ),
    )
    .await;
    let rows = profile(&pool).await;
    assert_eq!(rows.len(), 4);
    for row in rows {
        assert_eq!(
            row.field, 3,
            "repeat competitions and disqualified results do not add peers"
        );
        assert_eq!(row.percentile, Some(50.0));
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn uses_latest_competed_category_for_every_axis_and_excludes_other_sexes(pool: PgPool) {
    let mut earlier = competition(
        "earlier",
        vec![category(
            WeightClassSlug::M66,
            vec![weighing(
                lifting(athlete("Test", "Target"), ["60", "100", "120", "200"]),
                "66",
            )],
        )],
    );
    earlier.competition.start_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    earlier.competition.end_date = earlier.competition.start_date;
    import(&pool, earlier).await;
    import(
        &pool,
        competition(
            "current",
            vec![men_80(vec![
                lifting(athlete("Test", "Target"), ["40", "80", "100", "180"]),
                lifting(athlete("Test", "Other"), ["20", "60", "80", "160"]),
            ])],
        ),
    )
    .await;
    let mut future = competition(
        "disqualified",
        vec![category(
            WeightClassSlug::M66,
            vec![disqualified(
                weighing(
                    lifting(athlete("Test", "Target"), ["70", "110", "130", "210"]),
                    "66",
                ),
                None,
            )],
        )],
    );
    future.competition.start_date = NaiveDate::from_ymd_opt(2027, 1, 1).unwrap();
    future.competition.end_date = future.competition.start_date;
    import(&pool, future).await;
    let mut woman = lifting(athlete("Test", "Woman"), ["50", "90", "110", "190"]);
    woman.gender = Some(Gender::F);
    import(
        &pool,
        competition(
            "women",
            vec![category(WeightClassSlug::FPlus70, vec![woman])],
        ),
    )
    .await;
    let rows = profile(&pool).await;
    for row in &rows {
        assert_eq!(row.weight_class_max, Some(common::decimal("80")));
        assert_eq!(row.field, 1);
        assert_eq!(row.percentile, Some(100.0));
    }
    assert_eq!(
        rows[0].value,
        Some(common::decimal("40")),
        "older category PRs must not leak into the radar"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn preserves_zero_and_keeps_missing_lifts_or_peers_unscored(pool: PgPool) {
    import(
        &pool,
        competition(
            "partial",
            vec![men_80(vec![
                best(
                    best(athlete("Test", "Target"), Movement::MuscleUp, "0"),
                    Movement::PullUp,
                    "70",
                ),
                best(
                    best(athlete("Test", "Peer"), Movement::MuscleUp, "10"),
                    Movement::Dips,
                    "100",
                ),
            ])],
        ),
    )
    .await;
    let rows = profile(&pool).await;
    let muscleup = rows
        .iter()
        .find(|row| row.movement_name == "Muscle-up")
        .unwrap();
    assert_eq!(muscleup.value, Some(common::decimal("0")));
    assert_eq!(muscleup.percentile, Some(0.0));
    assert_eq!(muscleup.field, 1);
    let pullup = rows
        .iter()
        .find(|row| row.movement_name == "Pull-up")
        .unwrap();
    assert_eq!(pullup.field, 0);
    assert_eq!(pullup.percentile, None);
    let dips = rows.iter().find(|row| row.movement_name == "Dips").unwrap();
    assert_eq!(dips.value, None);
    assert_eq!(dips.percentile, None);
    assert_eq!(dips.field, 1);
}
