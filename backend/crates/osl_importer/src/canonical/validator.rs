use super::models::{CanonicalFormat, FORMAT_VERSION};
use crate::{ImporterError, Result};
use std::collections::HashSet;
use tracing::warn;

pub struct CanonicalValidator;

impl CanonicalValidator {
    pub fn validate(canonical: &CanonicalFormat) -> Result<ValidationReport> {
        let mut report = ValidationReport::default();

        if canonical.format_version != FORMAT_VERSION {
            report.errors.push(format!(
                "Unsupported format version: {}. Expected {}",
                canonical.format_version, FORMAT_VERSION
            ));
        }

        if canonical.source.extractor.is_empty() {
            report
                .errors
                .push("Source extractor is required".to_string());
        }

        if canonical.competition.name.is_empty() {
            report
                .errors
                .push("Competition name is required".to_string());
        }
        if canonical.competition.slug.is_empty() {
            report
                .errors
                .push("Competition slug is required".to_string());
        }
        if canonical.competition.end_date < canonical.competition.start_date {
            report
                .errors
                .push("Competition end_date must be >= start_date".to_string());
        }

        if canonical.competition.federation.name.is_empty() {
            report
                .errors
                .push("Federation name is required".to_string());
        }

        if canonical.competition.venue.is_none() {
            report
                .warnings
                .push("Competition venue is not specified".to_string());
        }
        if canonical.competition.city.is_none() {
            report
                .warnings
                .push("Competition city is not specified".to_string());
        }
        match canonical.competition.number_of_judges {
            None => report
                .warnings
                .push("Number of judges is not specified".to_string()),
            Some(judges) if judges != 1 && judges != 3 => report
                .errors
                .push(format!("Number of judges must be 1 or 3, got {}", judges)),
            Some(_) => {}
        }

        if canonical.movements.is_empty() {
            report
                .errors
                .push("At least one movement is required".to_string());
        }

        let mut movement_names = HashSet::new();
        for movement in &canonical.movements {
            if movement.name.is_empty() {
                report
                    .errors
                    .push("Movement name cannot be empty".to_string());
            }
            if movement.order < 1 {
                report.errors.push(format!(
                    "Movement '{}' has invalid order: {}. Order must be >= 1",
                    movement.name, movement.order
                ));
            }
            if !movement_names.insert(&movement.name) {
                report
                    .errors
                    .push(format!("Duplicate movement name: '{}'", movement.name));
            }
        }

        if canonical.categories.is_empty() {
            report
                .errors
                .push("At least one category is required".to_string());
        }

        for category in &canonical.categories {
            if category.name.is_empty() {
                report
                    .errors
                    .push("Category name cannot be empty".to_string());
            }
            let has_raw_bounds =
                category.weight_class_min.is_some() || category.weight_class_max.is_some();

            if category.weight_class_slug.is_some() && has_raw_bounds {
                report.errors.push(format!(
                    "Category '{}' sets weight_class_slug and raw bounds. The slug already \
                     carries them, so keep the slug for a standard class and the raw bounds \
                     only for a non standard one",
                    category.name
                ));
            }

            if let (Some(min), Some(max)) = (category.weight_class_min, category.weight_class_max)
                && min >= max
            {
                report.errors.push(format!(
                    "Category '{}' has weight_class_min {} above weight_class_max {}",
                    category.name, min, max
                ));
            }

            // Without a bound the class limit lives only in the name, so it is
            // lost as data. The +87 category was imported that way.
            if category.weight_class_slug.is_none() && !has_raw_bounds {
                report.warnings.push(format!(
                    "Category '{}' has no weight class. Set weight_class_slug for a standard \
                     class, or weight_class_min and weight_class_max for a non standard one",
                    category.name
                ));
            }

            if category.athletes.is_empty() {
                report
                    .warnings
                    .push(format!("Category '{}' has no athletes", category.name));
            }

            for (idx, athlete) in category.athletes.iter().enumerate() {
                let athlete_label =
                    format!("{}. {} {}", idx + 1, athlete.first_name, athlete.last_name);

                if athlete.first_name.is_empty() {
                    report.errors.push(format!(
                        "Athlete in category '{}' has empty first_name",
                        category.name
                    ));
                }
                if athlete.last_name.is_empty() {
                    report.errors.push(format!(
                        "Athlete in category '{}' has empty last_name",
                        category.name
                    ));
                }
                if athlete.bodyweight.is_none() {
                    report
                        .warnings
                        .push(format!("Athlete '{}' is missing bodyweight", athlete_label));
                }

                if athlete.lifts.is_empty() {
                    report
                        .warnings
                        .push(format!("Athlete '{}' has no lifts", athlete_label));
                }

                for lift in &athlete.lifts {
                    if !movement_names.contains(&lift.movement) {
                        report.errors.push(format!(
                            "Athlete '{}' has lift for unknown movement: '{}'",
                            athlete_label, lift.movement
                        ));
                    }

                    if lift.attempts.is_empty() {
                        report.errors.push(format!(
                            "Athlete '{}' has lift '{}' with no attempts",
                            athlete_label, lift.movement
                        ));
                    }

                    for attempt in &lift.attempts {
                        if attempt.attempt_number < 1 || attempt.attempt_number > 3 {
                            report.errors.push(format!(
                                "Athlete '{}', movement '{}': invalid attempt_number {}. Must be 1-3",
                                athlete_label, lift.movement, attempt.attempt_number
                            ));
                        }
                        if attempt.weight.is_sign_negative() {
                            report.errors.push(format!(
                                "Athlete '{}', movement '{}', attempt {}: negative weight",
                                athlete_label, lift.movement, attempt.attempt_number
                            ));
                        }
                    }
                }
            }
        }

        if !report.errors.is_empty() {
            Err(ImporterError::ValidationError(format!(
                "Validation failed with {} error(s): {}",
                report.errors.len(),
                report.errors.join("; ")
            )))
        } else {
            Ok(report)
        }
    }
}

