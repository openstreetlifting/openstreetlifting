use osl_db::repository::athlete::AthleteRepository;
use osl_domain::Movement;
use sqlx::PgPool;

mod common;
use common::{athlete, competition, disqualified, import, in_division, lifting, men_80};

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn history_provides_records_in_movement_order_and_counts_distinct_meets(pool: PgPool) {
    let lifter = || athlete("Alex", "Martin");
    import(
        &pool,
        competition(
            "first",
            vec![
                in_division(
                    men_80(vec![lifting(lifter(), ["0", "80", "90", "120"])]),
                    "Open",
                ),
                in_division(
                    men_80(vec![lifting(lifter(), ["0", "80", "90", "120"])]),
                    "Junior",
                ),
            ],
        ),
    )
    .await;
    let mut later = competition(
        "later",
        vec![men_80(vec![lifting(lifter(), ["0", "60", "110", "180"])])],
    );
    later.competition.start_date = chrono::NaiveDate::from_ymd_opt(2026, 2, 1).unwrap();
    later.competition.end_date = later.competition.start_date;
    import(&pool, later).await;
    import(
        &pool,
        competition(
            "disqualified",
            vec![men_80(vec![disqualified(
                lifting(lifter(), ["80", "100", "150", "250"]),
                None,
            )])],
        ),
    )
    .await;

    let detail = AthleteRepository::new(&pool)
        .find_by_slug_detailed("alex-martin")
        .await
        .unwrap();
    assert_eq!(detail.total_competitions, 3);
    assert_eq!(
        detail.competitions.len(),
        4,
        "both divisions remain in the history"
    );
    let records: Vec<_> = detail
        .personal_records
        .iter()
        .map(|record| {
            (
                record.movement_name.as_str(),
                record.max_weight.to_string(),
                record.competition_slug.as_str(),
            )
        })
        .collect();
    assert_eq!(
        records,
        vec![
            ("Muscle-up", "0".to_string(), "later"),
            ("Pull-up", "80".to_string(), "first"),
            ("Dips", "110".to_string(), "later"),
            ("Squat", "180".to_string(), "later"),
        ]
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn bombed_and_uncontested_movements_do_not_create_records(pool: PgPool) {
    let lifter = common::attempts(
        common::attempts(
            athlete("Partial", "Lifter"),
            Movement::MuscleUp,
            &[("20", false)],
        ),
        Movement::PullUp,
        &[("60", true)],
    );
    let mut partial = competition("partial", vec![men_80(vec![lifter])]);
    partial.movements = vec![Movement::MuscleUp, Movement::PullUp];
    import(&pool, partial).await;
    let detail = AthleteRepository::new(&pool)
        .find_by_slug_detailed("partial-lifter")
        .await
        .unwrap();
    assert_eq!(detail.personal_records.len(), 1);
    assert_eq!(detail.personal_records[0].movement_name, "Pull-up");
    assert_eq!(detail.personal_records[0].max_weight, common::decimal("60"));
}
