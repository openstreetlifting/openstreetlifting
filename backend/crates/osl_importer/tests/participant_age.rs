use osl_domain::Age;
use osl_importer::canonical::models::CanonicalFormat;
use osl_importer::canonical::{store, validator::CanonicalValidator};
use sqlx::PgPool;

mod common;

fn aged(age: Option<&str>) -> CanonicalFormat {
    let mut athlete = common::weighing(common::athlete("Ada", "Lovelace"), "79");
    athlete.age = age.map(|raw| Age::parse(raw).unwrap());
    common::competition("aged-open", vec![common::men_80(vec![athlete])])
}

fn headers(directory: &std::path::Path) -> Vec<String> {
    let csv = std::fs::read_to_string(directory.join("entries.csv")).unwrap();
    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    reader.headers().unwrap().iter().map(String::from).collect()
}

#[test]
fn keeps_age_on_round_trip() {
    let directory = std::env::temp_dir().join(format!("osl-age-csv-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&directory).unwrap();

    store::write(&directory, &aged(Some("27"))).unwrap();
    let read = store::read(&directory).unwrap();
    assert_eq!(read.categories[0].athletes[0].age.map(Age::get), Some(27));
    CanonicalValidator::validate(&read).unwrap();
    assert!(headers(&directory).iter().any(|name| name == "Age"));

    store::write(&directory, &aged(None)).unwrap();
    assert!(!headers(&directory).iter().any(|name| name == "Age"));

    std::fs::remove_dir_all(&directory).unwrap();
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn stores_and_updates_age(pool: PgPool) {
    let age = || async {
        sqlx::query_scalar::<_, Option<i16>>("SELECT age FROM competition_participants")
            .fetch_one(&pool)
            .await
            .unwrap()
    };

    common::import(&pool, aged(Some("27"))).await;
    assert_eq!(age().await, Some(27));

    common::import(&pool, aged(Some("28"))).await;
    assert_eq!(age().await, Some(28));

    common::import(&pool, aged(None)).await;
    assert_eq!(age().await, None);
}
