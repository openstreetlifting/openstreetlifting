use std::collections::HashMap;

use osl_domain::Movement;
use rust_decimal::Decimal;

pub const FILE_NAME: &str = "entries.csv";

pub const DIVISION: &str = "Division";
pub const SEX: &str = "Sex";
pub const CATEGORY_SEX: &str = "CategorySex";
pub const WEIGHT_CLASS: &str = "WeightClassKg";
pub const FIRST_NAME: &str = "FirstName";
pub const LAST_NAME: &str = "LastName";
pub const DISAMBIGUATION: &str = "Disambiguation";
pub const COUNTRY: &str = "Country";
pub const BODYWEIGHT: &str = "BodyweightKg";
pub const REPORTED_RIS: &str = "ReportedRis";
pub const BODYWEIGHT_SOURCE: &str = "BodyweightSource";
pub const REPORTED_RIS_EDITION: &str = "ReportedRisEdition";
pub const TOTAL: &str = "TotalKg";
pub const STATUS: &str = "Status";
pub const STATUS_REASON: &str = "StatusReason";
pub const NATIVE_NAME: &str = "NativeName";

// These columns may be omitted on input. Formatting always includes
// IDENTITY_COLUMNS so contributors can copy a consistent base header.
pub const OPTIONAL_COLUMNS: [&str; 9] = [
    CATEGORY_SEX,
    FIRST_NAME,
    DISAMBIGUATION,
    STATUS_REASON,
    DIVISION,
    WEIGHT_CLASS,
    NATIVE_NAME,
    BODYWEIGHT_SOURCE,
    REPORTED_RIS_EDITION,
];

pub const IDENTITY_COLUMNS: [&str; 10] = [
    SEX,
    FIRST_NAME,
    LAST_NAME,
    DISAMBIGUATION,
    COUNTRY,
    BODYWEIGHT,
    REPORTED_RIS,
    TOTAL,
    STATUS,
    STATUS_REASON,
];

pub const ATTEMPTS_PER_MOVEMENT: i16 = 3;

pub fn attempt_column(movement: Movement, attempt: i16) -> String {
    format!("{}{}Kg", movement.column_prefix(), attempt)
}

