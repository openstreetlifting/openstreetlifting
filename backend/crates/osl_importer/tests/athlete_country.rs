use osl_db::params::{RankingFilter, RankingMovement, SortDirection};
use osl_db::repository::{athlete::AthleteRepository, ranking::RankingRepository};
use osl_domain::CountryCode;
use osl_importer::canonical::{
    models::CanonicalFormat, store, transformer::CanonicalTransformer,
    validator::CanonicalValidator,
};
use sqlx::PgPool;
use uuid::Uuid;

mod common;

fn meet(slug: &str, country: Option<&str>) -> CanonicalFormat {
    let mut athlete = common::lifting(
        common::athlete("Country", "Lifter"),
        ["20", "60", "90", "130"],
    );
    athlete.country = country.map(|code| CountryCode::parse(code).unwrap());
    athlete.total = Some(common::decimal("300"));
    common::competition(slug, vec![common::men_80(vec![athlete])])
}

async fn identity(pool: &PgPool) -> (Uuid, String, Option<String>) {
    sqlx::query_as("SELECT athlete_id, slug, country FROM athletes")
        .fetch_one(pool)
        .await
        .unwrap()
}

#[test]
fn empty_country_round_trips_without_using_the_host_country() {
    let canonical = meet("unknown", None);
    CanonicalValidator::validate(&canonical).unwrap();
    let directory = std::env::temp_dir().join(format!("osl-country-{}", Uuid::new_v4()));
    std::fs::create_dir_all(&directory).unwrap();
    store::write(&directory, &canonical).unwrap();
    let read = store::read(&directory).unwrap();
    assert_eq!(read.categories[0].athletes[0].country, None);
    assert_eq!(read.competition.country.as_str(), "FR");
    std::fs::remove_dir_all(directory).unwrap();
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn learning_and_correcting_country_preserves_identity_and_links(pool: PgPool) {
    let importer = CanonicalTransformer::new(&pool);
    importer
        .import_to_database(meet("first", None))
        .await
        .unwrap();
    let before = identity(&pool).await;
    assert_eq!(before.2, None);
    for country in [Some("FR"), Some("IT"), None] {
        importer
            .import_to_database(meet("first", country))
            .await
            .unwrap();
        let after = identity(&pool).await;
        assert_eq!((after.0, after.1), (before.0, before.1.clone()));
        assert_eq!(after.2.as_deref(), country);
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn missing_country_does_not_erase_evidence_from_another_result(pool: PgPool) {
    let importer = CanonicalTransformer::new(&pool);
    for reverse in [false, true] {
        let mut batch = vec![meet("known", Some("FR")), meet("unknown", None)];
        if reverse {
            batch.reverse();
        }
        importer.import_batch(batch, false).await.unwrap();
        assert_eq!(identity(&pool).await.2.as_deref(), Some("FR"));
        let countries: Vec<Option<String>> = sqlx::query_scalar(
            "SELECT country FROM competition_participants ORDER BY country NULLS LAST",
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(countries, vec![Some("FR".into()), None]);
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn conflicting_imports_roll_back_and_complete_corrections_are_atomic(pool: PgPool) {
    let importer = CanonicalTransformer::new(&pool);
    importer
        .import_batch(
            vec![meet("one", Some("FR")), meet("two", Some("FR"))],
            false,
        )
        .await
        .unwrap();
    let before = identity(&pool).await;
    assert!(
        importer
            .import_to_database(meet("one", Some("IT")))
            .await
            .unwrap_err()
            .to_string()
            .contains("Conflicting athlete countries")
    );
    assert_eq!(identity(&pool).await, before);
    let countries: Vec<String> = sqlx::query_scalar("SELECT country FROM competition_participants")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(countries, vec!["FR", "FR"]);
    assert!(
        importer
            .import_batch(
                vec![meet("three", Some("IT")), meet("four", Some("DE"))],
                false
            )
            .await
            .is_err()
    );
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM competitions")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 2);
    importer
        .import_batch(
            vec![meet("two", Some("IT")), meet("one", Some("IT"))],
            false,
        )
        .await
        .unwrap();
    let after = identity(&pool).await;
    assert_eq!((after.0, after.1), (before.0, before.1));
    assert_eq!(after.2.as_deref(), Some("IT"));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn pruning_removes_old_country_evidence_within_the_import(pool: PgPool) {
    let importer = CanonicalTransformer::new(&pool);
    importer
        .import_to_database(meet("old", Some("FR")))
        .await
        .unwrap();
    let before = identity(&pool).await;
    importer
        .import_batch(vec![meet("new", Some("IT"))], true)
        .await
        .unwrap();
    let after = identity(&pool).await;
    assert_eq!(after.0, before.0);
    assert_eq!(after.2.as_deref(), Some("IT"));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn unknown_country_keeps_global_rankings_and_profile_history(pool: PgPool) {
    CanonicalTransformer::new(&pool)
        .import_to_database(meet("unknown", None))
        .await
        .unwrap();
    let (id, slug, _) = identity(&pool).await;
    let repo = RankingRepository::new(&pool);
    let mut filter = RankingFilter {
        gender: None,
        country: None,
        federation: None,
        name: None,
        movement: RankingMovement::Total,
        direction: SortDirection::Desc,
        event: osl_domain::FULL_EVENT.into(),
        category: None,
        year: None,
        competition_id: None,
        offset: 0,
        limit: 10,
    };
    let (rows, count) = repo.get_global_ranking(&filter).await.unwrap();
    assert_eq!(count, 1);
    assert_eq!(rows[0].country, None);
    assert_eq!(rows[0].athlete_id, id);
    filter.country = Some("FR".into());
    assert_eq!(repo.get_global_ranking(&filter).await.unwrap().1, 0);
    assert!(repo.list_distinct_countries(None).await.unwrap().is_empty());
    let history = AthleteRepository::new(&pool)
        .find_by_slug_detailed(&slug)
        .await
        .unwrap();
    assert_eq!(history.athlete.country, None);
    assert_eq!(history.total_competitions, 1);
    assert_eq!(history.personal_records.len(), 4);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn identity_migration_preserves_existing_profiles_and_participants(pool: PgPool) {
    let importer = CanonicalTransformer::new(&pool);
    for (country, number) in [("FR", 1), ("US", 2)] {
        let mut canonical = meet(&format!("old-{country}"), Some(country));
        let athlete = &mut canonical.categories[0].athletes[0];
        athlete.first_name = "Tony".into();
        athlete.last_name = "Nguyen".into();
        athlete.disambiguation = Some(number);
        importer.import_to_database(canonical).await.unwrap();
    }
    sqlx::query("INSERT INTO athlete_socials (athlete_id, social_id, handle) SELECT a.athlete_id, s.social_id, 'tony_fr' FROM athletes a CROSS JOIN socials s WHERE a.country = 'FR' AND s.name = 'instagram'")
        .execute(&pool).await.unwrap();
    // Restore the former schema around populated profiles and their results.
    sqlx::raw_sql("ALTER TABLE competition_participants DROP COLUMN country;
        ALTER TABLE athletes ALTER COLUMN country SET NOT NULL;
        DROP INDEX athletes_identity_unique;
        CREATE UNIQUE INDEX athletes_identity_unique ON athletes (match_key, gender, country, disambiguation) NULLS NOT DISTINCT;
        UPDATE athletes SET disambiguation = NULL;")
        .execute(&pool).await.unwrap();
    let mut relations = Vec::new();
    for query in [
        "SELECT jsonb_agg(to_jsonb(t) - 'country' ORDER BY participant_id)::text FROM competition_participants t",
        "SELECT jsonb_agg(to_jsonb(t) ORDER BY to_jsonb(t)::text)::text FROM lifts t",
        "SELECT jsonb_agg(to_jsonb(t) ORDER BY to_jsonb(t)::text)::text FROM attempts t",
        "SELECT jsonb_agg(to_jsonb(t) ORDER BY to_jsonb(t)::text)::text FROM athlete_socials t",
    ] {
        let snapshot: Option<String> = sqlx::query_scalar(query).fetch_one(&pool).await.unwrap();
        relations.push((query, snapshot));
    }
    let before: Vec<(Uuid, String)> =
        sqlx::query_as("SELECT athlete_id, slug FROM athletes ORDER BY slug")
            .fetch_all(&pool)
            .await
            .unwrap();
    sqlx::raw_sql(include_str!(
        "../../osl_db/migrations/20260923160000_country_independent_identity.sql"
    ))
    .execute(&pool)
    .await
    .unwrap();
    let after: Vec<(Uuid, String)> =
        sqlx::query_as("SELECT athlete_id, slug FROM athletes ORDER BY slug")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(after, before);
    for (query, snapshot) in relations {
        let after: Option<String> = sqlx::query_scalar(query).fetch_one(&pool).await.unwrap();
        assert_eq!(after, snapshot);
    }
    let countries: Vec<String> =
        sqlx::query_scalar("SELECT country FROM competition_participants ORDER BY country")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(countries, vec!["FR", "US"]);
    let numbers: Vec<i16> =
        sqlx::query_scalar("SELECT disambiguation FROM athletes ORDER BY country")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(numbers, vec![1, 2]);
    let mut canonical = meet("migrated", Some("FR"));
    let athlete = &mut canonical.categories[0].athletes[0];
    athlete.first_name = "Tony".into();
    athlete.last_name = "Nguyen".into();
    athlete.disambiguation = Some(1);
    CanonicalTransformer::new(&pool)
        .import_to_database(canonical)
        .await
        .unwrap();
    let imported: Uuid = sqlx::query_scalar("SELECT cp.athlete_id FROM competition_participants cp JOIN competitions c USING (competition_id) WHERE c.slug = 'migrated'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(imported, before[0].0);
}
