//! A handle is written against a name, but an athlete is a name together with
//! gender and disambiguation. These pin down what happens when the
//! two stop lining up: the row is narrowed by the columns that address an
//! athlete, and until it names exactly one, nothing at all is written.

use std::path::{Path, PathBuf};

use osl_domain::Movement;
use osl_importer::canonical::models::{AthleteData, CanonicalFormat};
use osl_importer::social::{check_instagram_handles, load_instagram_handles};
use sqlx::PgPool;
use uuid::Uuid;

mod common;

use common::{attempts, from, import, men_80, numbered, weighing};

fn athlete(first: &str, last: &str) -> AthleteData {
    let lifter = weighing(common::athlete(first, last), "79");
    attempts(lifter, Movement::MuscleUp, &[("60", true)])
}

fn file(slug: &str, athletes: Vec<AthleteData>) -> CanonicalFormat {
    let mut canonical = common::competition(slug, vec![men_80(athletes)]);
    canonical.movements = vec![Movement::MuscleUp];
    canonical
}

/// The file the importer reads, written where a test can throw it away.
struct HandleFile(PathBuf);

impl HandleFile {
    fn new(contents: &str) -> Self {
        let path = std::env::temp_dir().join(format!("osl-instagram-{}.csv", Uuid::new_v4()));
        std::fs::write(&path, contents).expect("temp file should be writable");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for HandleFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// Every handle in the database with the country of the athlete wearing it,
/// which is what tells the two people sharing a name apart.
async fn attached(pool: &PgPool) -> Vec<(String, String)> {
    sqlx::query_as(
        "SELECT a.country, s.handle FROM athlete_socials s
         JOIN athletes a USING (athlete_id) ORDER BY s.handle",
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn resolves_unique_names_without_filters(pool: PgPool) {
    import(
        &pool,
        file("competition-one", vec![athlete("Léa", "Mérandon")]),
    )
    .await;

    let handles = HandleFile::new(
        "Name,Sex,Country,Disambiguation,Instagram\n\
         Lea Merandon,,,,lea_sw\n",
    );

    let report = load_instagram_handles(handles.path(), &pool).await.unwrap();

    assert_eq!(report.matched, 1);
    assert_eq!(attached(&pool).await, vec![("FR".into(), "lea_sw".into())]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn accepts_legacy_files_without_identity_columns(pool: PgPool) {
    import(
        &pool,
        file("competition-one", vec![athlete("Léa", "Mérandon")]),
    )
    .await;

    let handles = HandleFile::new("Name,Instagram\nLea Merandon,lea_sw\n");

    let report = load_instagram_handles(handles.path(), &pool).await.unwrap();

    assert_eq!(report.matched, 1);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn requires_filters_when_names_are_ambiguous(pool: PgPool) {
    import(
        &pool,
        file(
            "competition-one",
            vec![
                athlete("Tony", "Nguyen"),
                numbered(from(athlete("Tony", "Nguyen"), "US"), 2),
            ],
        ),
    )
    .await;

    let ambiguous = HandleFile::new(
        "Name,Sex,Country,Disambiguation,Instagram\n\
         Tony Nguyen,,,,tony_sw\n",
    );

    let problem = load_instagram_handles(ambiguous.path(), &pool)
        .await
        .unwrap_err()
        .to_string();

    assert!(problem.contains("1 ambiguous"), "{problem}");
    assert!(attached(&pool).await.is_empty(), "nothing was written");

    // The same row, narrowed to the one it meant.
    let narrowed = HandleFile::new(
        "Name,Sex,Country,Disambiguation,Instagram\n\
         Tony Nguyen,,US,,tony_sw\n",
    );

    let report = load_instagram_handles(narrowed.path(), &pool)
        .await
        .unwrap();

    assert_eq!(report.matched, 1);
    assert_eq!(attached(&pool).await, vec![("US".into(), "tony_sw".into())]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn attaches_separate_handles_to_namesakes(pool: PgPool) {
    import(
        &pool,
        file(
            "competition-one",
            vec![
                athlete("Tony", "Nguyen"),
                numbered(from(athlete("Tony", "Nguyen"), "US"), 2),
            ],
        ),
    )
    .await;

    let handles = HandleFile::new(
        "Name,Sex,Country,Disambiguation,Instagram\n\
         Tony Nguyen,,FR,,tony_fr\n\
         Tony Nguyen,,US,,tony_us\n",
    );

    let report = load_instagram_handles(handles.path(), &pool).await.unwrap();

    assert_eq!(report.matched, 2);
    assert_eq!(
        attached(&pool).await,
        vec![
            ("FR".to_string(), "tony_fr".to_string()),
            ("US".to_string(), "tony_us".to_string())
        ]
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn resolves_namesakes_by_disambiguation(pool: PgPool) {
    import(
        &pool,
        file(
            "competition-one",
            vec![
                athlete("Tom", "Berthier"),
                numbered(athlete("Tom", "Berthier"), 2),
            ],
        ),
    )
    .await;

    let handles = HandleFile::new(
        "Name,Sex,Country,Disambiguation,Instagram\n\
         Tom Berthier,,,2,tom_two\n",
    );

    let report = load_instagram_handles(handles.path(), &pool).await.unwrap();

    assert_eq!(report.matched, 1);

    let wearer: Option<i16> = sqlx::query_scalar(
        "SELECT a.disambiguation FROM athlete_socials s JOIN athletes a USING (athlete_id)",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(wearer, Some(2));
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn rejects_unmatched_identity_filters(pool: PgPool) {
    import(
        &pool,
        file("competition-one", vec![athlete("Tony", "Nguyen")]),
    )
    .await;

    let handles = HandleFile::new(
        "Name,Sex,Country,Disambiguation,Instagram\n\
         Tony Nguyen,,DE,,tony_sw\n",
    );

    let report = check_instagram_handles(handles.path(), &pool).await;

    assert!(report.is_err());
    assert_eq!(attached(&pool).await, [] as [(String, String); 0]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn preserves_database_when_any_row_is_invalid(pool: PgPool) {
    import(
        &pool,
        file(
            "competition-one",
            vec![
                athlete("Léa", "Mérandon"),
                athlete("Tony", "Nguyen"),
                numbered(from(athlete("Tony", "Nguyen"), "US"), 2),
            ],
        ),
    )
    .await;

    let handles = HandleFile::new(
        "Name,Sex,Country,Disambiguation,Instagram\n\
         Lea Merandon,,,,lea_sw\n\
         Tony Nguyen,,,,tony_sw\n",
    );

    assert!(load_instagram_handles(handles.path(), &pool).await.is_err());
    assert!(
        attached(&pool).await.is_empty(),
        "the row that did resolve was not written either"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn resolves_handles_without_writing_when_checking(pool: PgPool) {
    import(
        &pool,
        file("competition-one", vec![athlete("Léa", "Mérandon")]),
    )
    .await;

    let handles = HandleFile::new(
        "Name,Sex,Country,Disambiguation,Instagram\n\
         Lea Merandon,,,,lea_sw\n",
    );

    let report = check_instagram_handles(handles.path(), &pool)
        .await
        .unwrap();

    assert_eq!(report.matched, 1);
    assert!(attached(&pool).await.is_empty(), "a check writes nothing");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn rejects_accounts_shared_by_athletes(pool: PgPool) {
    import(
        &pool,
        file(
            "competition-one",
            vec![athlete("Léa", "Mérandon"), athlete("Tony", "Nguyen")],
        ),
    )
    .await;

    let handles = HandleFile::new(
        "Name,Sex,Country,Disambiguation,Instagram\n\
         Lea Merandon,,,,shared_sw\n\
         Tony Nguyen,,,,shared_sw\n",
    );

    let problem = load_instagram_handles(handles.path(), &pool)
        .await
        .unwrap_err()
        .to_string();

    assert!(problem.contains("shared_sw"), "{problem}");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn rejects_duplicate_athlete_rows(pool: PgPool) {
    import(
        &pool,
        file("competition-one", vec![athlete("Léa", "Mérandon")]),
    )
    .await;

    let handles = HandleFile::new(
        "Name,Sex,Country,Disambiguation,Instagram\n\
         Lea Merandon,,,,lea_sw\n\
         Léa Mérandon,,,,lea_two\n",
    );

    let problem = load_instagram_handles(handles.path(), &pool)
        .await
        .unwrap_err()
        .to_string();

    assert!(problem.contains("listed twice"), "{problem}");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn rejects_handles_for_redacted_aliases(pool: PgPool) {
    import(
        &pool,
        file("redacted", vec![athlete("", "Redacted Athlete #1")]),
    )
    .await;
    for name in [
        "redacted athlete #1",
        "Redacted Athlete 1",
        "REDACTED ATHLETE #1",
    ] {
        let handles = HandleFile::new(&format!("Name,Instagram\n{name},fixture_handle\n"));
        assert!(load_instagram_handles(handles.path(), &pool).await.is_err());
        assert_eq!(attached(&pool).await, [] as [(String, String); 0]);
    }
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn clears_redacted_identity_on_reimport(pool: PgPool) {
    let fixture = || file("redacted", vec![athlete("", "Redacted Athlete #1")]);
    import(&pool, fixture()).await;
    sqlx::query("UPDATE athletes SET native_name = 'Фикстур Альфа', native_script = 'cyrillic', profile_picture_url = 'https://example.com/fixture.jpg' WHERE match_key = 'redacted athlete 1'")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO athlete_socials (athlete_id, social_id, handle) SELECT a.athlete_id, s.social_id, 'fixture_handle' FROM athletes a CROSS JOIN socials s WHERE a.match_key = 'redacted athlete 1' AND s.name = 'instagram'")
        .execute(&pool).await.unwrap();
    import(&pool, fixture()).await;
    let details: (Option<String>, Option<String>, Option<String>) = sqlx::query_as("SELECT native_name, native_script, profile_picture_url FROM athletes WHERE match_key = 'redacted athlete 1'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(details, (None, None, None));
    assert_eq!(attached(&pool).await, [] as [(String, String); 0]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn rejects_native_names_for_redacted_aliases(pool: PgPool) {
    let mut replacement = athlete("", "Redacted Athlete #1");
    replacement.total = replacement.total_from_lifts(&[Movement::MuscleUp]);
    replacement.native_name = Some("Фикстур Альфа".into());
    let error = osl_importer::canonical::transformer::CanonicalTransformer::new(&pool)
        .import_to_database(file("redacted", vec![replacement]))
        .await
        .unwrap_err();
    assert!(error.to_string().contains("NativeName"));
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM athletes")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}
