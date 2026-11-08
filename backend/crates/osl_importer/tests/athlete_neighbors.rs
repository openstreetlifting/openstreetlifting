use osl_db::repository::ranking::RankingRepository;
use sqlx::PgPool;
use uuid::Uuid;

mod common;
use common::{athlete, decimal, from, import, lifting, men_80};

async fn id(pool: &PgPool, last: &str) -> Uuid {
    sqlx::query_scalar("SELECT athlete_id FROM athletes WHERE last_name = $1")
        .bind(last)
        .fetch_one(pool)
        .await
        .unwrap()
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn returns_the_closest_rival_on_each_side_in_each_scope(pool: PgPool) {
    import(
        &pool,
        common::competition(
            "board",
            vec![men_80(vec![
                lifting(athlete("Top", "Leader"), ["50", "70", "100", "200"]),
                from(
                    lifting(athlete("Foreign", "Second"), ["40", "70", "100", "200"]),
                    "IT",
                ),
                lifting(athlete("Above", "Third"), ["30", "70", "100", "200"]),
                lifting(athlete("Profile", "Target"), ["20", "70", "100", "200"]),
                lifting(athlete("Below", "Fifth"), ["10", "70", "100", "200"]),
                lifting(athlete("Far", "Sixth"), ["5", "70", "100", "200"]),
            ])],
        ),
    )
    .await;
    let rows = RankingRepository::new(&pool)
        .get_athlete_total_neighbors(id(&pool, "Target").await)
        .await
        .unwrap();
    let global: Vec<_> = rows.iter().filter(|r| r.scope == "global").collect();
    assert_eq!(
        global
            .iter()
            .map(|r| r.last_name.as_str())
            .collect::<Vec<_>>(),
        ["Third", "Target", "Fifth"]
    );
    assert_eq!(
        global.iter().map(|r| r.place).collect::<Vec<_>>(),
        [3, 4, 5]
    );
    assert_eq!(
        global.iter().map(|r| r.gap).collect::<Vec<_>>(),
        [decimal("10"), decimal("0"), decimal("-10")]
    );
    assert!(global.iter().all(|r| r.field == 6));
    assert_eq!(global[1].competition_slug, "board");
    let national: Vec<_> = rows.iter().filter(|r| r.scope == "national").collect();
    assert_eq!(
        national
            .iter()
            .map(|r| r.last_name.as_str())
            .collect::<Vec<_>>(),
        ["Third", "Target", "Fifth"]
    );
    assert_eq!(national[1].place, 3);
    assert_eq!(national[1].field, 5);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn total_standing_and_neighbors_exclude_other_sexes_even_in_a_shared_category(pool: PgPool) {
    let mut woman = lifting(athlete("Other", "Sex"), ["50", "80", "100", "200"]);
    woman.gender = osl_domain::Gender::F;
    import(
        &pool,
        common::competition(
            "shared",
            vec![men_80(vec![
                lifting(athlete("Profile", "Target"), ["20", "70", "100", "200"]),
                woman,
            ])],
        ),
    )
    .await;
    let repo = RankingRepository::new(&pool);
    let athlete_id = id(&pool, "Target").await;
    let neighbors = repo.get_athlete_total_neighbors(athlete_id).await.unwrap();
    assert!(
        neighbors
            .iter()
            .all(|r| r.athlete_id == athlete_id && r.field == 1)
    );
    let standings = repo.get_athlete_metric_standings(athlete_id).await.unwrap();
    let total = standings
        .iter()
        .find(|s| s.metric == osl_db::params::RankingMovement::Total)
        .unwrap();
    assert_eq!((total.global_place, total.global_field), (1, 1));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn uses_best_total_in_latest_all4_class_and_drops_rivals_who_moved_class(pool: PgPool) {
    use common::{category, weighing};
    use osl_domain::{Movement, WeightClassSlug};
    import(
        &pool,
        common::competition(
            "old-light",
            vec![category(
                WeightClassSlug::M66,
                vec![weighing(
                    lifting(athlete("Profile", "Target"), ["50", "80", "120", "200"]),
                    "66",
                )],
            )],
        ),
    )
    .await;
    let mut first = common::competition(
        "heavy-best",
        vec![men_80(vec![
            lifting(athlete("Profile", "Target"), ["30", "70", "100", "200"]),
            lifting(athlete("Past", "Rival"), ["40", "70", "100", "200"]),
        ])],
    );
    first.competition.start_date = chrono::NaiveDate::from_ymd_opt(2026, 2, 1).unwrap();
    first.competition.end_date = first.competition.start_date;
    import(&pool, first).await;
    let mut latest = common::competition(
        "latest-heavy",
        vec![
            men_80(vec![lifting(
                athlete("Profile", "Target"),
                ["20", "70", "100", "200"],
            )]),
            category(
                WeightClassSlug::M87,
                vec![lifting(
                    athlete("Past", "Rival"),
                    ["50", "80", "120", "200"],
                )],
            ),
        ],
    );
    latest.competition.start_date = chrono::NaiveDate::from_ymd_opt(2026, 3, 1).unwrap();
    latest.competition.end_date = latest.competition.start_date;
    import(&pool, latest).await;
    let mut partial = common::competition(
        "newest-pull",
        vec![category(
            WeightClassSlug::M66,
            vec![common::best(
                athlete("Profile", "Target"),
                Movement::PullUp,
                "100",
            )],
        )],
    );
    partial.movements = vec![Movement::PullUp];
    partial.competition.start_date = chrono::NaiveDate::from_ymd_opt(2026, 4, 1).unwrap();
    partial.competition.end_date = partial.competition.start_date;
    import(&pool, partial).await;
    let repo = RankingRepository::new(&pool);
    let target = id(&pool, "Target").await;
    let rows = repo.get_athlete_total_neighbors(target).await.unwrap();
    let global: Vec<_> = rows.iter().filter(|r| r.scope == "global").collect();
    assert_eq!(
        global.iter().map(|r| r.athlete_id).collect::<Vec<_>>(),
        [target]
    );
    assert_eq!(global[0].value, decimal("400"));
    assert_eq!(global[0].competition_slug, "heavy-best");
    assert_eq!(global[0].weight_class_max, Some(decimal("80")));
    let mover = id(&pool, "Rival").await;
    let mover_rows = repo.get_athlete_total_neighbors(mover).await.unwrap();
    assert!(
        mover_rows
            .iter()
            .all(|r| r.athlete_id == mover && r.weight_class_max == Some(decimal("87")))
    );
    let standings = repo.get_athlete_metric_standings(target).await.unwrap();
    let total = standings
        .iter()
        .find(|s| s.metric == osl_db::params::RankingMovement::Total)
        .unwrap();
    assert_eq!(
        (total.value, total.global_place, total.global_field),
        (decimal("400"), 2, 2)
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn ties_keep_stable_positions_and_zero_gaps_at_both_boundaries(pool: PgPool) {
    import(
        &pool,
        common::competition(
            "ties",
            vec![men_80(
                ["One", "Two", "Three", "Four", "Five"]
                    .into_iter()
                    .map(|last| lifting(athlete("Tied", last), ["20", "70", "100", "200"]))
                    .collect(),
            )],
        ),
    )
    .await;
    let mut ids = Vec::new();
    for last in ["One", "Two", "Three", "Four", "Five"] {
        ids.push(id(&pool, last).await);
    }
    ids.sort();
    let repo = RankingRepository::new(&pool);
    for (index, target) in ids.iter().enumerate() {
        let rows = repo.get_athlete_total_neighbors(*target).await.unwrap();
        let global: Vec<_> = rows.iter().filter(|r| r.scope == "global").collect();
        let start = index.saturating_sub(1);
        let end = (index + 2).min(ids.len());
        assert_eq!(
            global.iter().map(|r| r.athlete_id).collect::<Vec<_>>(),
            ids[start..end]
        );
        assert!(global.iter().all(|r| r.gap == decimal("0")));
        assert_eq!(
            global
                .iter()
                .find(|r| r.athlete_id == *target)
                .unwrap()
                .place,
            i64::try_from(index + 1).unwrap()
        );
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn unknown_nationality_has_global_context_without_a_national_pool(pool: PgPool) {
    let mut target = lifting(athlete("Profile", "Unknown"), ["20", "70", "100", "200"]);
    target.country = None;
    import(
        &pool,
        common::competition("alone", vec![men_80(vec![target])]),
    )
    .await;
    let rows = RankingRepository::new(&pool)
        .get_athlete_total_neighbors(id(&pool, "Unknown").await)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].scope, "global");
    assert_eq!((rows[0].place, rows[0].field), (1, 1));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn no_eligible_total_in_latest_all4_class_returns_no_neighbors_or_total_standing(
    pool: PgPool,
) {
    import(
        &pool,
        common::competition(
            "earlier",
            vec![men_80(vec![lifting(
                athlete("Profile", "Target"),
                ["20", "70", "100", "200"],
            )])],
        ),
    )
    .await;
    let mut latest = common::competition(
        "new-class",
        vec![common::category(
            osl_domain::WeightClassSlug::M87,
            vec![common::best(
                athlete("Profile", "Target"),
                osl_domain::Movement::PullUp,
                "80",
            )],
        )],
    );
    latest.competition.start_date = chrono::NaiveDate::from_ymd_opt(2026, 2, 1).unwrap();
    latest.competition.end_date = latest.competition.start_date;
    import(&pool, latest).await;
    let repo = RankingRepository::new(&pool);
    let target = id(&pool, "Target").await;
    assert!(
        repo.get_athlete_total_neighbors(target)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(
        repo.get_athlete_metric_standings(target)
            .await
            .unwrap()
            .iter()
            .all(|r| r.metric != osl_db::params::RankingMovement::Total)
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn includes_same_sex_and_weight_bound_from_mixed_competition_categories(pool: PgPool) {
    let mut mixed = common::category(
        osl_domain::WeightClassSlug::M80,
        vec![lifting(
            athlete("Mixed", "Rival"),
            ["40", "70", "100", "200"],
        )],
    );
    mixed.gender = osl_domain::Gender::Mx;
    mixed.weight_class_slug = None;
    mixed.weight_class_max = Some(decimal("80"));
    import(
        &pool,
        common::competition(
            "mixed",
            vec![
                men_80(vec![lifting(
                    athlete("Profile", "Target"),
                    ["20", "70", "100", "200"],
                )]),
                mixed,
            ],
        ),
    )
    .await;
    let repo = RankingRepository::new(&pool);
    let target = id(&pool, "Target").await;
    let rows = repo.get_athlete_total_neighbors(target).await.unwrap();
    let global: Vec<_> = rows.iter().filter(|r| r.scope == "global").collect();
    assert_eq!(
        global.len(),
        2,
        "same gender and numeric weight category as the main leaderboard"
    );
    assert_eq!(global[0].last_name, "Rival");
    let standings = repo.get_athlete_metric_standings(target).await.unwrap();
    let total = standings
        .iter()
        .find(|s| s.metric == osl_db::params::RankingMovement::Total)
        .unwrap();
    assert_eq!((total.global_place, total.global_field), (2, 2));
}
