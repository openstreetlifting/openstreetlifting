//! An athlete is the folded name together with gender, country and
//! disambiguation, so a name on its own is a coarser key than an athlete.

use osl_domain::normalized_name::NormalizedAthleteName;

use crate::canonical::models::{AthleteData, CategoryData};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AthleteQuery {
    pub match_key: String,
    pub gender: Option<String>,
    pub country: Option<String>,
    pub disambiguation: Option<i16>,
}

impl AthleteQuery {
    pub fn new(
        full_name: &str,
        gender: Option<String>,
        country: Option<String>,
        disambiguation: Option<i16>,
    ) -> Self {
        Self {
            match_key: match_key(full_name),
            gender,
            country,
            disambiguation,
        }
    }

    /// A `None` reads as "any", so a unique name needs nothing filled in.
    pub fn matches_parts(&self, gender: &str, country: &str, disambiguation: Option<i16>) -> bool {
        self.gender.as_deref().is_none_or(|g| g == gender)
            && self.country.as_deref().is_none_or(|c| c == country)
            && self
                .disambiguation
                .is_none_or(|number| Some(number) == disambiguation)
    }

    pub fn matches_entry(&self, athlete: &AthleteData, category: &CategoryData) -> bool {
        if match_key(&athlete.display_name()) != self.match_key {
            return false;
        }

        let gender = athlete.gender.unwrap_or(category.gender);

        self.matches_parts(
            gender.as_str(),
            athlete.country.as_str(),
            athlete.disambiguation,
        )
    }

    pub fn narrowing(&self) -> String {
        let mut columns = Vec::new();

        if let Some(gender) = &self.gender {
            columns.push(format!("Sex '{gender}'"));
        }
        if let Some(country) = &self.country {
            columns.push(format!("Country '{country}'"));
        }
        if let Some(number) = self.disambiguation {
            columns.push(format!("Disambiguation '{number}'"));
        }

        columns.join(", ")
    }
}

pub fn match_key(full_name: &str) -> String {
    let mut parts = full_name.trim().splitn(2, char::is_whitespace);
    let first = parts.next().unwrap_or_default();
    let last = parts.next().unwrap_or_default();
    NormalizedAthleteName::new(first, last).match_name()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_whole_name_folds_the_way_the_two_halves_do() {
        assert_eq!(match_key("Léa MERANDON"), "lea merandon");
        assert_eq!(
            match_key("Adrien Pelfresne"),
            NormalizedAthleteName::new("Adrien", "Pelfresne").match_name()
        );
    }

    #[test]
    fn a_middle_name_stays_part_of_the_key() {
        assert_eq!(match_key("Jean Luc Picard"), match_key("Jean-Luc Picard"));
    }

    #[test]
    fn different_people_do_not_collide() {
        assert_ne!(match_key("Tom Berthier"), match_key("Tom Bertier"));
    }

    #[test]
    fn a_blank_column_matches_anything() {
        let query = AthleteQuery::new("Tony Nguyen", None, None, None);

        assert!(query.matches_parts("M", "FR", None));
        assert!(query.matches_parts("F", "US", Some(2)));
    }

    #[test]
    fn a_filled_column_narrows() {
        let query = AthleteQuery::new("Tony Nguyen", None, Some("FR".into()), None);

        assert!(query.matches_parts("M", "FR", None));
        assert!(!query.matches_parts("M", "US", None));

        let second = AthleteQuery::new("Tony Nguyen", None, None, Some(2));

        assert!(second.matches_parts("M", "FR", Some(2)));
        assert!(!second.matches_parts("M", "FR", None));
        assert!(!second.matches_parts("M", "FR", Some(1)));
    }

    #[test]
    fn the_columns_narrow_together() {
        let one = AthleteQuery::new("Tony Nguyen", Some("F".into()), Some("IT".into()), None);

        assert!(one.matches_parts("F", "IT", None));
        assert!(!one.matches_parts("M", "IT", None));
        assert!(!one.matches_parts("F", "SM", None));
    }
}
