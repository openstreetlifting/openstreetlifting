use osl_db::params::{RankingFilter, RankingMovement, SortDirection};
use osl_db::repository::ranking::RankingRepository;
use sqlx::PgPool;

mod common;

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn ranks_best_result_within_selected_series_before_search_and_pagination(pool: PgPool) {
    for (slug, series, year, first, last, muscleup) in [
        ("sdm-old", Some("sdm"), 2025, "Ada", "Twice", "20"),
        ("sdm-new", Some("sdm"), 2026, "Ada", "Twice", "30"),
        ("sdm-other", Some("sdm"), 2026, "Grace", "Other", "25"),
        ("wdm", Some("wdm"), 2026, "Ada", "Twice", "50"),
        ("unassigned", None, 2026, "Alan", "Outside", "60"),
    ] {
        let mut meet = common::competition(
            slug,
            vec![common::men_80(vec![common::lifting(
                common::athlete(first, last),
                [muscleup, "60", "100", "150"],
            )])],
        );
        meet.competition.series = series.map(str::to_owned);
        meet.competition.start_date = chrono::NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        meet.competition.end_date = meet.competition.start_date;
        common::import(&pool, meet).await;
    }
    let mut filter = RankingFilter {
        gender: None,
        country: None,
        federation: None,
        series: Some("sdm".into()),
        name: None,
        movement: RankingMovement::Total,
        direction: SortDirection::Desc,
        event: osl_domain::FULL_EVENT.into(),
        category: None,
        year: None,
        competition_id: None,
        offset: 0,
        limit: 1,
    };
    let repo = RankingRepository::new(&pool);
    let (rows, count) = repo.get_global_ranking(&filter).await.unwrap();
    assert_eq!(count, 2);
    assert_eq!(rows[0].competition_slug, "sdm-new");
    assert_eq!(rows[0].rank, 1);

    filter.offset = 1;
    let (rows, count) = repo.get_global_ranking(&filter).await.unwrap();
    assert_eq!(count, 2);
    assert_eq!(rows[0].last_name, "Other");
    assert_eq!(rows[0].rank, 2);

    filter.offset = 0;
    filter.name = Some("Grace".into());
    let (rows, count) = repo.get_global_ranking(&filter).await.unwrap();
    assert_eq!(count, 1);
    assert_eq!(rows[0].rank, 2);

    filter.name = None;
    filter.year = Some(2025);
    let (rows, count) = repo.get_global_ranking(&filter).await.unwrap();
    assert_eq!(count, 1);
    assert_eq!(rows[0].competition_slug, "sdm-old");

    filter.year = None;
    filter.series = Some("missing".into());
    let (rows, count) = repo.get_global_ranking(&filter).await.unwrap();
    assert!(rows.is_empty());
    assert_eq!(count, 0);

    let series = repo.list_distinct_series().await.unwrap();
    assert_eq!(
        series
            .iter()
            .map(|(id, _, _)| id.as_str())
            .collect::<Vec<_>>(),
        vec!["sdm", "wdm"]
    );
    assert_eq!(series[0].1, "sdm-new");
}
