//! Deterministic on-disk shape for canonical files.
//!
//! A competition file is built up over many edits, and the review is the git
//! diff. If key order or decimal scale drifts between writes, a change that
//! adds eight athletes shows up as a rewrite of the whole file and stops
//! being reviewable. Running every file through here keeps a diff to the
//! bricks that were actually added.

use std::collections::HashMap;

use super::models::CanonicalFormat;

/// Reorders a file into its canonical shape.
///
/// Order is not meaningful anywhere in the format: rank is recomputed from
/// the lifts, so nothing here changes what gets imported.
pub fn normalize(canonical: &mut CanonicalFormat) {
    canonical
        .movements
        .sort_by(|a, b| a.order.cmp(&b.order).then_with(|| a.name.cmp(&b.name)));

    let movement_order: HashMap<&str, i16> = canonical
        .movements
        .iter()
        .map(|m| (m.name.as_str(), m.order))
        .collect();

    // Sorting borrows the map above, so the order is captured first.
    let movement_order: HashMap<String, i16> = movement_order
        .into_iter()
        .map(|(name, order)| (name.to_owned(), order))
        .collect();

    canonical.categories.sort_by(|a, b| a.name.cmp(&b.name));

    for category in &mut canonical.categories {
        if let Some(max) = category.weight_class_max.as_mut() {
            *max = max.normalize();
        }

        category.athletes.sort_by(|a, b| {
            a.last_name
                .cmp(&b.last_name)
                .then_with(|| a.first_name.cmp(&b.first_name))
        });

        for athlete in &mut category.athletes {
            if let Some(bodyweight) = athlete.bodyweight.as_mut() {
                *bodyweight = bodyweight.normalize();
            }

            athlete.lifts.sort_by(|a, b| {
                let left = movement_order.get(&a.movement).copied().unwrap_or(i16::MAX);
                let right = movement_order.get(&b.movement).copied().unwrap_or(i16::MAX);
                left.cmp(&right).then_with(|| a.movement.cmp(&b.movement))
            });

            for lift in &mut athlete.lifts {
                lift.attempts.sort_by_key(|a| a.attempt_number);
                for attempt in &mut lift.attempts {
                    attempt.weight = attempt.weight.normalize();
                }
            }
        }
    }
}

/// Renders a normalized file as the bytes we write to disk.
pub fn to_string(canonical: &CanonicalFormat) -> serde_json::Result<String> {
    let mut rendered = serde_json::to_string_pretty(canonical)?;
    rendered.push('\n');
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::models::{
        AthleteData, AttemptData, CategoryData, CompetitionData, FORMAT_VERSION, FederationData,
        LiftData, MovementData, SourceMetadata, SourceType,
    };
    use chrono::NaiveDate;
    use osl_domain::{AthleteStatus, CountryCode, Gender};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    fn athlete(first: &str, last: &str, movements: &[&str]) -> AthleteData {
        AthleteData {
            first_name: first.to_string(),
            last_name: last.to_string(),
            gender: None,
            country: CountryCode::parse("FR").unwrap(),
            nationality: None,
            team: None,
            bodyweight: Some(Decimal::from_str("78.50").unwrap()),
            status: AthleteStatus::Competed,
            status_reason: None,
            lifts: movements
                .iter()
                .map(|m| LiftData {
                    movement: (*m).to_string(),
                    attempts: vec![
                        AttemptData {
                            attempt_number: 3,
                            weight: Decimal::from_str("100.00").unwrap(),
                            is_successful: false,
                            judge_note: None,
                        },
                        AttemptData {
                            attempt_number: 1,
                            weight: Decimal::from(80),
                            is_successful: true,
                            judge_note: None,
                        },
                    ],
                })
                .collect(),
        }
    }

    fn unsorted() -> CanonicalFormat {
        CanonicalFormat {
            format_version: FORMAT_VERSION.to_string(),
            source: SourceMetadata {
                r#type: SourceType::Image,
                url: None,
                extracted_at: chrono::Utc::now(),
                extractor: "test@1.0.0".to_string(),
                original_filename: None,
            },
            competition: CompetitionData {
                name: "Test Open".to_string(),
                slug: "test-open".to_string(),
                federation: FederationData {
                    name: "Test Federation".to_string(),
                    slug: None,
                    abbreviation: None,
                    country: None,
                },
                start_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
                end_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
                venue: None,
                city: None,
                country: CountryCode::parse("FR").unwrap(),
                number_of_judges: Some(3),
                status: None,
            },
            movements: vec![
                MovementData {
                    name: "Squat".to_string(),
                    order: 4,
                    is_required: Some(true),
                },
                MovementData {
                    name: "Muscle-up".to_string(),
                    order: 1,
                    is_required: Some(true),
                },
            ],
            categories: vec![
                CategoryData {
                    name: "M-87".to_string(),
                    gender: Gender::M,
                    weight_class_slug: None,
                    weight_class_max: Some(Decimal::from_str("87.0").unwrap()),
                    is_open_category: None,
                    athletes: vec![
                        athlete("Bob", "Zulu", &["Squat", "Muscle-up"]),
                        athlete("Alice", "Alpha", &["Squat", "Muscle-up"]),
                    ],
                },
                CategoryData {
                    name: "M-80".to_string(),
                    gender: Gender::M,
                    weight_class_slug: None,
                    weight_class_max: Some(Decimal::from(80)),
                    is_open_category: None,
                    athletes: vec![],
                },
            ],
        }
    }

    #[test]
    fn sorts_movements_by_declared_order() {
        let mut canonical = unsorted();
        normalize(&mut canonical);

        let names: Vec<_> = canonical.movements.iter().map(|m| &m.name).collect();
        assert_eq!(names, vec!["Muscle-up", "Squat"]);
    }

    #[test]
    fn sorts_categories_and_athletes_by_name() {
        let mut canonical = unsorted();
        normalize(&mut canonical);

        let categories: Vec<_> = canonical.categories.iter().map(|c| &c.name).collect();
        assert_eq!(categories, vec!["M-80", "M-87"]);

        let athletes: Vec<_> = canonical.categories[1]
            .athletes
            .iter()
            .map(|a| a.last_name.as_str())
            .collect();
        assert_eq!(athletes, vec!["Alpha", "Zulu"]);
    }

    #[test]
    fn sorts_lifts_by_movement_order_and_attempts_by_number() {
        let mut canonical = unsorted();
        normalize(&mut canonical);

        let athlete = &canonical.categories[1].athletes[0];
        let lifts: Vec<_> = athlete.lifts.iter().map(|l| l.movement.as_str()).collect();
        assert_eq!(lifts, vec!["Muscle-up", "Squat"]);

        let numbers: Vec<_> = athlete.lifts[0]
            .attempts
            .iter()
            .map(|a| a.attempt_number)
            .collect();
        assert_eq!(numbers, vec![1, 3]);
    }

    #[test]
    fn strips_trailing_zeros_so_diffs_stay_stable() {
        let mut canonical = unsorted();
        normalize(&mut canonical);

        let rendered = to_string(&canonical).unwrap();
        assert!(rendered.contains("\"87\""), "{rendered}");
        assert!(rendered.contains("\"78.5\""), "{rendered}");
        assert!(!rendered.contains("100.00"), "{rendered}");
    }

    #[test]
    fn normalizing_twice_changes_nothing() {
        let mut once = unsorted();
        normalize(&mut once);
        let first = to_string(&once).unwrap();

        let mut twice: CanonicalFormat = serde_json::from_str(&first).unwrap();
        normalize(&mut twice);
        let second = to_string(&twice).unwrap();

        assert_eq!(first, second);
    }
}
