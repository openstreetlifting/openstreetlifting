use osl_db::repository::competition::CompetitionRepository;
use osl_domain::Gender;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

async fn competition(pool: &PgPool) -> Uuid {
    let federation_id: Uuid = sqlx::query_scalar(
        "INSERT INTO federations (name) VALUES ('Munich Underground') RETURNING federation_id",
    )
    .fetch_one(pool)
    .await
    .unwrap();

    sqlx::query_scalar(
        "INSERT INTO competitions (name, slug, status, federation_id, start_date, end_date, country)
         VALUES ('Underground', 'underground', 'completed', $1, '2026-01-01', '2026-01-01', 'DE')
         RETURNING competition_id",
    )
    .bind(federation_id)
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn athlete(pool: &PgPool, name: &str, gender: &str) -> Uuid {
    sqlx::query_scalar(
        "INSERT INTO athletes (first_name, last_name, gender, country, slug, match_key)
         VALUES ('Test', $1, $2, 'DE', $1, $1)
         RETURNING athlete_id",
    )
    .bind(name)
    .bind(gender)
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn enter(
    pool: &PgPool,
    competition_id: Uuid,
    athlete_id: Uuid,
    ris: Option<i32>,
) -> sqlx::Result<Uuid> {
    sqlx::query_scalar(
        "INSERT INTO competition_participants
             (competition_id, weight_class_id, athlete_id, status, ris_score, ris_source)
         VALUES ($1, NULL, $2, 'competed', $3,
                 CASE WHEN $3::numeric IS NULL THEN NULL ELSE 'reported' END)
         RETURNING participant_id",
    )
    .bind(competition_id)
    .bind(athlete_id)
    .bind(ris.map(Decimal::from))
    .fetch_one(pool)
    .await
}

#[sqlx::test(migrations = "./migrations")]
async fn a_participant_can_have_no_weight_class(pool: PgPool) {
    let competition_id = competition(&pool).await;
    let athlete_id = athlete(&pool, "Memmer", "M").await;

    enter(&pool, competition_id, athlete_id, Some(84))
        .await
        .unwrap();

    let stored: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM competition_participants WHERE weight_class_id IS NULL",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(stored, 1);
}

#[sqlx::test(migrations = "./migrations")]
async fn two_classless_lifters_share_one_competition(pool: PgPool) {
    let competition_id = competition(&pool).await;
    let first = athlete(&pool, "Memmer", "M").await;
    let second = athlete(&pool, "Topic", "M").await;

    enter(&pool, competition_id, first, Some(84)).await.unwrap();
    enter(&pool, competition_id, second, Some(55))
        .await
        .unwrap();
}

#[sqlx::test(migrations = "./migrations")]
async fn the_same_lifter_cannot_enter_one_classless_contest_twice(pool: PgPool) {
    let competition_id = competition(&pool).await;
    let athlete_id = athlete(&pool, "Memmer", "M").await;

    enter(&pool, competition_id, athlete_id, Some(84))
        .await
        .unwrap();

    assert!(
        enter(&pool, competition_id, athlete_id, Some(84))
            .await
            .is_err()
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn men_and_women_are_separate_contests(pool: PgPool) {
    let competition_id = competition(&pool).await;
    let man = athlete(&pool, "Memmer", "M").await;
    let woman = athlete(&pool, "Weber", "F").await;

    enter(&pool, competition_id, man, Some(84)).await.unwrap();
    enter(&pool, competition_id, woman, Some(83)).await.unwrap();

    let detail = CompetitionRepository::new(&pool)
        .find_by_id_detailed(competition_id)
        .await
        .unwrap();

    assert_eq!(detail.categories.len(), 2);
    for contest in &detail.categories {
        assert_eq!(contest.category.weight_class_id, None);
        assert_eq!(contest.participants.len(), 1);
        assert_eq!(contest.participants[0].rank, Some(1));
    }

    let genders: Vec<Gender> = detail
        .categories
        .iter()
        .map(|contest| contest.category.gender)
        .collect();
    assert!(genders.contains(&Gender::M));
    assert!(genders.contains(&Gender::F));
}

#[sqlx::test(migrations = "./migrations")]
async fn a_classless_contest_ranks_on_ris(pool: PgPool) {
    let competition_id = competition(&pool).await;
    let lighter = athlete(&pool, "Topic", "M").await;
    let heavier = athlete(&pool, "Memmer", "M").await;

    enter(&pool, competition_id, lighter, Some(90))
        .await
        .unwrap();
    enter(&pool, competition_id, heavier, Some(70))
        .await
        .unwrap();

    let detail = CompetitionRepository::new(&pool)
        .find_by_id_detailed(competition_id)
        .await
        .unwrap();

    let contest = &detail.categories[0];
    let winner = contest
        .participants
        .iter()
        .find(|participant| participant.rank == Some(1))
        .unwrap();

    assert_eq!(winner.ris_score, Some(Decimal::from(90)));
    assert_eq!(contest.participants.len(), 2);
}
