use osl_db::params::{RankingFilter, RankingMovement, SortDirection};
use osl_db::projections::ranking::AthleteMetricStandingRow;
use osl_db::repository::ranking::RankingRepository;
use osl_domain::{Gender, Movement};
use osl_importer::canonical::models::{AthleteData, CanonicalFormat};
use sqlx::PgPool;
use uuid::Uuid;

mod common;

use common::{athlete, category, from, import, lifting, men_80, open_category, weighing};

async fn a_board(pool: &PgPool) {
    import(
        pool,
        common::competition(
            "worlds",
            vec![men_80(vec![
                weighing(
                    lifting(athlete("Best", "French"), ["60", "90", "110", "180"]),
                    "80",
                ),
                weighing(
                    lifting(athlete("Second", "French"), ["40", "70", "90", "150"]),
                    "80",
                ),
                weighing(
                    from(
                        lifting(athlete("Only", "Italian"), ["50", "80", "100", "160"]),
                        "IT",
                    ),
                    "80",
                ),
            ])],
        ),
    )
    .await;
}

async fn athlete_id(pool: &PgPool, last_name: &str) -> Uuid {
    sqlx::query_scalar("SELECT athlete_id FROM athletes WHERE last_name = $1")
        .bind(last_name)
        .fetch_one(pool)
        .await
        .expect("the imported athlete should exist")
}

async fn standings(pool: &PgPool, last_name: &str) -> Vec<AthleteMetricStandingRow> {
    RankingRepository::new(pool)
        .get_athlete_metric_standings(athlete_id(pool, last_name).await)
        .await
        .expect("standings should succeed")
}

