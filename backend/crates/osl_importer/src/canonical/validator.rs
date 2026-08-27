use super::models::CanonicalFormat;
use crate::{ImporterError, Result};
use osl_domain::{AthleteStatus, CompetitionStatus, NormalizedAthleteName, slugify};
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};
use tracing::warn;

pub struct CanonicalValidator;

impl CanonicalValidator {
    pub fn validate(canonical: &CanonicalFormat) -> Result<ValidationReport> {
        let mut report = ValidationReport::default();

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
        } else if slugify(&canonical.competition.federation.name).is_empty() {
            // The name becomes the directory the competition lives in, so one that
            // slugifies to nothing would collapse that level of the path away.
            report.errors.push(format!(
                "Federation name '{}' has no letter or digit to name its directory",
                canonical.competition.federation.name
            ));
        }

        if canonical.competition.city.is_none() {
            report
                .warnings
                .push("Competition city is not specified".to_string());
        }

        Self::check_announcement(canonical, &mut report);
        Self::check_divisions(canonical, &mut report);
        Self::check_athletes(canonical, &mut report);
        Self::check_athlete_identities(canonical, &mut report);

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

    /// A competition with no entries.csv is an announcement, and one with entries is a
    /// result. Letting the two drift apart would either hide results behind an
    /// upcoming status on re-import, or publish an empty competition as though
    /// it had been lifted.
    fn check_announcement(canonical: &CanonicalFormat, report: &mut ValidationReport) {
        let announced = canonical.competition.status == Some(CompetitionStatus::Upcoming);

        if canonical.categories.is_empty() {
            if !announced {
                report.errors.push(
                    "No entries.csv, so this is an announcement and needs status = \"upcoming\""
                        .to_string(),
                );
            }
            return;
        }

        if announced {
            report.errors.push(
                "entries.csv lists results, so status cannot be \"upcoming\". Move it to \
                 \"completed\" in the same edit"
                    .to_string(),
            );
        }

        if canonical.movements.is_empty() {
            report
                .errors
                .push("entries.csv lists results, so event is required".to_string());
        }
    }

    /// A division tells two contests in one competition apart, so a file that names
    /// one for some entries and not others is claiming a contest nobody ran.
    /// Left alone it hands that phantom contest its own winner, which is the
    /// collapse this column exists to prevent, running the other way.
    fn check_divisions(canonical: &CanonicalFormat, report: &mut ValidationReport) {
        let named: Vec<&str> = canonical
            .categories
            .iter()
            .filter_map(|category| category.division.as_deref())
            .collect();

        if named.is_empty() {
            return;
        }

        let missing: Vec<String> = canonical
            .categories
            .iter()
            .filter(|category| category.division.is_none())
            .map(|category| category.label())
            .collect();

        if !missing.is_empty() {
            report.errors.push(format!(
                "The competition names divisions ({}), so every entry needs one. Missing on: {}",
                named.join(", "),
                missing.join(", ")
            ));
        }

        let mut by_fold: HashMap<String, HashSet<&str>> = HashMap::new();
        for division in &named {
            by_fold
                .entry(division.trim().to_lowercase())
                .or_default()
                .insert(division);
        }

        for spellings in by_fold.values().filter(|s| s.len() > 1) {
            let mut spellings: Vec<&str> = spellings.iter().copied().collect();
            spellings.sort();
            report.errors.push(format!(
                "Divisions {} differ only in case or spacing, so they would import as \
                 separate contests. Pick one spelling",
                spellings
                    .iter()
                    .map(|s| format!("'{s}'"))
                    .collect::<Vec<_>>()
                    .join(" and ")
            ));
        }
    }

    fn check_athletes(canonical: &CanonicalFormat, report: &mut ValidationReport) {
        for category in &canonical.categories {
            for athlete in &category.athletes {
                let label = athlete.display_name();

                match (athlete.bodyweight, athlete.ris) {
                    (Some(_), Some(_)) => report.errors.push(format!(
                        "Athlete '{label}' sets both bodyweight and ris. We compute the score \
                         from the bodyweight, so give ris only when the source states a score \
                         and no bodyweight"
                    )),
                    (None, None) => report.warnings.push(format!(
                        "Athlete '{label}' has neither bodyweight nor ris, so no score can be \
                         recorded"
                    )),
                    _ => {}
                }

                if athlete.bodyweight.is_some_and(|w| w <= Decimal::ZERO) {
                    report.errors.push(format!(
                        "Athlete '{label}' has a bodyweight of zero or less"
                    ));
                }

                if athlete.ris.is_some_and(|r| r < Decimal::ZERO) {
                    report
                        .errors
                        .push(format!("Athlete '{label}' has a negative ris"));
                }

                if athlete.lifts.is_empty() {
                    report
                        .warnings
                        .push(format!("Athlete '{label}' has no lifts"));
                }

                if athlete.status == AthleteStatus::NoShow && !athlete.lifts.is_empty() {
                    report.errors.push(format!(
                        "Athlete '{label}' is a no_show but has lifts. Someone who took an \
                         attempt competed, and is disqualified if the result does not stand"
                    ));
                }

                if athlete.status == AthleteStatus::Competed && athlete.status_reason.is_some() {
                    report.errors.push(format!(
                        "Athlete '{label}' competed but carries a status reason. A reason \
                         explains an outcome, so it belongs to a status that is not competed"
                    ));
                }

                if let Some(disambiguation) = athlete.disambiguation
                    && disambiguation < 1
                {
                    report.errors.push(format!(
                        "Athlete '{label}' has disambiguation {disambiguation}. It numbers \
                         people sharing a name, so it starts at 1"
                    ));
                }

                for lift in &athlete.lifts {
                    if lift.best_lift.is_some_and(|w| w < Decimal::ZERO) {
                        report.errors.push(format!(
                            "Athlete '{label}', movement '{}': negative best lift",
                            lift.movement
                        ));
                    }
                }
            }
        }
    }

