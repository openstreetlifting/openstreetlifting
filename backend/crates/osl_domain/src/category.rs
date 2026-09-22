use rust_decimal::Decimal;

use crate::{Gender, WeightClass};

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
    fn a_division_leads_the_name() {
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
    fn a_meet_without_divisions_names_gender_and_class_alone() {
        assert_eq!(
            category_label(None, Gender::F, Some(Decimal::from(70)), None),
            "Women +70kg"
        );
    }

    #[test]
    fn a_contest_without_a_weight_class_is_named_by_gender_alone() {
        assert_eq!(category_label(None, Gender::M, None, None), "Men");
    }

    #[test]
    fn a_division_survives_a_missing_weight_class() {
        assert_eq!(
            category_label(Some("Elite"), Gender::M, None, None),
            "Elite Men"
        );
    }
}
