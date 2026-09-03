//! A handle is written against a name, but an athlete is a name together with
//! gender, country and disambiguation. These pin down what happens when the
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
async fn a_name_nobody_else_carries_needs_no_columns(pool: PgPool) {
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
async fn a_file_written_before_the_columns_existed_still_reads(pool: PgPool) {
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
async fn a_name_two_athletes_carry_is_refused_until_a_column_settles_it(pool: PgPool) {
    import(
        &pool,
        file(
            "competition-one",
            vec![
                athlete("Tony", "Nguyen"),
                from(athlete("Tony", "Nguyen"), "US"),
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
async fn each_of_two_namesakes_can_hold_their_own_handle(pool: PgPool) {
    import(
        &pool,
        file(
            "competition-one",
            vec![
                athlete("Tony", "Nguyen"),
                from(athlete("Tony", "Nguyen"), "US"),
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
async fn a_disambiguation_number_tells_two_athletes_of_one_country_apart(pool: PgPool) {
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
async fn a_column_that_matches_nobody_says_what_the_name_does_match(pool: PgPool) {
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
    assert!(attached(&pool).await.is_empty());
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn one_bad_row_holds_back_the_whole_file(pool: PgPool) {
    import(
        &pool,
        file(
            "competition-one",
            vec![
                athlete("Léa", "Mérandon"),
                athlete("Tony", "Nguyen"),
                from(athlete("Tony", "Nguyen"), "US"),
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
async fn checking_resolves_without_writing(pool: PgPool) {
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
async fn an_account_cannot_be_listed_against_two_athletes(pool: PgPool) {
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
async fn an_athlete_listed_twice_is_refused(pool: PgPool) {
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