fn metric(
    standings: &[AthleteMetricStandingRow],
    wanted: RankingMovement,
) -> &AthleteMetricStandingRow {
    standings
        .iter()
        .find(|standing| standing.metric == wanted)
        .unwrap_or_else(|| panic!("{wanted:?} should be ranked"))
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_place_comes_with_the_field_it_was_taken_in(pool: PgPool) {
    a_board(&pool).await;

    let standings = standings(&pool, "French").await;
    let standing = metric(&standings, RankingMovement::Ris);

    assert_eq!(standing.global_place, 1);
    assert_eq!(standing.global_field, 3, "everyone with a RIS score");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn the_country_place_only_counts_that_country(pool: PgPool) {
    a_board(&pool).await;
    let standings = standings(&pool, "Italian").await;
    let italian = metric(&standings, RankingMovement::Ris);

    assert_eq!(italian.global_place, 2, "second best RIS score overall");
    assert_eq!(italian.country, "IT");
    assert_eq!(
        (italian.country_place, italian.country_field),
        (1, 1),
        "the only Italian on the board is first of one"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn every_ranking_metric_has_a_global_and_country_place(pool: PgPool) {
    a_board(&pool).await;
    let standings = standings(&pool, "Italian").await;

    assert_eq!(standings.len(), 6, "RIS, total and all four movements");

    let total = metric(&standings, RankingMovement::Total);
    assert_eq!(total.value.to_string(), "390");
    assert_eq!((total.global_place, total.global_field), (2, 3));
    assert_eq!((total.country_place, total.country_field), (1, 1));

    let muscleup = metric(&standings, RankingMovement::Muscleup);
    assert_eq!(muscleup.value.to_string(), "50");
    assert_eq!((muscleup.global_place, muscleup.global_field), (2, 3));
    assert_eq!((muscleup.country_place, muscleup.country_field), (1, 1));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_partial_event_only_ranks_its_movements(pool: PgPool) {
    let mut half: CanonicalFormat = common::competition(
        "half-event",
        vec![men_80(vec![two_lifts(athlete("No", "Score"))])],
    );
    half.movements = vec![Movement::MuscleUp, Movement::PullUp];
    import(&pool, half).await;

    let metrics = standings(&pool, "Score").await;
    assert_eq!(
        metrics
            .iter()
            .map(|standing| standing.metric.as_str())
            .collect::<Vec<_>>(),
        vec!["muscleup", "pullup"],
        "individual lifts rank across partial events, but their total and RIS do not"
    );
}

fn two_lifts(lifter: AthleteData) -> AthleteData {
    common::attempts(
        common::attempts(lifter, Movement::MuscleUp, &[("40", true)]),
        Movement::PullUp,
        &[("60", true)],
    )
}

async fn two_classes(pool: &PgPool) {
    a_board(pool).await;
    import(
        pool,
        common::competition(
            "lighter",
            vec![category(
                osl_domain::WeightClassSlug::M66,
                vec![weighing(
                    lifting(athlete("Lighter", "Class"), ["55", "85", "105", "175"]),
                    "66",
                )],
            )],
        ),
    )
    .await;
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn metric_places_only_compare_the_athletes_weight_class(pool: PgPool) {
    two_classes(&pool).await;
    let heavier = standings(&pool, "French").await;
    let heavier_total = metric(&heavier, RankingMovement::Total);
    assert_eq!(
        (heavier_total.global_place, heavier_total.global_field),
        (1, 3),
        "the lighter class is excluded"
    );
    let heavier_ris = metric(&heavier, RankingMovement::Ris);
    assert_eq!(
        heavier_ris.global_field, 4,
        "RIS still compares every weight class"
    );

    let lighter = standings(&pool, "Class").await;
    let lighter_total = metric(&lighter, RankingMovement::Total);
    assert_eq!(
        (lighter_total.global_place, lighter_total.global_field),
        (1, 1),
        "an athlete alone in their class is first of one"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn metric_places_do_not_compare_men_and_women(pool: PgPool) {
    let mut woman = athlete("Strong", "Woman");
    woman.gender = Some(Gender::F);

    import(
        &pool,
        common::competition(
            "open-classes",
            vec![
                open_category(
                    Gender::M,
                    "80",
                    vec![weighing(
                        lifting(athlete("Comparable", "Man"), ["20", "30", "40", "50"]),
                        "81",
                    )],
                ),
                open_category(
                    Gender::F,
                    "80",
                    vec![weighing(lifting(woman, ["40", "60", "80", "100"]), "81")],
                ),
            ],
        ),
    )
    .await;

    for last_name in ["Man", "Woman"] {
        let standings = standings(&pool, last_name).await;
        let total = metric(&standings, RankingMovement::Total);
        assert_eq!(
            (total.global_place, total.global_field),
            (1, 1),
            "{last_name} only ranks against the same sex"
        );

        let ris = metric(&standings, RankingMovement::Ris);
        assert_eq!(
            (ris.global_field, ris.country_field),
            (2, 2),
            "RIS remains global across sex in both scopes"
        );
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_class_place_only_counts_that_class(pool: PgPool) {
    two_classes(&pool).await;
    let heavier = standings(&pool, "French").await;
    let heavier = metric(&heavier, RankingMovement::Total);

    assert_eq!(
        (heavier.global_place, heavier.global_field),
        (1, 3),
        "three lifters in the class, the lighter one is not one of them"
    );

    let lighter = standings(&pool, "Class").await;
    let lighter = metric(&lighter, RankingMovement::Total);

    assert_eq!(
        (lighter.global_place, lighter.global_field),
        (1, 1),
        "alone in his own class, whatever he totalled"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn the_class_country_place_narrows_both_ways(pool: PgPool) {
    two_classes(&pool).await;

    let italian = standings(&pool, "Italian").await;
    let italian = metric(&italian, RankingMovement::Total);

    assert_eq!(italian.global_place, 2, "second best total in the class");
    assert_eq!(
        (italian.country_place, italian.country_field),
        (1, 1),
        "the only Italian in that class"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_class_counts_everyone_who_has_lifted_in_it(pool: PgPool) {
    a_board(&pool).await;
    import(
        &pool,
        common::competition(
            "moved-up",
            vec![category(
                osl_domain::WeightClassSlug::M66,
                vec![weighing(
                    lifting(athlete("Moved", "Up"), ["30", "60", "80", "120"]),
                    "66",
                )],
            )],
        ),
    )
    .await;
    import(
        &pool,
        common::competition(
            "moved-up-heavier",
            vec![men_80(vec![weighing(
                lifting(athlete("Moved", "Up"), ["45", "75", "95", "155"]),
                "80",
            )])],
        ),
    )
    .await;

    let standings = standings(&pool, "Up").await;
    let standing = metric(&standings, RankingMovement::Total);

    assert_eq!(
        standing.global_field, 4,
        "the class of their best total, counting everyone who has lifted in it"
    );
    assert_eq!(
        standing.global_place, 3,
        "370 sits behind the two bigger totals in that class"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn tied_card_places_match_both_linked_leaderboards(pool: PgPool) {
    import(
        &pool,
        common::competition(
            "tied",
            vec![men_80(vec![
                lifting(athlete("Alice", "French"), ["40", "60", "80", "120"]),
                from(
                    lifting(athlete("Bella", "Italian"), ["40", "60", "80", "120"]),
                    "IT",
                ),
                lifting(athlete("Clara", "French"), ["40", "60", "80", "120"]),
                from(
                    lifting(athlete("Diana", "Italian"), ["40", "60", "80", "120"]),
                    "IT",
                ),
            ])],
        ),
    )
    .await;
    let repo = RankingRepository::new(&pool);
    for movement in RankingMovement::ALL {
        for country in [None, Some("FR"), Some("IT")] {
            let filter = RankingFilter {
                gender: (movement != RankingMovement::Ris).then_some(osl_domain::Gender::M),
                category: (movement != RankingMovement::Ris)
                    .then(|| osl_domain::WeightClass::UpTo(common::decimal("80"))),
                country: country.map(str::to_string),
                federation: None,
                name: None,
                movement,
                direction: SortDirection::Desc,
                event: osl_domain::FULL_EVENT.to_string(),
                year: None,
                competition_id: None,
                offset: 0,
                limit: 50,
            };
            let (rows, field) = repo.get_global_ranking(&filter).await.unwrap();
            let ids: Vec<_> = rows.iter().map(|row| row.athlete_id).collect();
            let mut ordered_ids = ids.clone();
            ordered_ids.sort();
            assert_eq!(ids, ordered_ids, "ties use a stable athlete order");
            for row in rows {
                let standings = repo
                    .get_athlete_metric_standings(row.athlete_id)
                    .await
                    .unwrap();
                let standing = metric(&standings, movement);
                let actual = if country.is_some() {
                    (standing.country_place, standing.country_field)
                } else {
                    (standing.global_place, standing.global_field)
                };
                assert_eq!(
                    actual,
                    (row.rank, field),
                    "{movement:?} card must match its board"
                );
            }
        }
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn equally_good_results_choose_the_latest_weight_class(pool: PgPool) {
    let lifter = || lifting(athlete("Moving", "Class"), ["40", "60", "80", "120"]);
    import(
        &pool,
        common::competition(
            "earlier",
            vec![category(
                osl_domain::WeightClassSlug::M66,
                vec![weighing(lifter(), "66")],
            )],
        ),
    )
    .await;
    let mut later = common::competition("later", vec![men_80(vec![lifter()])]);
    later.competition.start_date = chrono::NaiveDate::from_ymd_opt(2026, 2, 1).unwrap();
    later.competition.end_date = later.competition.start_date;
    import(&pool, later).await;

    let standings = standings(&pool, "Class").await;
    for movement in RankingMovement::ALL
        .into_iter()
        .filter(|m| *m != RankingMovement::Ris)
    {
        let standing = metric(&standings, movement);
        assert_eq!(
            standing.weight_class_max,
            Some(common::decimal("80")),
            "{movement:?}"
        );
        assert_eq!((standing.global_place, standing.global_field), (1, 1));
    }
}
