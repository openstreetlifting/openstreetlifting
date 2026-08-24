#![allow(dead_code)]

use chrono::NaiveDate;
use osl_domain::{
    AthleteStatus, CompetitionStatus, CountryCode, Gender, Movement, WeightClassSlug,
};
use osl_importer::canonical::models::{
    AthleteData, AttemptData, CanonicalFormat, CategoryData, CompetitionData, FORMAT_VERSION,
    FederationData, LiftData,
};
use osl_importer::canonical::transformer::CanonicalTransformer;
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::str::FromStr;

pub fn decimal(raw: &str) -> Decimal {
    Decimal::from_str(raw).expect("test weight should parse")
}

pub fn meet(slug: &str, categories: Vec<CategoryData>) -> CanonicalFormat {
    CanonicalFormat {
        format_version: FORMAT_VERSION.to_string(),
        sources: Vec::new(),
        competition: CompetitionData {
            name: slug.to_string(),
            slug: slug.to_string(),
            federation: FederationData {
                name: "Test Federation".to_string(),
                abbreviation: None,
                country: None,
            },
            start_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            city: Some("Paris".to_string()),
            region: None,
            country: CountryCode::parse("FR").unwrap(),
            status: Some(CompetitionStatus::Completed),
        },
        movements: Movement::ALL.to_vec(),
        categories,
    }
}

pub fn announcement(slug: &str) -> CanonicalFormat {
    let mut canonical = meet(slug, Vec::new());
    canonical.competition.status = Some(CompetitionStatus::Upcoming);
    canonical.movements.clear();
    canonical
}

pub fn category(slug: WeightClassSlug, athletes: Vec<AthleteData>) -> CategoryData {
    let gender = if slug.as_str().starts_with('F') {
        Gender::F
    } else {
        Gender::M
    };

    CategoryData {
        division: None,
        gender,
        weight_class_slug: Some(slug),
        weight_class_min: None,
        weight_class_max: None,
        athletes,
    }
}

pub fn open_category(gender: Gender, min: &str, athletes: Vec<AthleteData>) -> CategoryData {
    CategoryData {
        division: None,
        gender,
        weight_class_slug: None,
        weight_class_min: Some(decimal(min)),
        weight_class_max: None,
        athletes,
    }
}

pub fn men_80(athletes: Vec<AthleteData>) -> CategoryData {
    category(WeightClassSlug::M80, athletes)
}

pub fn in_division(mut category: CategoryData, division: &str) -> CategoryData {
    category.division = Some(division.to_string());
    category
}

pub fn athlete(first: &str, last: &str) -> AthleteData {
    AthleteData {
        first_name: first.to_string(),
        last_name: last.to_string(),
        disambiguation: None,
        gender: Some(Gender::M),
        country: CountryCode::parse("FR").unwrap(),
        bodyweight: Some(Decimal::from(80)),
        ris: None,
        status: AthleteStatus::Competed,
        status_reason: None,
        lifts: Vec::new(),
    }
}

pub fn from(mut athlete: AthleteData, country: &str) -> AthleteData {
    athlete.country = CountryCode::parse(country).expect("test country should parse");
    athlete
}

pub fn weighing(mut athlete: AthleteData, bodyweight: &str) -> AthleteData {
    athlete.bodyweight = Some(decimal(bodyweight));
    athlete
}

pub fn scored(mut athlete: AthleteData, ris: &str) -> AthleteData {
    athlete.bodyweight = None;
    athlete.ris = Some(decimal(ris));
    athlete
}

pub fn disqualified(mut athlete: AthleteData, reason: Option<&str>) -> AthleteData {
    athlete.status = AthleteStatus::Disqualified;
    athlete.status_reason = reason.map(|r| r.to_string());
    athlete
}

pub fn numbered(mut athlete: AthleteData, disambiguation: i16) -> AthleteData {
    athlete.disambiguation = Some(disambiguation);
    athlete
}

pub fn best(mut athlete: AthleteData, movement: Movement, weight: &str) -> AthleteData {
    athlete.lifts.push(LiftData {
        movement,
        attempts: None,
        best_lift: Some(decimal(weight)),
    });
    athlete
}

pub fn attempts(
    mut athlete: AthleteData,
    movement: Movement,
    cells: &[(&str, bool)],
) -> AthleteData {
    let attempts = cells
        .iter()
        .enumerate()
        .map(|(index, (weight, is_successful))| AttemptData {
            attempt_number: index as i16 + 1,
            weight: decimal(weight),
            is_successful: *is_successful,
        })
        .collect();

    athlete.lifts.push(LiftData {
        movement,
        attempts: Some(attempts),
        best_lift: None,
    });
    athlete
}

pub fn lifting(athlete: AthleteData, weights: [&str; 4]) -> AthleteData {
    Movement::ALL
        .into_iter()
        .zip(weights)
        .fold(athlete, |athlete, (movement, weight)| {
            attempts(athlete, movement, &[(weight, true)])
        })
}

pub async fn import(pool: &PgPool, canonical: CanonicalFormat) {
    CanonicalTransformer::new(pool)
        .import_to_database(canonical)
        .await
        .expect("import should succeed");
}

pub async fn try_import(pool: &PgPool, canonical: CanonicalFormat) -> osl_importer::Result<()> {
    CanonicalTransformer::new(pool)
        .import_to_database(canonical)
        .await
}
