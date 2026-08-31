use osl_db::repository::ranking::RankingRepository;
use osl_domain::Movement;
use osl_importer::canonical::models::{AthleteData, CanonicalFormat};
use sqlx::PgPool;
use uuid::Uuid;

mod common;

use common::{athlete, category, from, import, lifting, men_80, weighing};

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

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_place_comes_with_the_field_it_was_taken_in(pool: PgPool) {
    a_board(&pool).await;

    let standing = RankingRepository::new(&pool)
        .get_athlete_standing(athlete_id(&pool, "French").await)
        .await
        .expect("standing should succeed")
        .expect("a four movement total is ranked");

    assert_eq!(standing.global_place, 1);
    assert_eq!(standing.global_field, 3, "everyone the board ranks");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn the_country_place_only_counts_that_country(pool: PgPool) {
    a_board(&pool).await;
    let repo = RankingRepository::new(&pool);

    let italian = repo
        .get_athlete_standing(athlete_id(&pool, "Italian").await)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(italian.global_place, 2, "second best total overall");
    assert_eq!(italian.country, "IT");
    assert_eq!(
        (italian.country_place, italian.country_field),
        (1, 1),
        "the only Italian on the board is first of one"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn an_athlete_the_board_does_not_rank_has_no_standing(pool: PgPool) {
    let mut half: CanonicalFormat = common::competition(
        "half-event",
        vec![men_80(vec![two_lifts(athlete("No", "Score"))])],
    );
    half.movements = vec![Movement::MuscleUp, Movement::PullUp];
    import(&pool, half).await;

    let standing = RankingRepository::new(&pool)
        .get_athlete_standing(athlete_id(&pool, "Score").await)
        .await
        .expect("standing should succeed");

    assert!(standing.is_none(), "no four movement total, no place");
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
async fn a_class_place_only_counts_that_class(pool: PgPool) {
    two_classes(&pool).await;
    let repo = RankingRepository::new(&pool);

    let heavier = repo
        .get_athlete_class_standing(athlete_id(&pool, "French").await)
        .await
        .unwrap()
        .expect("a four movement total is ranked in its class");

    assert_eq!(
        (heavier.class_place, heavier.class_field),
        (1, 3),
        "three lifters in the class, the lighter one is not one of them"
    );

    let lighter = repo
        .get_athlete_class_standing(athlete_id(&pool, "Class").await)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        (lighter.class_place, lighter.class_field),
        (1, 1),
        "alone in his own class, whatever he totalled"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn the_class_country_place_narrows_both_ways(pool: PgPool) {
    two_classes(&pool).await;

    let italian = RankingRepository::new(&pool)
        .get_athlete_class_standing(athlete_id(&pool, "Italian").await)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(italian.class_place, 2, "second best total in the class");
    assert_eq!(
        (italian.class_country_place, italian.class_country_field),
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

    let standing = RankingRepository::new(&pool)
        .get_athlete_class_standing(athlete_id(&pool, "Up").await)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        standing.class_field, 4,
        "the class of their best total, counting everyone who has lifted in it"
    );
    assert_eq!(
        standing.class_place, 3,
        "370 sits behind the two bigger totals in that class"
    );
}