    /// Two entries that fold to one identity are either the same person, which
    /// is fine when they entered two categories, or two people who need a
    /// disambiguation number. Within a single category they can only be a
    /// duplicated row.
    fn check_athlete_identities(canonical: &CanonicalFormat, report: &mut ValidationReport) {
        let mut seen: HashMap<(String, Option<i16>), Vec<String>> = HashMap::new();

        for category in &canonical.categories {
            let mut seen_in_category = HashSet::new();

            for athlete in &category.athletes {
                let identity = NormalizedAthleteName::new(&athlete.first_name, &athlete.last_name)
                    .match_name();

                if !seen_in_category.insert((identity.clone(), athlete.disambiguation)) {
                    report.errors.push(format!(
                        "Category '{}' lists '{}' twice. Remove the duplicate, or set \
                         disambiguation if these are two different people",
                        category.label(),
                        athlete.display_name()
                    ));
                }

                seen.entry((identity, athlete.disambiguation))
                    .or_default()
                    .push(category.label());
            }
        }

        for ((identity, disambiguation), categories) in seen {
            if categories.len() > 1 && disambiguation.is_none() {
                report.warnings.push(format!(
                    "'{}' appears in {} categories ({}) and will import as one athlete. Set \
                     disambiguation if these are different people",
                    identity,
                    categories.len(),
                    categories.join(", ")
                ));
            }
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
    };
    use chrono::NaiveDate;
    use osl_domain::{AthleteStatus, CountryCode, Gender, Movement, WeightClassSlug};
    use rust_decimal::Decimal;

    fn announced() -> CanonicalFormat {
        CanonicalFormat {
            sources: Vec::new(),
            competition: CompetitionData {
                name: "Test Open".to_string(),
                slug: "test-open".to_string(),
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
                status: Some(CompetitionStatus::Upcoming),
            },
            movements: Vec::new(),
            categories: Vec::new(),
        }
    }

    fn athlete(first: &str, last: &str) -> AthleteData {
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
            lifts: vec![LiftData {
                movement: Movement::Squat,
                attempts: Some(vec![AttemptData {
                    attempt_number: 1,
                    weight: Decimal::from(100),
                    is_successful: true,
                }]),
                best_lift: None,
            }],
        }
    }

    fn completed() -> CanonicalFormat {
        let mut canonical = announced();
        canonical.competition.status = Some(CompetitionStatus::Completed);
        canonical.movements = vec![Movement::Squat];
        canonical.categories = vec![CategoryData {
            division: None,
            gender: Gender::M,
            weight_class_slug: Some(WeightClassSlug::M80),
            weight_class_min: None,
            weight_class_max: None,
            athletes: vec![athlete("Alice", "Alpha")],
        }];
        canonical
    }

    #[test]
    fn accepts_a_well_formed_file() {
        assert!(CanonicalValidator::validate(&completed()).is_ok());
    }

    #[test]
    fn accepts_an_announcement() {
        assert!(CanonicalValidator::validate(&announced()).is_ok());
    }

    #[test]
    fn a_federation_name_that_slugifies_to_nothing_is_rejected() {
        let mut canonical = completed();
        canonical.competition.federation.name = "---".to_string();

        let error = CanonicalValidator::validate(&canonical)
            .unwrap_err()
            .to_string();
        assert!(error.contains("no letter or digit"), "{error}");
    }

    #[test]
    fn an_end_date_before_the_start_is_rejected() {
        let mut canonical = completed();
        canonical.competition.end_date = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
        assert!(CanonicalValidator::validate(&canonical).is_err());
    }

    #[test]
    fn an_announcement_without_an_upcoming_status_is_rejected() {
        let mut canonical = announced();
        canonical.competition.status = Some(CompetitionStatus::Completed);
        assert!(CanonicalValidator::validate(&canonical).is_err());
    }

