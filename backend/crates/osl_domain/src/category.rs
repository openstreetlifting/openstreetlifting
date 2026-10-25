use rust_decimal::Decimal;

use crate::{Gender, WeightClass};

#[must_use]
pub fn category_label(
    division: Option<&str>,
    gender: Gender,
    min: Option<Decimal>,
    max: Option<Decimal>,
) -> String {
    let who = match gender {
        Gender::M => "Men",
        Gender::F => "Women",
        Gender::Mx => "Mixed",
    };

    let class = WeightClass::label(min, max);

    [division.unwrap_or_default(), who, class.as_str()]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefixes_labels_with_division() {
        assert_eq!(
            category_label(
                Some("Elite"),
                Gender::M,
                Some(Decimal::from(73)),
                Some(Decimal::from(80))
            ),
            "Elite Men -80kg"
        );
    }

    #[test]
    fn labels_gender_and_class_when_division_is_absent() {
        assert_eq!(
            category_label(None, Gender::F, Some(Decimal::from(70)), None),
            "Women +70kg"
        );
    }

    #[test]
    fn labels_gender_when_class_is_absent() {
        assert_eq!(category_label(None, Gender::M, None, None), "Men");
    }

    #[test]
    fn preserves_division_when_class_is_absent() {
        assert_eq!(
            category_label(Some("Elite"), Gender::M, None, None),
            "Elite Men"
        );
    }
}
