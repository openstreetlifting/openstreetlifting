use osl_db::filter::Filters;
use osl_db::params::{RankingBy, RankingFilter, RankingMovement, SortDirection};
use osl_db::repository::ranking::RankingRepository;
use osl_domain::Gender;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

fn totals(by: impl Into<Filters<RankingBy>>) -> RankingFilter {
    RankingFilter {
        by: by.into(),
        name: None,
        movement: RankingMovement::Total,
        direction: SortDirection::Desc,
        event: osl_domain::FULL_EVENT.to_string(),
        offset: 0,
        limit: 100,
    }
}

struct Competition {
    competition_id: Uuid,
    federation_id: Uuid,
}

async fn competition(pool: &PgPool) -> Competition {
    let federation_id: Uuid = sqlx::query_scalar(
        "INSERT INTO federations (name) VALUES ('Test Federation') RETURNING federation_id",
    )
    .fetch_one(pool)
    .await
    .unwrap();

    let competition_id: Uuid = sqlx::query_scalar(
        "INSERT INTO competitions (name, slug, status, federation_id, start_date, end_date, country, event_code)
         VALUES ('Test Open', 'test-open', 'completed', $1, '2026-01-01', '2026-01-01', 'FR', 'MPDS')
         RETURNING competition_id",
    )
    .bind(federation_id)
    .fetch_one(pool)
    .await
    .unwrap();

    Competition {
        competition_id,
        federation_id,
    }
}

async fn enter(
    pool: &PgPool,
    competition: &Competition,
    gender: &str,
    min: Option<i32>,
    max: Option<i32>,
    division: Option<&str>,
) {
    let weight_class_id: Uuid = sqlx::query_scalar(
        "INSERT INTO weight_classes (gender, min_kg, max_kg) VALUES ($1, $2, $3)
         ON CONFLICT ON CONSTRAINT weight_class_bounds_unique DO UPDATE SET gender = EXCLUDED.gender
         RETURNING weight_class_id",
    )
    .bind(gender)
    .bind(min.map(Decimal::from))
    .bind(max.map(Decimal::from))
    .fetch_one(pool)
    .await
    .unwrap();

    let division_id: Option<Uuid> = match division {
        Some(name) => Some(
            sqlx::query_scalar(
                "INSERT INTO divisions (federation_id, name) VALUES ($1, $2)
                 ON CONFLICT ON CONSTRAINT division_name_unique_per_federation
                 DO UPDATE SET name = EXCLUDED.name
                 RETURNING division_id",
            )
            .bind(competition.federation_id)
            .bind(name)
            .fetch_one(pool)
            .await
            .unwrap(),
        ),
        None => None,
    };

    let handle = Uuid::new_v4().to_string();
    let athlete_id: Uuid = sqlx::query_scalar(
        "INSERT INTO athletes (first_name, last_name, gender, country, slug, match_key)
         VALUES ('Test', $1, $2, 'FR', $1, $1)
         RETURNING athlete_id",
    )
    .bind(&handle)
    .bind(gender)
    .fetch_one(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO competition_participants (competition_id, weight_class_id, division_id, athlete_id, status, total)
         VALUES ($1, $2, $3, $4, 'competed', 100)",
    )
    .bind(competition.competition_id)
    .bind(weight_class_id)
    .bind(division_id)
    .bind(athlete_id)
    .execute(pool)
    .await
    .unwrap();
}

#[sqlx::test(migrations = "./migrations")]
async fn lists_classes_lightest_first(pool: PgPool) {
    let competition = competition(&pool).await;
    enter(&pool, &competition, "M", Some(94), Some(101), None).await;
    enter(&pool, &competition, "M", None, Some(66), None).await;
    enter(&pool, &competition, "M", Some(101), None, None).await;
    enter(&pool, &competition, "M", Some(73), Some(80), None).await;

    let classes = RankingRepository::new(&pool)
        .facets(&totals([RankingBy::Sex(Gender::M)]))
        .await
        .unwrap()
        .classes;

    assert_eq!(classes, vec!["-66kg", "-80kg", "-101kg", "+101kg"]);
}

#[sqlx::test(migrations = "./migrations")]
async fn filters_classes_by_gender(pool: PgPool) {
    let competition = competition(&pool).await;
    enter(&pool, &competition, "M", Some(73), Some(80), None).await;
    enter(&pool, &competition, "F", Some(63), Some(70), None).await;

    let repository = RankingRepository::new(&pool);

    assert_eq!(
        repository
            .facets(&totals([RankingBy::Sex(Gender::F)]))
            .await
            .unwrap()
            .classes,
        vec!["-70kg"]
    );
    assert_eq!(
        repository
            .facets(&totals([RankingBy::Sex(Gender::M)]))
            .await
            .unwrap()
            .classes,
        vec!["-80kg"]
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn deduplicates_classes_across_divisions(pool: PgPool) {
    let competition = competition(&pool).await;
    enter(&pool, &competition, "M", Some(73), Some(80), Some("Elite")).await;
    enter(&pool, &competition, "M", Some(73), Some(80), Some("Open")).await;

    let classes = RankingRepository::new(&pool)
        .facets(&totals(Filters::default()))
        .await
        .unwrap()
        .classes;

    assert_eq!(classes, vec!["-80kg"]);
}

#[sqlx::test(migrations = "./migrations")]
async fn returns_empty_list_when_classes_are_absent(pool: PgPool) {
    let competition = competition(&pool).await;

    let athlete_id: Uuid = sqlx::query_scalar(
        "INSERT INTO athletes (first_name, last_name, gender, country, slug, match_key)
         VALUES ('Test', 'Memmer', 'M', 'DE', 'memmer', 'memmer')
         RETURNING athlete_id",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO competition_participants (competition_id, weight_class_id, athlete_id, status, total)
         VALUES ($1, NULL, $2, 'competed', 100)",
    )
    .bind(competition.competition_id)
    .bind(athlete_id)
    .execute(&pool)
    .await
    .unwrap();

    assert_eq!(
        RankingRepository::new(&pool)
            .facets(&totals(Filters::default()))
            .await
            .unwrap()
            .classes,
        [] as [String; 0]
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn leaves_unranked_entrants_out_of_the_classes(pool: PgPool) {
    let competition = competition(&pool).await;
    enter(&pool, &competition, "M", Some(73), Some(80), None).await;
    sqlx::query("UPDATE competition_participants SET status = 'disqualified', total = NULL")
        .execute(&pool)
        .await
        .unwrap();

    let repository = RankingRepository::new(&pool);

    assert_eq!(
        repository
            .facets(&totals(Filters::default()))
            .await
            .unwrap()
            .classes,
        [] as [String; 0]
    );
}