    #[test]
    fn an_announcement_without_any_status_is_rejected() {
        let mut canonical = announced();
        canonical.competition.status = None;
        assert!(CanonicalValidator::validate(&canonical).is_err());
    }

    #[test]
    fn results_under_an_upcoming_status_are_rejected() {
        let mut canonical = completed();
        canonical.competition.status = Some(CompetitionStatus::Upcoming);
        assert!(CanonicalValidator::validate(&canonical).is_err());
    }

    #[test]
    fn results_without_an_event_are_rejected() {
        let mut canonical = completed();
        canonical.movements.clear();
        assert!(CanonicalValidator::validate(&canonical).is_err());
    }

    #[test]
    fn an_announcement_may_omit_its_event() {
        let mut canonical = announced();
        canonical.movements.clear();
        assert!(CanonicalValidator::validate(&canonical).is_ok());
    }

    #[test]
    fn both_bodyweight_and_ris_is_rejected() {
        let mut canonical = completed();
        canonical.categories[0].athletes[0].ris = Some(Decimal::from(90));
        assert!(CanonicalValidator::validate(&canonical).is_err());
    }

    #[test]
    fn neither_bodyweight_nor_ris_only_warns() {
        let mut canonical = completed();
        canonical.categories[0].athletes[0].bodyweight = None;

        let report = CanonicalValidator::validate(&canonical).unwrap();
        assert!(report.warnings.iter().any(|w| w.contains("neither")));
    }

    #[test]
    fn a_disambiguation_below_one_is_rejected() {
        let mut canonical = completed();
        canonical.categories[0].athletes[0].disambiguation = Some(0);
        assert!(CanonicalValidator::validate(&canonical).is_err());
    }

    #[test]
    fn the_same_athlete_twice_in_one_category_is_rejected() {
        let mut canonical = completed();
        canonical.categories[0]
            .athletes
            .push(athlete("Alice", "Alpha"));
        assert!(CanonicalValidator::validate(&canonical).is_err());
    }

    #[test]
    fn the_same_athlete_in_two_categories_only_warns() {
        let mut canonical = completed();
        let mut second = canonical.categories[0].clone();
        second.weight_class_slug = Some(WeightClassSlug::M87);
        canonical.categories.push(second);

        let report = CanonicalValidator::validate(&canonical).unwrap();
        assert!(report.warnings.iter().any(|w| w.contains("2 categories")));
    }

    #[test]
    fn the_same_athlete_in_two_divisions_only_warns() {
        let mut canonical = completed();
        canonical.categories[0].division = Some("Elite".to_string());

        let mut second = canonical.categories[0].clone();
        second.division = Some("Open".to_string());
        canonical.categories.push(second);

        let report = CanonicalValidator::validate(&canonical).unwrap();
        assert!(report.warnings.iter().any(|w| w.contains("2 categories")));
    }

    #[test]
    fn a_meet_with_no_divisions_is_fine() {
        assert!(CanonicalValidator::validate(&completed()).is_ok());
    }

    #[test]
    fn an_entry_without_a_division_in_a_divisioned_meet_is_rejected() {
        let mut canonical = completed();
        canonical.categories[0].division = Some("Elite".to_string());

        let mut open = canonical.categories[0].clone();
        open.division = None;
        open.weight_class_slug = Some(WeightClassSlug::M87);
        canonical.categories.push(open);

        let error = CanonicalValidator::validate(&canonical)
            .unwrap_err()
            .to_string();
        assert!(error.contains("every entry needs one"), "{error}");
    }

    #[test]
    fn divisions_differing_only_in_case_are_rejected() {
        let mut canonical = completed();
        canonical.categories[0].division = Some("Elite".to_string());

        let mut shouty = canonical.categories[0].clone();
        shouty.division = Some("elite".to_string());
        shouty.weight_class_slug = Some(WeightClassSlug::M87);
        canonical.categories.push(shouty);

        let error = CanonicalValidator::validate(&canonical)
            .unwrap_err()
            .to_string();
        assert!(error.contains("differ only in case"), "{error}");
    }

    #[test]
    fn two_real_divisions_are_accepted() {
        let mut canonical = completed();
        canonical.categories[0].division = Some("Elite".to_string());

        let mut open = canonical.categories[0].clone();
        open.division = Some("Open".to_string());
        open.athletes[0].first_name = "Bea".to_string();
        canonical.categories.push(open);

        assert!(CanonicalValidator::validate(&canonical).is_ok());
    }

    #[test]
    fn a_negative_best_lift_is_rejected() {
        let mut canonical = completed();
        canonical.categories[0].athletes[0].lifts[0].attempts = None;
        canonical.categories[0].athletes[0].lifts[0].best_lift = Some(Decimal::from(-10));
        assert!(CanonicalValidator::validate(&canonical).is_err());
    }
}
