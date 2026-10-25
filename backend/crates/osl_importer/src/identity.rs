use osl_domain::normalized_name::NormalizedAthleteName;

use crate::canonical::models::AthleteData;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AthleteQuery {
    pub match_key: String,
    pub gender: Option<String>,
    pub country: Option<String>,
    pub disambiguation: Option<i16>,
}

impl AthleteQuery {
    #[must_use]
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

    #[must_use]
    pub fn matches_parts(&self, gender: &str, country: &str, disambiguation: Option<i16>) -> bool {
        self.gender.as_deref().is_none_or(|g| g == gender)
            && self.country.as_deref().is_none_or(|c| c == country)
            && self
                .disambiguation
                .is_none_or(|number| Some(number) == disambiguation)
    }

    #[must_use]
    pub fn matches_entry(&self, athlete: &AthleteData) -> bool {
        if match_key(&athlete.display_name()) != self.match_key {
            return false;
        }

        self.matches_parts(
            athlete.gender.as_str(),
            athlete
                .country
                .as_ref()
                .map_or("", osl_domain::CountryCode::as_str),
            athlete.disambiguation,
        )
    }

    #[must_use]
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
    fn normalizes_full_names() {
        assert_eq!(match_key("Léa MERANDON"), "lea merandon");
        assert_eq!(
            match_key("Adrien Pelfresne"),
            NormalizedAthleteName::new("Adrien", "Pelfresne").match_name()
        );
    }

    #[test]
    fn preserves_middle_names_in_keys() {
        assert_eq!(match_key("Jean Luc Picard"), match_key("Jean-Luc Picard"));
    }

    #[test]
    fn distinguishes_different_athletes() {
        assert_ne!(match_key("Tom Berthier"), match_key("Tom Bertier"));
    }

    #[test]
    fn matches_any_identity_when_filters_are_empty() {
        let query = AthleteQuery::new("Tony Nguyen", None, None, None);

        assert!(query.matches_parts("M", "FR", None));
        assert!(query.matches_parts("F", "US", Some(2)));
    }

    #[test]
    fn applies_identity_filters() {
        let query = AthleteQuery::new("Tony Nguyen", None, Some("FR".into()), None);

        assert!(query.matches_parts("M", "FR", None));
        assert!(!query.matches_parts("M", "US", None));

        let second = AthleteQuery::new("Tony Nguyen", None, None, Some(2));

        assert!(second.matches_parts("M", "FR", Some(2)));
        assert!(!second.matches_parts("M", "FR", None));
        assert!(!second.matches_parts("M", "FR", Some(1)));
    }

    #[test]
    fn combines_identity_filters() {
        let one = AthleteQuery::new("Tony Nguyen", Some("F".into()), Some("IT".into()), None);

        assert!(one.matches_parts("F", "IT", None));
        assert!(!one.matches_parts("M", "IT", None));
        assert!(!one.matches_parts("F", "SM", None));
    }
}
