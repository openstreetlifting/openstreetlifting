use chrono::NaiveDate;
use osl_db::projections::athlete::AthleteBalanceRow;
use osl_db::repository::athlete::AthleteRepository;
use osl_domain::{Gender, Movement, WeightClassSlug};
use sqlx::PgPool;

mod common;
use common::{
    athlete, best, category, competition, decimal, disqualified, import, lifting, men_80, weighing,
};

async fn balance(pool: &PgPool) -> Vec<AthleteBalanceRow> {
    let repo = AthleteRepository::new(pool);
    let target = repo.find_by_slug("test-target").await.unwrap();
    repo.lift_balance(target.athlete_id).await.unwrap()
}

fn lift(rows: &[AthleteBalanceRow], movement: Movement) -> &AthleteBalanceRow {
    rows.iter()
        .find(|row| row.movement_name == movement)
        .unwrap()
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn compares_best_total_shares_with_class_peers(pool: PgPool) {
    import(
        &pool,
        competition(
            "first",
            vec![men_80(vec![
                lifting(athlete("Test", "Target"), ["20", "60", "80", "240"]),
                lifting(athlete("Test", "Even"), ["40", "80", "100", "180"]),
                lifting(athlete("Test", "Squatter"), ["30", "70", "90", "210"]),
                disqualified(
                    lifting(athlete("Test", "Disqualified"), ["90", "130", "140", "250"]),
                    None,
                ),
                best(
                    best(athlete("Test", "Partial"), Movement::MuscleUp, "50"),
                    Movement::PullUp,
                    "100",
                ),
            ])],
        ),
    )
    .await;
    import(
        &pool,
        competition(
            "second",
            vec![men_80(vec![
                lifting(athlete("Test", "Target"), ["10", "50", "60", "180"]),
                lifting(athlete("Test", "Even"), ["10", "50", "60", "180"]),
            ])],
        ),
    )
    .await;

    let rows = balance(&pool).await;
    assert_eq!(rows.len(), 4);
    for row in &rows {
        assert_eq!(row.total, decimal("400"), "the best total is the source");
        assert_eq!(
            row.field, 2,
            "partial, disqualified and repeat results add no peers"
        );
        assert!(row.class_spread.is_some());
    }
    let expected = [
        (Movement::MuscleUp, 5.0, 8.75),
        (Movement::PullUp, 15.0, 18.75),
        (Movement::Dips, 20.0, 23.75),
        (Movement::Squat, 60.0, 48.75),
    ];
    for (movement, share, class_share) in expected {
        let row = lift(&rows, movement);
        assert!((row.share - share).abs() < 1e-9, "{movement:?} share");
        assert!(
            (row.class_share.unwrap() - class_share).abs() < 1e-9,
            "{movement:?} class share"
        );
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn uses_latest_category_and_gender(pool: PgPool) {
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
    let mut woman = lifting(athlete("Test", "Woman"), ["50", "90", "110", "190"]);
    woman.gender = Gender::F;
    import(
        &pool,
        competition(
            "women",
            vec![category(WeightClassSlug::FPlus70, vec![woman])],
        ),
    )
    .await;

    let rows = balance(&pool).await;
    assert_eq!(rows.len(), 4);
    for row in &rows {
        assert_eq!(row.weight_class_max, Some(decimal("80")));
        assert_eq!(row.total, decimal("400"), "older category results stay out");
        assert_eq!(row.field, 1);
        assert_eq!(row.class_spread, None, "one peer has no spread");
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn needs_four_valid_lifts(pool: PgPool) {
    import(
        &pool,
        competition(
            "zero",
            vec![men_80(vec![
                lifting(athlete("Test", "Target"), ["0", "80", "100", "180"]),
                lifting(athlete("Test", "Peer"), ["20", "60", "80", "160"]),
            ])],
        ),
    )
    .await;
    assert!(balance(&pool).await.is_empty());
}
