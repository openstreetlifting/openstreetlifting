//! Who a result belongs to is decided by the folded name, so the same person
//! spelled three ways stays one athlete and two people sharing a name stay two.
//! Both failures used to be silent, which is why they are pinned down here.

use osl_domain::Movement;
use osl_importer::canonical::models::{AthleteData, CanonicalFormat};
use sqlx::PgPool;

mod common;

use common::{attempts, from, import, men_80, numbered, weighing};

fn athlete(first: &str, last: &str, disambiguation: Option<i16>) -> AthleteData {
    let lifter = weighing(common::athlete(first, last), "79");
    let lifter = attempts(lifter, Movement::MuscleUp, &[("60", true)]);

    match disambiguation {
        Some(number) => numbered(lifter, number),
        None => lifter,
    }
}

fn file(slug: &str, athletes: Vec<AthleteData>) -> CanonicalFormat {
    let mut canonical = common::competition(slug, vec![men_80(athletes)]);
    canonical.movements = vec![Movement::MuscleUp];
    canonical
}

async fn athlete_count(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM athletes")
        .fetch_one(pool)
        .await
        .unwrap()
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn one_person_spelled_three_ways_stays_one_athlete(pool: PgPool) {
    import(
        &pool,
        file("competition-one", vec![athlete("Lea", "MERANDON", None)]),
    )
    .await;
    import(
        &pool,
        file("competition-two", vec![athlete("Léa", "Mérandon", None)]),
    )
    .await;
    import(
        &pool,
        file("competition-three", vec![athlete("léa", "merandon", None)]),
    )
    .await;

    assert_eq!(athlete_count(&pool).await, 1);

    let participations: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM competition_participants cp
         JOIN athletes a USING (athlete_id) WHERE a.match_key = 'lea merandon'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(participations, 3, "all three results belong to one person");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn the_stored_name_keeps_the_spelling_the_file_used(pool: PgPool) {
    import(
        &pool,
        file("competition-one", vec![athlete("Léa", "Mérandon", None)]),
    )
    .await;

    let (first, last): (String, String) =
        sqlx::query_as("SELECT first_name, last_name FROM athletes")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!((first.as_str(), last.as_str()), ("Léa", "Mérandon"));

    // The slug is folded, so the URL carries no accent.
    let slug: String = sqlx::query_scalar("SELECT slug FROM athletes")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(slug, "lea-merandon");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn two_people_sharing_a_name_stay_apart(pool: PgPool) {
    import(
        &pool,
        file(
            "competition-one",
            vec![
                athlete("Tom", "Berthier", None),
                athlete("Tom", "Berthier", Some(2)),
            ],
        ),
    )
    .await;

    assert_eq!(athlete_count(&pool).await, 2);

    // And they stay apart on a later import that mentions only one of them.
    import(
        &pool,
        file("competition-two", vec![athlete("Tom", "Berthier", Some(2))]),
    )
    .await;
    assert_eq!(athlete_count(&pool).await, 2);

    let second: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM competition_participants cp
         JOIN athletes a USING (athlete_id) WHERE a.disambiguation = 2",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(second, 2);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn punctuation_does_not_create_a_second_athlete(pool: PgPool) {
    import(
        &pool,
        file(
            "competition-one",
            vec![athlete("Jean-Luc", "O'Brien", None)],
        ),
    )
    .await;
    import(
        &pool,
        file("competition-two", vec![athlete("Jean Luc", "OBrien", None)]),
    )
    .await;

    assert_eq!(athlete_count(&pool).await, 1);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_reimport_creates_no_new_athlete(pool: PgPool) {
    import(
        &pool,
        file("competition-one", vec![athlete("Lea", "Merandon", None)]),
    )
    .await;
    let before = athlete_count(&pool).await;

    import(
        &pool,
        file("competition-one", vec![athlete("Lea", "Merandon", None)]),
    )
    .await;

    assert_eq!(athlete_count(&pool).await, before);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn the_database_refuses_a_duplicate_identity(pool: PgPool) {
    import(
        &pool,
        file("competition-one", vec![athlete("Lea", "Merandon", None)]),
    )
    .await;

    // Bypasses the importer to prove the constraint holds regardless of caller.
    let inserted = sqlx::query(
        "INSERT INTO athletes (first_name, last_name, gender, country, slug, match_key)
         VALUES ('Lea', 'Merandon', 'M', 'FR', 'lea-merandon-2', 'lea merandon')",
    )
    .execute(&pool)
    .await;

    assert!(
        inserted.is_err(),
        "a second row with the same identity must be rejected"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn athletes_from_different_countries_stay_apart(pool: PgPool) {
    let spanish = from(athlete("Jose", "Garcia", None), "ES");
    let mexican = from(athlete("Jose", "Garcia", None), "MX");

    import(&pool, file("competition-one", vec![spanish, mexican])).await;

    assert_eq!(athlete_count(&pool).await, 2);
}
