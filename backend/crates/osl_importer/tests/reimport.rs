//! A canonical file is built up over several passes, so importing a corrected
//! file has to land the correction rather than stop at the row already there.

use osl_importer::canonical::{models::CanonicalFormat, transformer::CanonicalTransformer};
use rust_decimal::Decimal;
use sqlx::PgPool;

fn canonical(weight_class_slug: &str, nationality: Option<&str>) -> CanonicalFormat {
    let nationality = match nationality {
        Some(code) => format!(r#""nationality": "{}","#, code),
        None => String::new(),
    };

    serde_json::from_str(&format!(
        r#"{{
          "format_version": "1.2.0",
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
          "movements": [{{ "name": "Pull-up", "order": 1 }}],
          "categories": [
            {{
              "name": "Test category",
              "gender": "M",
              "weight_class_slug": "{}",
              "athletes": [
                {{
                  "first_name": "John",
                  "last_name": "Doe",
                  "country": "FR",
                  {}
                  "bodyweight": "72.5",
                  "status": "competed",
                  "lifts": [
                    {{
                      "movement": "Pull-up",
                      "attempts": [
                        {{ "attempt_number": 1, "weight": "60", "is_successful": true }}
                      ]
                    }}
                  ]
                }}
              ]
            }}
          ]
        }}"#,
        weight_class_slug, nationality
    ))
    .expect("fixture is a valid canonical file")
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn reimport_corrects_athlete_nationality(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(canonical("M-73", Some("US")))
        .await
        .unwrap();
    transformer
        .import_to_database(canonical("M-73", Some("FR")))
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
        .import_to_database(canonical("M-73", Some("FR")))
        .await
        .unwrap();
    transformer
        .import_to_database(canonical("M-80", Some("FR")))
        .await
        .unwrap();

    let category = sqlx::query!(
        r#"SELECT weight_class_min, weight_class_max FROM categories WHERE name = 'Test category'"#
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(category.weight_class_min, Some(Decimal::from(73)));
    assert_eq!(category.weight_class_max, Some(Decimal::from(80)));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn reimport_without_nationality_keeps_the_known_one(pool: PgPool) {
    let transformer = CanonicalTransformer::new(&pool);

    transformer
        .import_to_database(canonical("M-73", Some("FR")))
        .await
        .unwrap();
    transformer
        .import_to_database(canonical("M-73", None))
        .await
        .unwrap();

    let athlete = sqlx::query!(r#"SELECT nationality FROM athletes WHERE last_name = 'Doe'"#)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(athlete.nationality.as_deref(), Some("FR"));
}