#[derive(Debug, Default)]
pub struct ValidationReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationReport {
    pub fn log_warnings(&self) {
        for warning in &self.warnings {
            warn!("{}", warning);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::models::{
        AthleteData, AttemptData, CategoryData, CompetitionData, FederationData, LiftData,
        MovementData, SourceMetadata, SourceType,
    };
    use chrono::NaiveDate;
    use osl_domain::{AthleteStatus, CountryCode, Gender, WeightClassSlug};
    use rust_decimal::Decimal;

    fn minimal() -> CanonicalFormat {
        CanonicalFormat {
            format_version: FORMAT_VERSION.to_string(),
            source: SourceMetadata {
                r#type: SourceType::Image,
                url: None,
                extracted_at: chrono::Utc::now(),
                extractor: "test-extractor@1.0.0".to_string(),
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
            movements: vec![MovementData {
                name: "Squat".to_string(),
                order: 1,
                is_required: Some(true),
            }],
            categories: vec![CategoryData {
                name: "M-80".to_string(),
                gender: Gender::M,
                weight_class_slug: None,
                weight_class_min: None,
                weight_class_max: Some(Decimal::from(80)),
                athletes: vec![AthleteData {
                    first_name: "Adrien".to_string(),
                    last_name: "Pelfresne".to_string(),
                    gender: None,
                    country: CountryCode::parse("FR").unwrap(),
                    nationality: None,
                    team: None,
                    bodyweight: Some(Decimal::from(78)),
                    status: AthleteStatus::Competed,
                    status_reason: None,
                    lifts: vec![LiftData {
                        movement: "Squat".to_string(),
                        attempts: vec![AttemptData {
                            attempt_number: 1,
                            weight: Decimal::from(100),
                            is_successful: true,
                            judge_note: None,
                        }],
                    }],
                }],
            }],
        }
    }

    #[test]
    fn accepts_a_well_formed_file() {
        assert!(CanonicalValidator::validate(&minimal()).is_ok());
    }

    #[test]
    fn older_format_version_is_rejected() {
        let mut canonical = minimal();
        canonical.format_version = "1.1.0".to_string();

        let err = CanonicalValidator::validate(&canonical)
            .unwrap_err()
            .to_string();
        assert!(err.contains("Unsupported format version"), "{err}");
    }

    #[test]
    fn missing_extractor_is_rejected() {
        let mut canonical = minimal();
        canonical.source.extractor = String::new();

        let err = CanonicalValidator::validate(&canonical)
            .unwrap_err()
            .to_string();
        assert!(err.contains("extractor is required"), "{err}");
    }

    #[test]
    fn a_slug_alongside_raw_bounds_is_rejected() {
        let mut canonical = minimal();
        canonical.categories[0].weight_class_slug = Some(WeightClassSlug::M80);

        let err = CanonicalValidator::validate(&canonical)
            .unwrap_err()
            .to_string();
        assert!(err.contains("weight_class_slug and raw bounds"), "{err}");
    }

    #[test]
    fn a_slug_on_its_own_is_accepted() {
        let mut canonical = minimal();
        canonical.categories[0].weight_class_slug = Some(WeightClassSlug::M80);
        canonical.categories[0].weight_class_max = None;

        assert!(CanonicalValidator::validate(&canonical).is_ok());
    }

    #[test]
    fn judge_counts_the_schema_rejects_fail_validation() {
        let mut canonical = minimal();
        canonical.competition.number_of_judges = Some(2);

        let err = CanonicalValidator::validate(&canonical)
            .unwrap_err()
            .to_string();
        assert!(err.contains("must be 1 or 3"), "{err}");
    }

    #[test]
    fn lift_for_an_undeclared_movement_is_rejected() {
        let mut canonical = minimal();
        canonical.categories[0].athletes[0].lifts[0].movement = "Bench".to_string();

        let err = CanonicalValidator::validate(&canonical)
            .unwrap_err()
            .to_string();
        assert!(err.contains("unknown movement"), "{err}");
    }
}
