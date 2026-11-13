use chrono::NaiveDate;
use osl_db::params::{RecordClass, RecordsFilter};
use osl_db::projections::records::RecordRow;
use osl_db::repository::records::RecordsRepository;
use osl_domain::{Gender, Movement, WeightClassSlug};
use osl_importer::canonical::models::{AthleteData, CanonicalFormat, CategoryData};
use rust_decimal::Decimal;
use sqlx::PgPool;

mod common;

use common::{athlete, attempts, category, disqualified, from, import, lifting, men_80};

fn on(slug: &str, date: &str, categories: Vec<CategoryData>) -> CanonicalFormat {
    let mut canonical = common::competition(slug, categories);
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    canonical.competition.start_date = date;
    canonical.competition.end_date = date;
    canonical
}

fn muscleup(lifter: AthleteData, weight: &str) -> AthleteData {
    attempts(lifter, Movement::MuscleUp, &[(weight, true)])
}

fn woman(first: &str, last: &str) -> AthleteData {
    let mut lifter = athlete(first, last);
    lifter.gender = Gender::F;
    lifter
}

const fn filter(weight_class: Option<RecordClass>) -> RecordsFilter {
    RecordsFilter {
        movement: Some(Movement::MuscleUp),
        gender: Some(Gender::M),
        weight_class,
        country: None,
        federation: None,
        year: None,
        limit: 3,
    }
}

async fn records(pool: &PgPool, filter: &RecordsFilter) -> Vec<RecordRow> {
    RecordsRepository::new(pool)
        .list(filter)
        .await
        .expect("records should load")
}

fn holders(rows: &[RecordRow]) -> Vec<(&str, String)> {
    rows.iter()
        .map(|row| (row.last_name.as_str(), row.weight.to_string()))
        .collect()
}