pub fn best_column(movement: Movement) -> String {
    format!("Best{}Kg", movement.column_prefix())
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Layout {
    pub mixed: bool,
    pub divisioned: bool,
    pub classed: bool,
    pub native_names: bool,
    pub bodyweight_sources: bool,
    pub reported_ris_editions: bool,
}

pub fn headers(layout: Layout) -> Vec<String> {
    let mut headers: Vec<String> = layout
        .divisioned
        .then(|| DIVISION.to_string())
        .into_iter()
        .collect();

    for column in IDENTITY_COLUMNS {
        headers.push(column.to_string());
        if column == SEX && layout.mixed {
            headers.push(CATEGORY_SEX.to_string());
        }
        if column == SEX && layout.classed {
            headers.push(WEIGHT_CLASS.to_string());
        }
    }

    if layout.native_names {
        headers.push(NATIVE_NAME.to_string());
    }
    if layout.bodyweight_sources {
        headers.push(BODYWEIGHT_SOURCE.to_string());
    }
    if layout.reported_ris_editions {
        headers.push(REPORTED_RIS_EDITION.to_string());
    }

    for movement in Movement::ALL {
        for attempt in 1..=ATTEMPTS_PER_MOVEMENT {
            headers.push(attempt_column(movement, attempt));
        }
        headers.push(best_column(movement));
    }

    headers
}

#[derive(Debug, Clone, Copy)]
pub struct Attempt {
    pub weight: Decimal,
    pub is_successful: bool,
}

pub fn parse_attempt(cell: &str) -> Result<Option<Attempt>, String> {
    let cell = cell.trim();

    if cell.is_empty() {
        return Ok(None);
    }

    let (raw, is_successful) = match cell.strip_suffix(['x', 'X']) {
        Some(raw) => (raw.trim_end(), false),
        None => (cell, true),
    };

    if raw.is_empty() {
        return Err(format!(
            "'{cell}' has no weight, write a missed lift as 100x"
        ));
    }

    if raw.starts_with('-') {
        return Err(format!(
            "'{cell}' is negative, write a missed lift as {}x",
            raw.trim_start_matches('-')
        ));
    }

    let weight: Decimal = raw
        .parse()
        .map_err(|_| format!("'{cell}' is not a weight, expected something like 100 or 100x"))?;

    Ok(Some(Attempt {
        weight,
        is_successful,
    }))
}

pub fn render_attempt(attempt: &Attempt) -> String {
    let weight = attempt.weight.normalize();

    if attempt.is_successful {
        weight.to_string()
    } else {
        format!("{weight}x")
    }
}

pub fn parse_decimal(cell: &str) -> Result<Option<Decimal>, String> {
    let cell = cell.trim();

    if cell.is_empty() {
        return Ok(None);
    }

    cell.parse()
        .map(Some)
        .map_err(|_| format!("'{cell}' is not a number"))
}

pub fn render_decimal(value: Option<Decimal>) -> String {
    value.map(|v| v.normalize().to_string()).unwrap_or_default()
}

#[derive(Debug)]
pub struct Columns {
    index: HashMap<String, usize>,
}

impl Columns {
    pub fn read(header: &csv::StringRecord) -> Result<Self, String> {
        let mut index = HashMap::new();

        if header.iter().any(|name| name.trim() == "Ris") {
            return Err(
                "Ris is no longer supported; rename it to ReportedRis and keep only that column"
                    .into(),
            );
        }

        for (position, name) in header.iter().enumerate() {
            let name = name.trim().to_string();

            if index.insert(name.clone(), position).is_some() {
                return Err(format!("column '{name}' appears twice"));
            }
        }

        let expected = headers(Layout {
            mixed: true,
            divisioned: true,
            classed: true,
            native_names: true,
            bodyweight_sources: true,
            reported_ris_editions: true,
        });

        let missing: Vec<&String> = expected
            .iter()
            .filter(|c| !OPTIONAL_COLUMNS.contains(&c.as_str()) && !index.contains_key(*c))
            .collect();
        if !missing.is_empty() {
            return Err(format!(
                "missing column(s): {}",
                missing
                    .iter()
                    .map(|c| c.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        let unknown: Vec<&String> = index.keys().filter(|c| !expected.contains(c)).collect();
        if !unknown.is_empty() {
            let mut unknown: Vec<&str> = unknown.iter().map(|c| c.as_str()).collect();
            unknown.sort();
            return Err(format!("unknown column(s): {}", unknown.join(", ")));
        }

        Ok(Self { index })
    }

    pub fn has(&self, column: &str) -> bool {
        self.index.contains_key(column)
    }

    pub fn get<'a>(&self, record: &'a csv::StringRecord, column: &str) -> &'a str {
        self.index
            .get(column)
            .and_then(|position| record.get(*position))
            .unwrap_or("")
            .trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn a_bare_weight_is_a_good_lift() {
        let attempt = parse_attempt("100").unwrap().unwrap();
        assert_eq!(attempt.weight, Decimal::from(100));
        assert!(attempt.is_successful);
    }

    #[test]
    fn an_x_suffix_is_a_miss() {
        let attempt = parse_attempt("100x").unwrap().unwrap();
        assert_eq!(attempt.weight, Decimal::from(100));
        assert!(!attempt.is_successful);
    }

    #[test]
    fn zero_is_a_lift_and_can_be_missed() {
        let good = parse_attempt("0").unwrap().unwrap();
        assert_eq!(good.weight, Decimal::ZERO);
        assert!(good.is_successful);

        let missed = parse_attempt("0x").unwrap().unwrap();
        assert_eq!(missed.weight, Decimal::ZERO);
        assert!(!missed.is_successful);
    }

    #[test]
    fn an_empty_cell_is_not_an_attempt() {
        assert!(parse_attempt("").unwrap().is_none());
        assert!(parse_attempt("   ").unwrap().is_none());
    }

    #[test]
    fn a_negative_weight_points_at_the_x_suffix() {
        let error = parse_attempt("-100").unwrap_err();
        assert!(error.contains("100x"), "{error}");
    }

    #[test]
    fn rendering_round_trips() {
        for cell in ["100", "100x", "0", "0x", "12.5", "12.5x"] {
            let attempt = parse_attempt(cell).unwrap().unwrap();
            assert_eq!(render_attempt(&attempt), cell);
        }
    }

    #[test]
    fn trailing_zeros_are_stripped() {
        let attempt = Attempt {
            weight: Decimal::from_str("100.00").unwrap(),
            is_successful: true,
        };
        assert_eq!(render_attempt(&attempt), "100");
    }

    #[test]
    fn headers_cover_every_movement() {
        let headers = headers(Layout::default());
        assert_eq!(headers.len(), 10 + 4 * 4);
        assert!(headers.contains(&"MuscleUp1Kg".to_string()));
        assert!(headers.contains(&"BestSquatKg".to_string()));
    }

    #[test]
    fn a_divisioned_meet_leads_with_the_division() {
        let headers = headers(Layout {
            divisioned: true,
            native_names: false,
            ..Layout::default()
        });
        assert_eq!(headers.len(), 11 + 4 * 4);
        assert_eq!(headers[0], DIVISION);
    }

    #[test]
    fn a_classed_meet_names_the_class_after_the_sex() {
        let headers = headers(Layout {
            classed: true,
            ..Layout::default()
        });
        assert_eq!(headers.len(), 11 + 4 * 4);
        assert_eq!(headers[0], SEX);
        assert_eq!(headers[1], WEIGHT_CLASS);
    }

    #[test]
    fn a_meet_without_weight_classes_omits_the_column() {
        assert!(!headers(Layout::default()).contains(&WEIGHT_CLASS.to_string()));
    }

    #[test]
    fn an_unknown_column_is_rejected() {
        let mut header = csv::StringRecord::from(headers(Layout::default()));
        header.push_field("Total");
        let error = Columns::read(&header).unwrap_err();
        assert!(error.contains("Total"), "{error}");
    }

    #[test]
    fn the_old_ris_header_is_rejected_even_alongside_reported_ris() {
        for alongside in [false, true] {
            let mut fields = headers(Layout::default());
            if !alongside {
                fields.retain(|field| field != REPORTED_RIS);
            }
            fields.push("Ris".into());
            let error = Columns::read(&csv::StringRecord::from(fields)).unwrap_err();
            assert!(error.contains("Ris is no longer supported"), "{error}");
            assert!(error.contains("ReportedRis"), "{error}");
        }
    }

    #[test]
    fn legacy_total_headers_are_rejected_with_or_without_the_new_column() {
        for alongside in [false, true] {
            let mut fields = headers(Layout::default());
            if !alongside {
                fields.retain(|field| field != TOTAL);
            }
            fields.push("ReportedTotalKg".into());
            assert!(Columns::read(&csv::StringRecord::from(fields)).is_err());
        }
    }

    #[test]
    fn missing_required_columns_are_rejected() {
        for column in [
            SEX,
            LAST_NAME,
            COUNTRY,
            BODYWEIGHT,
            REPORTED_RIS,
            TOTAL,
            STATUS,
            "BestSquatKg",
        ] {
            let mut fields = headers(Layout::default());
            fields.retain(|c| c != column);
            let error = Columns::read(&csv::StringRecord::from(fields)).unwrap_err();
            assert!(error.contains(column), "{error}");
        }
    }

    #[test]
    fn misspelled_optional_columns_are_rejected() {
        for (column, typo) in [
            (FIRST_NAME, "Firstname"),
            (DISAMBIGUATION, "Disambiguaton"),
            (STATUS_REASON, "StatusReasn"),
        ] {
            let mut fields = headers(Layout::default());
            *fields.iter_mut().find(|field| *field == column).unwrap() = typo.to_string();
            let error = Columns::read(&csv::StringRecord::from(fields)).unwrap_err();
            assert!(error.contains("unknown column"), "{error}");
            assert!(error.contains(typo), "{error}");
        }
    }

    #[test]
    fn a_file_without_a_division_column_is_accepted() {
        for layout in [
            Layout::default(),
            Layout {
                divisioned: true,
                native_names: false,
                ..Layout::default()
            },
            Layout {
                divisioned: false,
                native_names: true,
                ..Layout::default()
            },
            Layout {
                divisioned: true,
                native_names: true,
                ..Layout::default()
            },
        ] {
            assert!(Columns::read(&csv::StringRecord::from(headers(layout))).is_ok());
        }
    }
}
