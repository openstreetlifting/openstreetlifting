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

    match division {
        Some(division) => format!("{division} {who} {class}"),
        None => format!("{who} {class}"),
    }
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
}
