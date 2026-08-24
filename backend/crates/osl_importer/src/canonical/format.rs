use osl_domain::Gender;
use rust_decimal::Decimal;

use super::models::{CanonicalFormat, CategoryData};

pub fn normalize(canonical: &mut CanonicalFormat) {
    canonical.movements.sort_by_key(|m| m.display_order());
    canonical.movements.dedup();
    canonical
        .categories
        .sort_by(|a, b| category_order(a).cmp(&category_order(b)));

    for category in &mut canonical.categories {
        category.athletes.sort_by(|a, b| {
            a.last_name
                .cmp(&b.last_name)
                .then_with(|| a.first_name.cmp(&b.first_name))
                .then_with(|| a.disambiguation.cmp(&b.disambiguation))
        });

        for athlete in &mut category.athletes {
            athlete.lifts.sort_by_key(|l| l.movement.display_order());

            for lift in &mut athlete.lifts {
                if let Some(attempts) = lift.attempts.as_mut() {
                    attempts.sort_by_key(|a| a.attempt_number);
                }
            }
        }
    }
}

fn category_order(category: &CategoryData) -> (Option<&str>, u8, Decimal, Decimal) {
    let gender = match category.gender {
        Gender::M => 0,
        Gender::F => 1,
        Gender::Mx => 2,
    };

    let (min, max) = category.bounds();

    (
        category.division.as_deref(),
        gender,
        max.unwrap_or(Decimal::MAX),
        min.unwrap_or(Decimal::ZERO),
    )
}
