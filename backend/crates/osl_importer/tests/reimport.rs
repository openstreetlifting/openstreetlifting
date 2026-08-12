//! A canonical file is built up over several passes, so importing a corrected
//! file has to land the correction rather than stop at the row already there.

use osl_domain::CompetitionStatus;
use osl_importer::canonical::{models::CanonicalFormat, transformer::CanonicalTransformer};
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::str::FromStr;

/// `weight_class` is the raw JSON for the class fields, so a test can pass a
/// slug or raw bounds without a second fixture.
fn canonical(weight_class: &str, nationality: Option<&str>) -> CanonicalFormat {
    scored(weight_class, nationality, r#""bodyweight": "72.5","#)
}

/// `score` is the raw JSON for bodyweight or ris, so a test can pick which.
fn scored(weight_class: &str, nationality: Option<&str>, score: &str) -> CanonicalFormat {
    let nationality = match nationality {
        Some(code) => format!(r#""nationality": "{}","#, code),
        None => String::new(),
    };

    serde_json::from_str(&format!(
        r#"{{
          "format_version": "1.5.0",
          "source": {{
            "type": "manual",
            "extracted_at": "2025-06-01T10:00:00Z",
            "extractor": "test"
          }},
          "competition": {{
            "name": "Test Open",
            "slug": "test-open",
            "federation": {{ "name": "Test Federation" }},
            "start_date": "2025-06-01",
            "end_date": "2025-06-01",
            "country": "FR"
          }},
          "movements": [
            {{ "name": "Muscle-up", "order": 1 }},
            {{ "name": "Pull-up", "order": 2 }},
            {{ "name": "Dips", "order": 3 }},
            {{ "name": "Squat", "order": 4 }}
          ],
          "categories": [
            {{
              "name": "Test category",
              "gender": "M",
              {}
              "athletes": [
                {{
                  "first_name": "John",
                  "last_name": "Doe",
                  "country": "FR",
                  {}
                  {}
                  "status": "competed",
                  "lifts": [
                    {{
                      "movement": "Muscle-up",
                      "attempts": [
                        {{ "attempt_number": 1, "weight": "50", "is_successful": true }}
                      ]
                    }},
                    {{
                      "movement": "Pull-up",
                      "attempts": [
                        {{ "attempt_number": 1, "weight": "60", "is_successful": true }}
                      ]
                    }},
                    {{
                      "movement": "Dips",
                      "attempts": [
                        {{ "attempt_number": 1, "weight": "80", "is_successful": true }}
                      ]
                    }},
                    {{
                      "movement": "Squat",
                      "attempts": [
                        {{ "attempt_number": 1, "weight": "120", "is_successful": true }}
                      ]
                    }}
                  ]
                }}
              ]
            }}
          ]
        }}"#,
        weight_class, nationality, score
    ))
    .expect("fixture is a valid canonical file")
}

fn slug(slug: &str) -> String {
    format!(r#""weight_class_slug": "{}","#, slug)
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_file_without_a_status_imports_as_completed(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(canonical(&slug("M-73"), Some("FR")))
        .await
        .unwrap();

    let status: String =
        sqlx::query_scalar!(r#"SELECT status FROM competitions WHERE slug = 'test-open'"#)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(status, "completed");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn reimport_keeps_a_status_the_file_does_not_state(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    let mut cancelled = canonical(&slug("M-73"), Some("FR"));
    cancelled.competition.status = Some(CompetitionStatus::Cancelled);
    transformer.import_to_database(cancelled).await.unwrap();

    transformer
        .import_to_database(canonical(&slug("M-73"), Some("FR")))
        .await
        .unwrap();

    let status: String =
        sqlx::query_scalar!(r#"SELECT status FROM competitions WHERE slug = 'test-open'"#)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(status, "cancelled");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn reimport_corrects_athlete_nationality(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(canonical(&slug("M-73"), Some("US")))
        .await
        .unwrap();
    transformer
        .import_to_database(canonical(&slug("M-73"), Some("FR")))
        .await
        .unwrap();

    let athlete = sqlx::query!(r#"SELECT nationality FROM athletes WHERE last_name = 'Doe'"#)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(athlete.nationality.as_deref(), Some("FR"));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn reimport_corrects_category_bounds(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(canonical(&slug("M-73"), Some("FR")))
        .await
        .unwrap();
    transformer
        .import_to_database(canonical(&slug("M-80"), Some("FR")))
        .await
        .unwrap();

    let category = sqlx::query!(
        r#"SELECT wc.min_kg AS weight_class_min, wc.max_kg AS weight_class_max
           FROM categories c
           JOIN weight_classes wc USING (weight_class_id)
           WHERE c.name = 'Test category'"#
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(category.weight_class_min, Some(Decimal::from(73)));
    assert_eq!(category.weight_class_max, Some(Decimal::from(80)));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn open_class_is_stored_as_a_lower_bound(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(canonical(r#""weight_class_min": "87","#, Some("FR")))
        .await
        .unwrap();

    let category = sqlx::query!(
        r#"SELECT wc.min_kg AS weight_class_min, wc.max_kg AS weight_class_max
           FROM categories c
           JOIN weight_classes wc USING (weight_class_id)
           WHERE c.name = 'Test category'"#
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(category.weight_class_min, Some(Decimal::from(87)));
    assert_eq!(category.weight_class_max, None);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn reimport_without_nationality_keeps_the_known_one(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(canonical(&slug("M-73"), Some("FR")))
        .await
        .unwrap();
    transformer
        .import_to_database(canonical(&slug("M-73"), None))
        .await
        .unwrap();

    let athlete = sqlx::query!(r#"SELECT nationality FROM athletes WHERE last_name = 'Doe'"#)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(athlete.nationality.as_deref(), Some("FR"));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_reported_score_is_kept_and_marked(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(scored(&slug("M-73"), Some("FR"), r#""ris": "84.21","#))
        .await
        .unwrap();

    let row =
        sqlx::query!(r#"SELECT ris_score, ris_source, bodyweight FROM competition_participants"#)
            .fetch_one(&pool)
            .await
            .unwrap();

    assert_eq!(row.ris_score, Some(Decimal::from_str("84.21").unwrap()));
    assert_eq!(row.ris_source.as_deref(), Some("reported"));
    assert_eq!(row.bodyweight, None);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_computed_score_is_marked_as_ours(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(canonical(&slug("M-73"), Some("FR")))
        .await
        .unwrap();

    let row = sqlx::query!(r#"SELECT ris_score, ris_source FROM competition_participants"#)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert!(row.ris_score.is_some());
    assert_eq!(row.ris_source.as_deref(), Some("computed"));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn a_recompute_leaves_a_reported_score_alone(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    let file = scored(&slug("M-73"), Some("FR"), r#""ris": "84.21","#);
    transformer.import_to_database(file.clone()).await.unwrap();
    transformer.import_to_database(file).await.unwrap();

    let row = sqlx::query!(r#"SELECT ris_score, ris_source FROM competition_participants"#)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(row.ris_score, Some(Decimal::from_str("84.21").unwrap()));
    assert_eq!(row.ris_source.as_deref(), Some("reported"));
}