fn class(slug: &str) -> RecordClass {
    RecordClass::Slug(slug.to_string())
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn ranks_each_athlete_once_by_their_best(pool: PgPool) {
    import(
        &pool,
        on(
            "first",
            "2025-01-01",
            vec![men_80(vec![
                muscleup(athlete("Alex", "Strong"), "45"),
                muscleup(athlete("Ben", "Steady"), "40"),
            ])],
        ),
    )
    .await;
    import(
        &pool,
        on(
            "second",
            "2026-01-01",
            vec![men_80(vec![muscleup(athlete("Alex", "Strong"), "50")])],
        ),
    )
    .await;

    let rows = records(&pool, &filter(Some(class("M-80")))).await;
    assert_eq!(
        holders(&rows),
        vec![("Strong", "50".into()), ("Steady", "40".into())]
    );
    assert_eq!(rows[0].competition_slug, "second");
    assert_eq!(rows[0].rank, 1);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn the_open_record_spans_weight_classes(pool: PgPool) {
    import(
        &pool,
        on(
            "meet",
            "2026-01-01",
            vec![
                men_80(vec![muscleup(athlete("Heavy", "Lifter"), "40")]),
                category(
                    WeightClassSlug::M73,
                    vec![muscleup(athlete("Light", "Lifter"), "55")],
                ),
            ],
        ),
    )
    .await;

    let open = records(&pool, &filter(Some(RecordClass::Open))).await;
    assert_eq!(
        open.iter()
            .map(|row| (row.first_name.as_str(), row.weight_class.as_deref()))
            .collect::<Vec<_>>(),
        vec![("Light", None), ("Heavy", None)]
    );

    let all = records(&pool, &filter(None)).await;
    assert_eq!(
        all.iter()
            .map(|row| row.weight_class.as_deref())
            .collect::<Vec<_>>(),
        vec![None, None, Some("M-73"), Some("M-80")],
        "the open group comes first, then classes from lightest to heaviest"
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn the_earlier_lift_wins_a_tie(pool: PgPool) {
    import(
        &pool,
        on(
            "later",
            "2026-01-01",
            vec![men_80(vec![muscleup(athlete("Aaron", "Later"), "50")])],
        ),
    )
    .await;
    import(
        &pool,
        on(
            "earlier",
            "2025-01-01",
            vec![men_80(vec![muscleup(athlete("Zed", "Earlier"), "50")])],
        ),
    )
    .await;

    let rows = records(&pool, &filter(Some(class("M-80")))).await;
    assert_eq!(
        holders(&rows),
        vec![("Earlier", "50".into()), ("Later", "50".into())]
    );
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn ignores_disqualified_athletes(pool: PgPool) {
    import(
        &pool,
        on(
            "meet",
            "2026-01-01",
            vec![men_80(vec![
                disqualified(muscleup(athlete("Out", "Ruled"), "70"), None),
                muscleup(athlete("Clean", "Lift"), "30"),
            ])],
        ),
    )
    .await;

    let rows = records(&pool, &filter(Some(class("M-80")))).await;
    assert_eq!(holders(&rows), vec![("Lift", "30".into())]);
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn filters_by_country_gender_and_limit(pool: PgPool) {
    import(
        &pool,
        on(
            "meet",
            "2026-01-01",
            vec![
                men_80(vec![
                    lifting(athlete("First", "French"), ["60", "90", "110", "180"]),
                    lifting(athlete("Second", "French"), ["50", "90", "110", "180"]),
                    from(
                        lifting(athlete("Top", "Italian"), ["70", "90", "110", "180"]),
                        "IT",
                    ),
                ]),
                category(
                    WeightClassSlug::F57,
                    vec![lifting(woman("Only", "Woman"), ["20", "40", "50", "90"])],
                ),
            ],
        ),
    )
    .await;

    let mut french = filter(Some(class("M-80")));
    french.country = Some("FR".into());
    assert_eq!(
        holders(&records(&pool, &french).await),
        vec![("French", "60".into()), ("French", "50".into())]
    );

    let mut first = filter(Some(class("M-80")));
    first.limit = 1;
    assert_eq!(
        holders(&records(&pool, &first).await),
        vec![("Italian", "70".into())]
    );

    let mut women = filter(Some(RecordClass::Open));
    women.gender = Some(Gender::F);
    assert_eq!(
        holders(&records(&pool, &women).await),
        vec![("Woman", "20".into())]
    );
}

#[derive(sqlx::FromRow, Debug, PartialEq)]
struct Step {
    last_name: String,
    weight: Decimal,
    previous_weight: Option<Decimal>,
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn keeps_each_result_that_raised_the_record(pool: PgPool) {
    for (slug, date, last, weight) in [
        ("one", "2024-01-01", "Pioneer", "40"),
        ("two", "2025-01-01", "Equal", "40"),
        ("three", "2026-01-01", "Breaker", "45"),
    ] {
        import(
            &pool,
            on(
                slug,
                date,
                vec![men_80(vec![muscleup(athlete("A", last), weight)])],
            ),
        )
        .await;
    }

    let steps: Vec<Step> = sqlx::query_as(
        "SELECT a.last_name, p.weight, p.previous_weight
         FROM record_progressions p JOIN athletes a USING (athlete_id)
         WHERE p.movement_name = 'Muscle-up' AND p.weight_class_id IS NULL
         ORDER BY p.date",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert_eq!(
        steps,
        vec![
            Step {
                last_name: "Pioneer".into(),
                weight: Decimal::from(40),
                previous_weight: None,
            },
            Step {
                last_name: "Breaker".into(),
                weight: Decimal::from(45),
                previous_weight: Some(Decimal::from(40)),
            },
        ],
        "equalling the record does not take it"
    );

    let class_steps: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM record_progressions
         WHERE movement_name = 'Muscle-up' AND weight_class_id IS NOT NULL",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(class_steps, 2, "the M-80 class has the same two steps");
}

#[sqlx::test(migrations = "../osl_db/migrations")]
async fn takes_each_best_within_a_federation_or_year(pool: PgPool) {
    let mut local = on(
        "local-2025",
        "2025-01-01",
        vec![men_80(vec![
            muscleup(athlete("Alex", "Strong"), "40"),
            muscleup(athlete("Ben", "Steady"), "35"),
        ])],
    );
    local.competition.federation.name = "Local Federation".into();
    import(&pool, local).await;
    import(
        &pool,
        on(
            "worlds",
            "2026-01-01",
            vec![men_80(vec![muscleup(athlete("Alex", "Strong"), "50")])],
        ),
    )
    .await;

    let mut federation = filter(Some(class("M-80")));
    federation.federation = Some("Local Federation".into());
    assert_eq!(
        holders(&records(&pool, &federation).await),
        vec![("Strong", "40".into()), ("Steady", "35".into())],
        "the 50 kg lift belongs to another federation"
    );

    let mut year = filter(Some(RecordClass::Open));
    year.year = Some(2025);
    assert_eq!(
        holders(&records(&pool, &year).await),
        vec![("Strong", "40".into()), ("Steady", "35".into())],
        "the 50 kg lift was set in 2026"
    );

    assert_eq!(
        holders(&records(&pool, &filter(Some(class("M-80")))).await),
        vec![("Strong", "50".into()), ("Steady", "35".into())]
    );
}
